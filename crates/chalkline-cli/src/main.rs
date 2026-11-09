//! Command-line interface and entrypoint for `chalkline`.

#![forbid(unsafe_code)]

mod args;
mod args_docs;
mod handlers;
mod handlers_docs;

use anyhow::Result;
use args::{Cli, Commands};
use clap::Parser;
use handlers::*;
use handlers_docs::*;
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

    // One span per invocation (`GB-339`), opened once logging is configured so it covers every fallible step
    // below. Wrapping only the dispatch let pre-dispatch `?` calls return from `main` with the span still recorded
    // as running - and `--dir`, the argument a user is most likely to mistype, was exactly the path that escaped.
    let command = std::env::args().nth(1).filter(|arg| !arg.starts_with('-'));
    let invocation = gaugeblock_workspace::InvocationSpan::start("chalkline", command.as_deref());

    let result = (|| -> Result<()> {
        let root_dir = chalkline_core::root::resolve_root(cli.dir.as_deref(), &env::current_dir()?)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        match cli.command {
            Commands::Grill(args) => handle_grill(&root_dir, args, cli.json)?,
            Commands::CompileDag(args) => handle_compile(&root_dir, args, cli.json)?,
            Commands::Sync(args) => handle_sync(&root_dir, args, cli.json)?,
            Commands::Recon(args) => handle_recon(&root_dir, args, cli.json)?,
            Commands::Docs(args) => handle_docs(&root_dir, args, cli.json)?,
            Commands::Mcp => {
                chalkline_mcp::run_stdio_server(&root_dir)
                    .map_err(|e| anyhow::anyhow!("MCP server error: {e}"))?;
            }
        }

        Ok(())
    })();

    invocation.finish(result.is_ok());
    result
}
