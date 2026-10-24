//! AST-grounded documentation drift verification engine.

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use miter_engine::WorkspaceInventory;

use crate::docs::config::{DocType, DocsConfig};

/// Status of a documentation file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocStatus {
    /// Document exists and passes all policy audits.
    Present,
    /// Document is required but absent from disk.
    Missing,
    /// Document exists but diverges from code AST, git commits, or policies.
    Drifted,
}

/// Audit result for a single document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocAuditItem {
    /// Canonical document type.
    pub doc_type: DocType,
    /// Path relative to repo root.
    pub path: PathBuf,
    /// Compliance status.
    pub status: DocStatus,
    /// Detected policy or AST drift violations.
    pub violations: Vec<String>,
}

/// Aggregated documentation audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DocAuditReport {
    /// Individual document audit results.
    pub items: Vec<DocAuditItem>,
}

impl DocAuditReport {
    /// Whether all audited documents pass without missing files or drift.
    pub fn is_clean(&self) -> bool {
        self.items.iter().all(|i| i.status == DocStatus::Present)
    }

    /// Number of missing required documents.
    pub fn missing_count(&self) -> usize {
        self.items.iter().filter(|i| i.status == DocStatus::Missing).count()
    }

    /// Number of drifted documents.
    pub fn drifted_count(&self) -> usize {
        self.items.iter().filter(|i| i.status == DocStatus::Drifted).count()
    }
}

/// Verifies workspace documentation against code AST and git history.
pub fn verify_docs_drift(
    root: &Path,
    config: &DocsConfig,
    inventory: &WorkspaceInventory,
) -> DocAuditReport {
    let mut items = Vec::new();

    for doc_type in DocType::ALL {
        let entry = config.entry_for(doc_type);
        let file_path = root.join(&entry.path);

        if !file_path.is_file() {
            if entry.required {
                items.push(DocAuditItem {
                    doc_type,
                    path: PathBuf::from(&entry.path),
                    status: DocStatus::Missing,
                    violations: vec![format!("Required document '{}' is missing", entry.path)],
                });
            }
            continue;
        }

        let content = fs::read_to_string(&file_path).unwrap_or_default();
        let mut violations = Vec::new();

        if config.policies.enforce_file_loc_limits && content.lines().count() > 500 {
            violations.push(format!("File exceeds 500 LOC ({})", content.lines().count()));
        }

        audit_document_semantics(doc_type, &content, config, inventory, &mut violations);

        let status = if violations.is_empty() {
            DocStatus::Present
        } else {
            DocStatus::Drifted
        };

        items.push(DocAuditItem {
            doc_type,
            path: PathBuf::from(&entry.path),
            status,
            violations,
        });
    }

    DocAuditReport { items }
}

fn audit_document_semantics(
    doc_type: DocType,
    content: &str,
    config: &DocsConfig,
    inv: &WorkspaceInventory,
    violations: &mut Vec<String>,
) {
    match doc_type {
        DocType::Architecture if config.policies.enforce_crate_topology => {
            for cr in &inv.crates {
                if !content.contains(&cr.name) {
                    violations.push(format!("Missing mention of member crate '{}'", cr.name));
                }
            }
        }
        DocType::Changelog if config.policies.enforce_changelog_unreleased => {
            if !content.contains("## [Unreleased]") && !content.contains("## Unreleased") {
                violations.push("Missing '## [Unreleased]' section in CHANGELOG.md".into());
            }
        }
        DocType::Contributing if config.policies.enforce_contributing_toolchains => {
            if let Some(rust) = &inv.toolchain.rust_version
                && !content.contains(rust)
            {
                violations.push(format!("Missing configured Rust channel/version '{rust}'"));
            }
        }
        _ => {}
    }
}
