//! Handlers for `chalkline docs` commands.

use anyhow::{Result, anyhow};
use chalkline_core::docs::templates::DECISIONS_INDEX_TEMPLATE;
use chalkline_core::docs::{
    DocMigrationPlan, DocType, DocsConfig, inspect_workspace, render_template, sync_repo_docs,
};
use std::fs;
use std::path::Path;

use crate::args_docs::{DocsArgs, DocsCommands};

mod check;
use check::handle_check;

/// Dispatches `chalkline docs` subcommands.
pub fn handle_docs(root_dir: &Path, args: DocsArgs, json: bool) -> Result<()> {
    match args.command {
        DocsCommands::Check(check_args) => return handle_check(root_dir, &check_args, json),
        DocsCommands::Sync(sync_args) => {
            let target = sync_args.path.as_deref().unwrap_or(root_dir);
            let inv = inspect_workspace(target)
                .map_err(|e| anyhow!("Failed to inspect workspace: {e}"))?;
            let modified = sync_repo_docs(target, &inv, sync_args.apply)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&modified)?);
            } else if sync_args.apply {
                println!("Synchronized {} documentation files:", modified.len());
                for p in &modified {
                    println!("  + {}", p.display());
                }
            } else {
                println!(
                    "Preview: {} documentation files require sync:",
                    modified.len()
                );
                for p in &modified {
                    println!("  * {}", p.display());
                }
                println!("\nRun with --apply to write changes to disk.\n");
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

            // Decision records are scaffolded separately: they are a growing set with their own rules, not
            // one of the nine fixed documents. The index states the ordering rule where the next author will
            // read it, rather than in a conventions document they may never open (`GB-276`).
            let decisions_index = root_dir.join("docs").join("decisions").join("README.md");
            if !decisions_index.is_file() || scaffold_args.force {
                if let Some(parent) = decisions_index.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&decisions_index, DECISIONS_INDEX_TEMPLATE)?;
                created.push("docs/decisions/README.md".to_string());
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
                let written = plan
                    .execute(root_dir, &config)
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
                println!(
                    "\n=== Proposed Migration Plan for {} ===",
                    source_path.display()
                );
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
