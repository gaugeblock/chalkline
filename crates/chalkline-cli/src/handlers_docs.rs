//! Handlers for `chalkline docs` commands.

use std::fs;
use std::path::Path;
use anyhow::{Result, anyhow};
use chalkline_core::docs::{
    DocMigrationPlan, DocStatus, DocType, DocsConfig, inspect_workspace, render_template,
    verify_docs_drift,
};

use crate::args_docs::{DocsArgs, DocsCommands};

/// Dispatches `chalkline docs` subcommands.
pub fn handle_docs(root_dir: &Path, args: DocsArgs, json: bool) -> Result<()> {
    match args.command {
        DocsCommands::Check(check_args) => {
            let target = check_args.path.as_deref().unwrap_or(root_dir);
            let config = DocsConfig::load_or_default(target)?;
            let inv = inspect_workspace(target)
                .map_err(|e| anyhow!("Failed to inspect workspace: {e}"))?;
            let report = verify_docs_drift(target, &config, &inv);

            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("\n=== Gaugeblock 9-Document Architecture Audit ===");
                println!("Repository: {}\n", target.display());
                for item in &report.items {
                    let badge = match item.status {
                        DocStatus::Present => "[OK]     ",
                        DocStatus::Missing => "[MISSING]",
                        DocStatus::Drifted => "[DRIFT]  ",
                    };
                    println!("{} {:<16} ({})", badge, format!("{:?}", item.doc_type), item.path.display());
                    for v in &item.violations {
                        println!("           * {v}");
                    }
                }
                println!("\nSummary: {} present, {} missing, {} drifted\n",
                    report.items.iter().filter(|i| i.status == DocStatus::Present).count(),
                    report.missing_count(),
                    report.drifted_count(),
                );
            }
        }
        DocsCommands::Scaffold(scaffold_args) => {
            let config = DocsConfig::load_or_default(root_dir)?;
            let inv = inspect_workspace(root_dir)
                .map_err(|e| anyhow!("Failed to inspect workspace: {e}"))?;

            let mut created = Vec::new();
            for doc_type in DocType::ALL {
                let entry = config.entry_for(doc_type);
                let path = root_dir.join(&entry.path);

                if let Some(filter) = &scaffold_args.doc {
                    let name = format!("{:?}", doc_type).to_lowercase();
                    if !name.contains(&filter.to_lowercase()) {
                        continue;
                    }
                }

                if path.is_file() && !scaffold_args.force {
                    continue;
                }

                let content = render_template(doc_type, &inv);
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&path, content)?;
                created.push(entry.path.clone());
            }

            if json {
                println!("{}", serde_json::to_string_pretty(&created)?);
            } else {
                println!("Scaffolded {} documentation files:", created.len());
                for c in &created {
                    println!("  + {c}");
                }
            }
        }
        DocsCommands::Migrate(migrate_args) => {
            let source_path = if migrate_args.source.is_absolute() {
                migrate_args.source
            } else {
                root_dir.join(migrate_args.source)
            };
            let config = DocsConfig::load_or_default(root_dir)?;
            let plan = DocMigrationPlan::plan_from_file(&source_path, &config)
                .map_err(|e| anyhow!("Failed to generate migration plan: {e}"))?;

            if migrate_args.apply {
                let written = plan.execute(root_dir, &config)
                    .map_err(|e| anyhow!("Failed to execute migration: {e}"))?;
                if json {
                    println!("{}", serde_json::to_string_pretty(&written)?);
                } else {
                    println!("Applied migration from {}:", source_path.display());
                    for p in written {
                        println!("  + {}", p.display());
                    }
                }
            } else if json {
                println!("{}", serde_json::to_string_pretty(&plan)?);
            } else {
                println!("\n=== Proposed Migration Plan for {} ===", source_path.display());
                for (doc_type, sections) in &plan.classified_sections {
                    println!("\nTarget: {:?} ({} sections)", doc_type, sections.len());
                    for s in sections {
                        println!("  - [{}] {}", s.level, s.heading);
                    }
                }
                println!("\nRun with --apply to write to disk.\n");
            }
        }
    }
    Ok(())
}
