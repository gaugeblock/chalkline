//! Handler for `chalkline compile-dag`.

use crate::args::CompileArgs;
use anyhow::{Context, Result, bail};
use chalkline_core::{DagCompiler, SpecParser};
use std::path::{Path, PathBuf};

/// Finds the target specification file.
pub fn resolve_spec_file(root_dir: &Path, explicit: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(p) = explicit {
        if p.exists() {
            return Ok(p);
        }
        let joined = root_dir.join(&p);
        if joined.exists() {
            return Ok(joined);
        }
        bail!("Specified spec file not found: {}", p.display());
    }

    let candidates = ["SPEC.md", "PRODUCT.md", "ARCHITECTURE.md"];
    for name in candidates {
        let path = root_dir.join(name);
        if path.exists() {
            return Ok(path);
        }
    }

    bail!(
        "No specification file found in {}. Create SPEC.md first.",
        root_dir.display()
    );
}

/// Executes the compile-dag subcommand.
pub fn handle_compile(root_dir: &Path, args: CompileArgs, json: bool) -> Result<()> {
    let spec_path = resolve_spec_file(root_dir, args.spec)?;
    let parser = SpecParser::new();
    let doc = parser
        .parse_file(&spec_path)
        .with_context(|| format!("Failed to parse {}", spec_path.display()))?;

    let compiler = DagCompiler::new();
    let receipt = compiler
        .compile(root_dir, &doc)
        .with_context(|| "Failed to compile tasks into storypole")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&receipt)?);
    } else {
        println!("=== Compiled Task DAG to Storypole ===");
        println!("Spec File:        {}", spec_path.display());
        println!("Tasks Compiled:   {}", receipt.tasks_compiled);
        println!("Edges Linked:     {}", receipt.edges_linked);
        if !receipt.ticket_keys.is_empty() {
            println!("Tickets Created:  {}", receipt.ticket_keys.join(", "));
        }
    }

    Ok(())
}
