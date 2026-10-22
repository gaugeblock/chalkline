//! Codebase reconnaissance engine that auto-resolves conventions without bothering humans.

#![forbid(unsafe_code)]

use crate::error::Result;
use crate::model::DecisionHole;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Detected architectural conventions and patterns in a target repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconReport {
    /// Root path of the analyzed repository.
    pub repo_root: PathBuf,
    /// Detected language.
    pub primary_language: String,
    /// Rust edition if applicable.
    pub rust_edition: Option<String>,
    /// Error handling crate detected (`thiserror`, `anyhow`, etc.).
    pub error_strategy: String,
    /// Async runtime detected (`tokio`, `async-std`, or `sync`).
    pub async_runtime: String,
    /// Serialization engine (`serde`, `none`).
    pub serialization: String,
    /// Tracing and telemetry framework (`tracing`, `log`, `none`).
    pub tracing_framework: String,
    /// List of workspace crate names.
    pub member_crates: Vec<String>,
}

/// Codebase reconnaissance engine.
#[derive(Debug, Default)]
pub struct ReconEngine;

impl ReconEngine {
    /// Creates a new `ReconEngine`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Analyzes the codebase at `path` and builds a `ReconReport`.
    pub fn scan_repo(&self, path: impl AsRef<Path>) -> Result<ReconReport> {
        let root = path.as_ref().to_path_buf();
        let cargo_toml = root.join("Cargo.toml");

        let mut rust_edition = None;
        let mut error_strategy = "unknown".to_string();
        let mut async_runtime = "sync".to_string();
        let mut serialization = "none".to_string();
        let mut tracing_framework = "none".to_string();
        let mut member_crates = Vec::new();

        if cargo_toml.exists() {
            let manifest_content = std::fs::read_to_string(&cargo_toml)?;
            if manifest_content.contains("edition = \"2024\"")
                || manifest_content.contains("edition.workspace = true")
            {
                rust_edition = Some("2024".to_string());
            } else if manifest_content.contains("edition = \"2021\"") {
                rust_edition = Some("2021".to_string());
            }

            if manifest_content.contains("thiserror") {
                error_strategy = "thiserror".to_string();
            } else if manifest_content.contains("anyhow") {
                error_strategy = "anyhow".to_string();
            }

            if manifest_content.contains("tokio") {
                async_runtime = "tokio".to_string();
            }

            if manifest_content.contains("serde") {
                serialization = "serde".to_string();
            }

            if manifest_content.contains("tracing") {
                tracing_framework = "tracing".to_string();
            }

            // Detect crates/* subdirectories
            let crates_dir = root.join("crates");
            if crates_dir.is_dir() {
                let entries = std::fs::read_dir(&crates_dir)
                    .into_iter()
                    .flatten()
                    .flatten();
                let crates = entries
                    .filter(|e| e.path().is_dir())
                    .filter_map(|e| e.file_name().into_string().ok());
                member_crates.extend(crates);
            }
        }

        Ok(ReconReport {
            repo_root: root,
            primary_language: "rust".to_string(),
            rust_edition,
            error_strategy,
            async_runtime,
            serialization,
            tracing_framework,
            member_crates,
        })
    }

    /// Auto-resolves decision holes that match discovered conventions.
    pub fn auto_resolve_decisions(&self, report: &ReconReport, holes: &mut [DecisionHole]) {
        for hole in holes.iter_mut() {
            if hole.selected_option.is_some() {
                continue;
            }

            match hole.category.as_str() {
                "error_strategy" => {
                    for (idx, opt) in hole.options.iter().enumerate() {
                        if opt.label.to_lowercase().contains(&report.error_strategy) {
                            hole.auto_resolved_by_miter = Some(idx);
                            hole.selected_option = Some(idx);
                            break;
                        }
                    }
                }
                "async_runtime" => {
                    for (idx, opt) in hole.options.iter().enumerate() {
                        if opt.label.to_lowercase().contains(&report.async_runtime) {
                            hole.auto_resolved_by_miter = Some(idx);
                            hole.selected_option = Some(idx);
                            break;
                        }
                    }
                }
                "serialization" => {
                    for (idx, opt) in hole.options.iter().enumerate() {
                        if opt.label.to_lowercase().contains(&report.serialization) {
                            hole.auto_resolved_by_miter = Some(idx);
                            hole.selected_option = Some(idx);
                            break;
                        }
                    }
                }
                "tracing" => {
                    for (idx, opt) in hole.options.iter().enumerate() {
                        if opt.label.to_lowercase().contains(&report.tracing_framework) {
                            hole.auto_resolved_by_miter = Some(idx);
                            hole.selected_option = Some(idx);
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
