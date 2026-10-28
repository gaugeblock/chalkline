//! AST-grounded documentation drift verification engine.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use miter_engine::WorkspaceInventory;

use crate::docs::config::{DocType, DocsConfig};
use crate::docs::rules;

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
        self.items
            .iter()
            .filter(|i| i.status == DocStatus::Missing)
            .count()
    }

    /// Number of drifted documents.
    pub fn drifted_count(&self) -> usize {
        self.items
            .iter()
            .filter(|i| i.status == DocStatus::Drifted)
            .count()
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
            violations.push(format!(
                "File exceeds 500 LOC ({})",
                content.lines().count()
            ));
        }

        audit_document_semantics(doc_type, &content, &file_path, config, inventory, &mut violations);

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
    file_path: &Path,
    config: &DocsConfig,
    inv: &WorkspaceInventory,
    violations: &mut Vec<String>,
) {
    let doc_dir = file_path.parent().unwrap_or(Path::new("."));
    rules::audit_markdown_links(content, doc_dir, violations);

    match doc_type {
        DocType::Architecture if config.policies.enforce_crate_topology => {
            rules::audit_architecture(content, inv, violations);
        }
        DocType::Spec if config.policies.enforce_public_type_contracts => {
            rules::audit_spec_contracts(content, inv, violations);
        }
        DocType::Readme if config.policies.enforce_readme_cli_flags => {
            rules::audit_readme_cli(content, inv, violations);
        }
        DocType::Agents => {
            rules::audit_agents_directives(content, violations);
        }
        DocType::Changelog if config.policies.enforce_changelog_unreleased => {
            rules::audit_changelog(content, violations);
        }
        DocType::Contributing if config.policies.enforce_contributing_toolchains => {
            rules::audit_contributing(content, inv, violations);
        }
        _ => {}
    }
}
