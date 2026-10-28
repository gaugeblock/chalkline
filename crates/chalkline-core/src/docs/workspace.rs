//! Workspace-wide documentation discovery and aggregated auditing.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::docs::config::DocsConfig;
use crate::docs::drift::{DocAuditReport, DocStatus, verify_docs_drift};
use crate::error::{ChalklineError, Result};
use miter_engine::inspect_workspace;

/// Discovered member repository within a multi-repo workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveredRepo {
    /// Name of repository (e.g. "chalkline", "miter", "workspace").
    pub name: String,
    /// Absolute path to repository root on disk.
    pub path: PathBuf,
    /// Whether this is the meta-harness root repository.
    pub is_root: bool,
}

/// Aggregated multi-repository documentation audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkspaceDocsAuditReport {
    /// Individual repository audit reports keyed by repo name.
    pub targets: Vec<(String, DocAuditReport)>,
}

impl WorkspaceDocsAuditReport {
    /// Whether all audited repositories pass without missing files or drift.
    pub fn is_clean(&self) -> bool {
        self.targets.iter().all(|(_, r)| r.is_clean())
    }

    /// Total number of present canonical documents across the workspace.
    pub fn total_present(&self) -> usize {
        self.targets
            .iter()
            .map(|(_, r)| r.items.iter().filter(|i| i.status == DocStatus::Present).count())
            .sum()
    }

    /// Total number of missing documents across the workspace.
    pub fn total_missing(&self) -> usize {
        self.targets.iter().map(|(_, r)| r.missing_count()).sum()
    }

    /// Total number of drifted documents across the workspace.
    pub fn total_drifted(&self) -> usize {
        self.targets.iter().map(|(_, r)| r.drifted_count()).sum()
    }
}

/// Discover all member repositories and root in a multi-repo workspace.
pub fn discover_workspace_repos(workspace_root: &Path) -> Vec<DiscoveredRepo> {
    let mut repos = Vec::new();

    // Include the workspace root itself as target "workspace"
    repos.push(DiscoveredRepo {
        name: "workspace".to_string(),
        path: workspace_root.to_path_buf(),
        is_root: true,
    });

    let Ok(entries) = fs::read_dir(workspace_root) else {
        return repos;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        // Skip hidden, build, script, and web artifact directories
        if dir_name.starts_with('.')
            || dir_name == "target"
            || dir_name == "scripts"
            || dir_name == "docs"
            || dir_name == "node_modules"
        {
            continue;
        }

        let has_cargo = path.join("Cargo.toml").is_file();
        let has_git = path.join(".git").is_dir() || path.join(".git").is_file();
        let has_docs = path.join("README.md").is_file();

        if (has_cargo || has_git) && has_docs {
            repos.push(DiscoveredRepo {
                name: dir_name.to_string(),
                path,
                is_root: false,
            });
        }
    }

    repos.sort_by(|a, b| {
        // Keep root first, then sort alphabetically
        match (a.is_root, b.is_root) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });

    repos
}

/// Run documentation audit across all discovered workspace repositories.
pub fn verify_workspace_docs_drift(workspace_root: &Path) -> Result<WorkspaceDocsAuditReport> {
    let repos = discover_workspace_repos(workspace_root);
    let mut targets = Vec::new();

    for repo in repos {
        let config = DocsConfig::load_or_default(&repo.path)?;
        let inv = inspect_workspace(&repo.path)
            .map_err(|e| ChalklineError::Validation(format!("AST inspection failed for {}: {e}", repo.name)))?;
        let report = verify_docs_drift(&repo.path, &config, &inv);
        targets.push((repo.name, report));
    }

    Ok(WorkspaceDocsAuditReport { targets })
}

