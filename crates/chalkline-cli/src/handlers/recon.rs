//! Handler for `chalkline recon`.

use crate::args::ReconArgs;
use anyhow::Result;
use chalkline_core::ReconEngine;
use std::path::Path;

/// Executes the recon subcommand.
pub fn handle_recon(root_dir: &Path, args: ReconArgs, json: bool) -> Result<()> {
    let target = args.path.as_deref().unwrap_or(root_dir);
    let recon = ReconEngine::new();
    let report = recon.scan_repo(target)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("=== Codebase Reconnaissance ===");
        println!("Target Root:        {}", report.repo_root.display());
        println!("Primary Language:   {}", report.primary_language);
        if let Some(ed) = &report.rust_edition {
            println!("Rust Edition:       {ed}");
        }
        println!("Error Strategy:     {}", report.error_strategy);
        println!("Async Runtime:      {}", report.async_runtime);
        println!("Serialization:      {}", report.serialization);
        println!("Tracing Framework:  {}", report.tracing_framework);
        if !report.member_crates.is_empty() {
            println!("Member Crates:      {}", report.member_crates.join(", "));
        }
    }

    Ok(())
}

