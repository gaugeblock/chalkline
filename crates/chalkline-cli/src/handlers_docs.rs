//! Handlers for `chalkline docs` commands.

use std::fs;
use std::path::Path;
use anyhow::{Result, anyhow};
use chalkline_core::docs::{
    DocMigrationPlan, DocStatus, DocType, DocsConfig, inspect_workspace, render_template,
    sync_repo_docs, verify_docs_drift, verify_workspace_docs_drift,
};

use crate::args_docs::{DocsArgs, DocsCommands};

/// Dispatches `chalkline docs` subcommands.
pub fn handle_docs(root_dir: &Path, args: DocsArgs, json: bool) -> Result<()> {
    match args.command {
        DocsCommands::Check(check_args) => {
            let target = check_args.path.as_deref().unwrap_or(root_dir);

            if check_args.workspace {
                let ws_report = verify_workspace_docs_drift(target)?;
                if json {
                    println!("{}", serde_json::to_string_pretty(&ws_report)?);
                } else {
                    println!("\n=== Gaugeblock 9-Document Architecture Audit (Workspace) ===");
                    println!("Workspace Root: {}\n", target.display());
                    println!("{:<14} {:<6} {:<7} {:<6} {:<6} {:<6} {:<8} {:<9} {:<11} {:<8} {:<10}",
                        "Repo", "README", "CONTRIB", "PROD", "ARCH", "SPEC", "DESIGN", "ROADMAP", "CHANGELOG", "AGENTS", "Status"
                    );
                    println!("{}", "-".repeat(95));
                    for (repo_name, report) in &ws_report.targets {
                        let mut status_map = std::collections::HashMap::new();
                        for item in &report.items {
                            status_map.insert(item.doc_type, item.status);
                        }
                        let badge = |dt: DocType| match status_map.get(&dt) {
                            Some(DocStatus::Present) => "[OK]  ",
                            Some(DocStatus::Drifted) => "[DIFF]",
                            Some(DocStatus::Missing) => "[MISS]",
                            None => " -    ",
                        };
                        let overall = if report.is_clean() { "CLEAN" } else { "VIOLATIONS" };
                        println!("{:<14} {:<6} {:<7} {:<6} {:<6} {:<6} {:<8} {:<9} {:<11} {:<8} {:<10}",
                            repo_name,
                            badge(DocType::Readme),
                            badge(DocType::Contributing),
                            badge(DocType::Product),
                            badge(DocType::Architecture),
                            badge(DocType::Spec),
                            badge(DocType::Design),
                            badge(DocType::Roadmap),
                            badge(DocType::Changelog),
                            badge(DocType::Agents),
                            overall
                        );
                    }
                    println!("\nSummary: {} targets audited, {} present, {} missing, {} drifted\n",
                        ws_report.targets.len(),
                        ws_report.total_present(),
                        ws_report.total_missing(),
                        ws_report.total_drifted(),
                    );
                }
                if !ws_report.is_clean() {
                    return Err(anyhow!("Workspace documentation audit failed with violations"));
                }
                return Ok(());
            }

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
                println!("Preview: {} documentation files require sync:", modified.len());
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
