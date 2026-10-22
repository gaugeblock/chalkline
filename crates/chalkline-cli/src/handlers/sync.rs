//! Handler for `chalkline sync`.

use super::compile::resolve_spec_file;
use crate::args::SyncArgs;
use anyhow::{Context, Result};
use chalkline_core::SpecParser;
use std::path::Path;

/// Executes the sync subcommand.
pub fn handle_sync(root_dir: &Path, args: SyncArgs, json: bool) -> Result<()> {
    let spec_path = resolve_spec_file(root_dir, args.spec)?;
    let parser = SpecParser::new();
    let doc = parser
        .parse_file(&spec_path)
        .with_context(|| format!("Failed to parse {}", spec_path.display()))?;

    // Render updated markdown back to disk
    let rendered = parser.render_markdown(&doc);
    std::fs::write(&spec_path, rendered)
        .with_context(|| format!("Failed to write updated spec: {}", spec_path.display()))?;

    if json {
        let out = serde_json::json!({
            "status": "synced",
            "spec_file": spec_path,
            "tasks_count": doc.tasks.len(),
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("=== Synced Specification ===");
        println!("Spec File:    {}", spec_path.display());
        println!("Tasks:        {}", doc.tasks.len());
        println!("Status:       Up to date");
    }

    Ok(())
}

