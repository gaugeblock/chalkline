//! CLI argument definitions for `chalkline docs` commands.

use clap::{Args, Subcommand};
use std::path::PathBuf;

/// Arguments for `chalkline docs`.
#[derive(Debug, Args)]
pub struct DocsArgs {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: DocsCommands,
}

/// Subcommands for `chalkline docs`.
#[derive(Debug, Subcommand)]
pub enum DocsCommands {
    /// Audit repository documentation against .gaugeblock/docs.toml and code AST.
    Check(DocsCheckArgs),
    /// Scaffold missing repository documentation files using AST-grounded templates.
    Scaffold(DocsScaffoldArgs),
    /// Disentangle a brownfield document into the 9 canonical documents.
    Migrate(DocsMigrateArgs),
}

/// Arguments for `chalkline docs check`.
#[derive(Debug, Args)]
pub struct DocsCheckArgs {
    /// Path to target repository (defaults to current directory).
    pub path: Option<PathBuf>,
    /// Audit all member repositories across the workspace.
    #[arg(short = 'w', long, alias = "all")]
    pub workspace: bool,
}

/// Arguments for `chalkline docs scaffold`.
#[derive(Debug, Args)]
pub struct DocsScaffoldArgs {
    /// Only scaffold a specific document type (e.g. `readme`, `agents`, `architecture`).
    #[arg(long)]
    pub doc: Option<String>,
    /// Force overwrite of existing files.
    #[arg(long)]
    pub force: bool,
}

/// Arguments for `chalkline docs migrate`.
#[derive(Debug, Args)]
pub struct DocsMigrateArgs {
    /// Path to brownfield document to disentangle (e.g. `README.md`).
    pub source: PathBuf,
    /// Apply changes directly to disk instead of dry-run preview.
    #[arg(long)]
    pub apply: bool,
}
