//! Configuration for the 9-Document repository architecture (`.gaugeblock/docs.toml`).

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{ChalklineError, Result};

/// Canonical documentation types managed by gaugeblock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocType {
    /// Elevator pitch, quickstart, badges.
    Readme,
    /// Human contributor onboarding and etiquette.
    Contributing,
    /// User needs, personas, jobs to be done.
    Product,
    /// Subsystem topologies, crate boundaries.
    Architecture,
    /// Behavioral invariants, wire schemas, ADT contracts.
    Spec,
    /// Architectural Decision Records (ADRs) and trade-offs.
    Design,
    /// Horizon, milestone epics, upcoming features.
    Roadmap,
    /// Historical released changelog (Keep a Changelog format).
    Changelog,
    /// Operational directives and laws for autonomous LLM agents.
    Agents,
}

impl DocType {
    /// Canonical default filename.
    pub fn default_filename(&self) -> &'static str {
        match self {
            DocType::Readme => "README.md",
            DocType::Contributing => "CONTRIBUTING.md",
            DocType::Product => "PRODUCT.md",
            DocType::Architecture => "ARCHITECTURE.md",
            DocType::Spec => "SPEC.md",
            DocType::Design => "DESIGN.md",
            DocType::Roadmap => "ROADMAP.md",
            DocType::Changelog => "CHANGELOG.md",
            DocType::Agents => "AGENTS.md",
        }
    }

    /// All 9 canonical types in order.
    pub const ALL: [DocType; 9] = [
        DocType::Readme,
        DocType::Contributing,
        DocType::Product,
        DocType::Architecture,
        DocType::Spec,
        DocType::Design,
        DocType::Roadmap,
        DocType::Changelog,
        DocType::Agents,
    ];
}

/// Individual document configuration entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocEntryConfig {
    /// Relative path from repository root.
    pub path: String,
    /// Whether this document is required or optional.
    #[serde(default = "default_true")]
    pub required: bool,
}

fn default_true() -> bool {
    true
}

/// Verification policies for documentation auditing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPoliciesConfig {
    /// Enforce that all public structs and functions in Rust/TS are documented.
    #[serde(default = "default_true")]
    pub enforce_public_type_contracts: bool,
    /// Enforce that ARCHITECTURE.md contains all Cargo workspace members.
    #[serde(default = "default_true")]
    pub enforce_crate_topology: bool,
    /// Enforce that no doc file exceeds 500 lines without splitting.
    #[serde(default = "default_true")]
    pub enforce_file_loc_limits: bool,
    /// Audit README.md CLI flags against Clap CLI definitions.
    #[serde(default = "default_true")]
    pub enforce_readme_cli_flags: bool,
    /// Audit CHANGELOG.md Unreleased section against Git commits since last tag.
    #[serde(default = "default_true")]
    pub enforce_changelog_unreleased: bool,
    /// Audit CONTRIBUTING.md against rust-toolchain.toml and package.json engines.
    #[serde(default = "default_true")]
    pub enforce_contributing_toolchains: bool,
}

impl Default for DocsPoliciesConfig {
    fn default() -> Self {
        Self {
            enforce_public_type_contracts: true,
            enforce_crate_topology: true,
            enforce_file_loc_limits: true,
            enforce_readme_cli_flags: true,
            enforce_changelog_unreleased: true,
            enforce_contributing_toolchains: true,
        }
    }
}

/// Master configuration mapped from `.gaugeblock/docs.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsConfig {
    /// Mapped document files.
    pub docs: DocumentEntries,
    /// Verification and compliance policies.
    #[serde(default)]
    pub policies: DocsPoliciesConfig,
}

/// Mapped document entries per canonical document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentEntries {
    /// README.md
    pub readme: DocEntryConfig,
    /// CONTRIBUTING.md
    pub contributing: DocEntryConfig,
    /// PRODUCT.md
    pub product: DocEntryConfig,
    /// ARCHITECTURE.md
    pub architecture: DocEntryConfig,
    /// SPEC.md
    pub spec: DocEntryConfig,
    /// DESIGN.md
    pub design: DocEntryConfig,
    /// ROADMAP.md
    pub roadmap: DocEntryConfig,
    /// CHANGELOG.md
    pub changelog: DocEntryConfig,
    /// AGENTS.md
    pub agents: DocEntryConfig,
}

impl Default for DocsConfig {
    fn default() -> Self {
        Self {
            docs: DocumentEntries {
                readme: DocEntryConfig {
                    path: "README.md".into(),
                    required: true,
                },
                contributing: DocEntryConfig {
                    path: "CONTRIBUTING.md".into(),
                    required: true,
                },
                product: DocEntryConfig {
                    path: "PRODUCT.md".into(),
                    required: false,
                },
                architecture: DocEntryConfig {
                    path: "ARCHITECTURE.md".into(),
                    required: true,
                },
                spec: DocEntryConfig {
                    path: "SPEC.md".into(),
                    required: true,
                },
                design: DocEntryConfig {
                    path: "DESIGN.md".into(),
                    required: false,
                },
                roadmap: DocEntryConfig {
                    path: "ROADMAP.md".into(),
                    required: false,
                },
                changelog: DocEntryConfig {
                    path: "CHANGELOG.md".into(),
                    required: true,
                },
                agents: DocEntryConfig {
                    path: "AGENTS.md".into(),
                    required: true,
                },
            },
            policies: DocsPoliciesConfig::default(),
        }
    }
}

impl DocsConfig {
    /// Returns the configuration entry for a given document type.
    pub fn entry_for(&self, doc_type: DocType) -> &DocEntryConfig {
        match doc_type {
            DocType::Readme => &self.docs.readme,
            DocType::Contributing => &self.docs.contributing,
            DocType::Product => &self.docs.product,
            DocType::Architecture => &self.docs.architecture,
            DocType::Spec => &self.docs.spec,
            DocType::Design => &self.docs.design,
            DocType::Roadmap => &self.docs.roadmap,
            DocType::Changelog => &self.docs.changelog,
            DocType::Agents => &self.docs.agents,
        }
    }

    /// Loads `.gaugeblock/docs.toml` from a repository root, or returns defaults if absent.
    pub fn load_or_default(root: &Path) -> Result<Self> {
        let path = root.join(".gaugeblock").join("docs.toml");
        if !path.is_file() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path)?;
        let config: Self = toml::from_str(&content)
            .map_err(|e| ChalklineError::Validation(format!("Invalid docs.toml: {e}")))?;
        Ok(config)
    }

    /// Saves the configuration to `.gaugeblock/docs.toml`.
    pub fn save(&self, root: &Path) -> Result<PathBuf> {
        let dir = root.join(".gaugeblock");
        fs::create_dir_all(&dir)?;
        let path = dir.join("docs.toml");
        let content = toml::to_string_pretty(self).map_err(|e| {
            ChalklineError::Validation(format!("Failed to serialize docs.toml: {e}"))
        })?;
        fs::write(&path, content)?;
        Ok(path)
    }
}
