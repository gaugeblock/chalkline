//! 9-Document repository architecture manager and AST-grounded documentation engine.
//!
//! Exposes:
//! - [`DocsConfig`]: Configuration mapped from `.gaugeblock/docs.toml`.
//! - [`DocType`]: Canonical 9 document taxonomy.
//! - [`render_template`]: AST-grounded template scaffolding.
//! - [`verify_docs_drift`]: Static audit against code AST, git commits, and toolchain.
//! - [`DocMigrationPlan`]: Brownfield document section disentangler and migration executor.

pub mod config;
pub mod drift;
pub mod migration;
pub mod templates;
pub mod workspace;

#[cfg(test)]
mod tests;

pub use config::{DocEntryConfig, DocType, DocsConfig, DocsPoliciesConfig, DocumentEntries};
pub use drift::{DocAuditItem, DocAuditReport, DocStatus, verify_docs_drift};
pub use migration::{DocMigrationPlan, MarkdownSection};
pub use miter_engine::inspect_workspace;
pub use templates::render_template;
pub use workspace::{
    DiscoveredRepo, WorkspaceDocsAuditReport, discover_workspace_repos, verify_workspace_docs_drift,
};
