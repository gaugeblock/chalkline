//! Handler for `chalkline grill` interactive /grill-me loop.

use super::compile::resolve_spec_file;
use crate::args::GrillArgs;
use anyhow::{Context, Result};
use chalkline_core::{DagCompiler, Griller, ReconEngine, SpecDoc, SpecParser};
use inquire::Select;
use std::path::Path;

/// Executes the interactive /grill-me session.
pub fn handle_grill(root_dir: &Path, args: GrillArgs, json: bool) -> Result<()> {
    let spec_path_opt = args
        .spec
        .clone()
        .or_else(|| resolve_spec_file(root_dir, None).ok());

    let parser = SpecParser::new();
    let mut doc = match &spec_path_opt {
        Some(path) => parser.parse_file(path).unwrap_or_else(|_| SpecDoc {
            path: path.clone(),
            title: "Feature Specification".to_string(),
            overview: String::new(),
            decisions: Vec::new(),
            contracts: Vec::new(),
            tasks: Vec::new(),
        }),
        None => SpecDoc {
            path: root_dir.join("SPEC.md"),
            title: "Feature Specification".to_string(),
            overview: String::new(),
            decisions: Vec::new(),
            contracts: Vec::new(),
            tasks: Vec::new(),
        },
    };

    let prompt = args
        .prompt
        .or_else(|| {
            if !doc.overview.is_empty() {
                Some(doc.overview.clone())
            } else {
                Some(doc.title.clone())
            }
        })
        .unwrap_or_else(|| "New feature implementation".to_string());

    let recon = ReconEngine::new();
    let report = recon.scan_repo(root_dir)?;

    let griller = Griller::new();
    let mut holes = griller.identify_decision_holes(&prompt, Some(&doc))?;
    recon.auto_resolve_decisions(&report, &mut holes);

    if !json {
        println!("=== chalkline /grill-me Studio ===");
        println!("Target Repository: {}", root_dir.display());
        println!("Feature Prompt:    {prompt}");
        println!("Discovered Holes:  {}", holes.len());
        println!();
    }

    for (i, hole) in holes.iter_mut().enumerate() {
        if hole.is_resolved() {
            if !json {
                let opt_name = hole
                    .selected_option
                    .and_then(|idx| hole.options.get(idx))
                    .map(|o| o.label.as_str())
                    .unwrap_or("Unknown");
                println!(
                    "[{}/{}] {} (Auto-resolved via recon: {})",
                    i + 1,
                    hole.category,
                    hole.question,
                    opt_name
                );
            }
            continue;
        }

        if args.non_interactive {
            hole.resolve(0);
            continue;
        }

        let options_display: Vec<String> = hole
            .options
            .iter()
            .map(|opt| format!("{} — {}", opt.label, opt.tradeoff_summary))
            .collect();

        let ans = Select::new(&hole.question, options_display)
            .prompt()
            .with_context(|| "User cancelled decision prompt")?;

        let selected_idx = hole
            .options
            .iter()
            .position(|opt| ans.starts_with(&opt.label))
            .unwrap_or(0);

        hole.resolve(selected_idx);
    }

    doc.decisions.extend(holes.clone());

    // Write updated spec to disk
    let rendered = parser.render_markdown(&doc);
    std::fs::write(&doc.path, rendered)?;

    let compiled_receipt = if args.compile {
        let compiler = DagCompiler::new();
        Some(compiler.compile(root_dir, &doc)?)
    } else {
        None
    };

    if json {
        let out = serde_json::json!({
            "status": "completed",
            "spec_file": doc.path,
            "resolved_holes": holes,
            "compiled_dag": compiled_receipt,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!();
        println!("All ambiguities resolved! Updated {}", doc.path.display());
        if let Some(receipt) = compiled_receipt {
            println!(
                "Successfully compiled {} tasks into storypole DAG ({} edges linked).",
                receipt.tasks_compiled, receipt.edges_linked
            );
        }
    }

    Ok(())
}
