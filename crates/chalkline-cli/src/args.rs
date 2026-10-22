//! CLI command line argument definitions for `chalkline`.

#![forbid(unsafe_code)]

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// CLI arguments for `chalkline`.
#[derive(Debug, Parser)]
#[command(
    name = "chalkline",
    version,
    about = "Interactive spec planner, ambiguity griller, and topological task DAG compiler"
)]
pub struct Cli {
    /// Target repository directory (defaults to current directory).
    #[arg(short, long, global = true)]
    pub dir: Option<PathBuf>,

    /// Emit machine-readable JSON output instead of human-readable text.
    #[arg(long, global = true)]
    pub json: bool,

    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for `chalkline`.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Conduct interactive /grill-me session on a prompt or spec.
    Grill(GrillArgs),

    /// Compile tasks from a specification file into a storypole task DAG.
    CompileDag(CompileArgs),

    /// Sync task completion checkboxes between storypole and specification markdown.
    Sync(SyncArgs),

    /// Run codebase reconnaissance with miter-engine to inspect detected conventions.
    Recon(ReconArgs),

    /// Launch the stdio JSON-RPC 2.0 Model Context Protocol (MCP) server.
    Mcp,
}

/// Arguments for `chalkline grill`.
#[derive(Debug, Args)]
pub struct GrillArgs {
    /// Feature prompt or specification goal.
    pub prompt: Option<String>,

    /// Path to specification file (defaults to auto-detected SPEC.md or PRODUCT.md).
    #[arg(short, long)]
    pub spec: Option<PathBuf>,

    /// Non-interactive mode (auto-resolves or selects first option).
    #[arg(long)]
    pub non_interactive: bool,

    /// Automatically compile task DAG into storypole upon completing the grill session.
    #[arg(long)]
    pub compile: bool,
}

/// Arguments for `chalkline compile-dag`.
#[derive(Debug, Args)]
pub struct CompileArgs {
    /// Path to specification file (defaults to auto-detected SPEC.md or PRODUCT.md).
    #[arg(short, long)]
    pub spec: Option<PathBuf>,
}

/// Arguments for `chalkline sync`.
#[derive(Debug, Args)]
pub struct SyncArgs {
    /// Path to specification file (defaults to auto-detected SPEC.md or PRODUCT.md).
    #[arg(short, long)]
    pub spec: Option<PathBuf>,
}

/// Arguments for `chalkline recon`.
#[derive(Debug, Args)]
pub struct ReconArgs {
    /// Path to target repository (defaults to current directory).
    pub path: Option<PathBuf>,
}
