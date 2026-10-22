//! Command-line interface and entrypoint for `chalkline`.

#![forbid(unsafe_code)]

mod args;
mod handlers;

use anyhow::Result;
use args::{Cli, Commands};
use clap::Parser;
use handlers::*;
use std::env;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let filter = if cli.json { "error" } else { "warn" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(filter)),
        )
        .with_writer(std::io::stderr)
        .init();

    let root_dir = match cli.dir {
        Some(d) => d,
        None => env::current_dir()?,
    };

    match cli.command {
        Commands::Grill(args) => handle_grill(&root_dir, args, cli.json)?,
        Commands::CompileDag(args) => handle_compile(&root_dir, args, cli.json)?,
        Commands::Sync(args) => handle_sync(&root_dir, args, cli.json)?,
        Commands::Recon(args) => handle_recon(&root_dir, args, cli.json)?,
        Commands::Mcp => {
            chalkline_mcp::run_stdio_server(&root_dir)
                .map_err(|e| anyhow::anyhow!("MCP server error: {e}"))?;
        }
    }

    Ok(())
}
