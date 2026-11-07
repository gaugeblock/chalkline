//! `chalkline docs check`: the nine-document and decision-record audit (`GB-291`).
//!
//! Split out of the dispatch when that file passed the 250-line limit. The arm was 133 lines on its own,
//! which is the size of a module rather than the size of a match arm.

use anyhow::{Result, anyhow};
use chalkline_core::docs::{
    DocStatus, DocType, DocsConfig, inspect_workspace, verify_docs_drift,
    verify_workspace_docs_drift,
};
use std::path::Path;

use crate::args_docs::DocsCheckArgs;

/// Runs the audit for one repository or a whole workspace.
pub(crate) fn handle_check(root_dir: &Path, check_args: &DocsCheckArgs, json: bool) -> Result<()> {
    let target = check_args.path.as_deref().unwrap_or(root_dir);

    if check_args.workspace {
        let ws_report = verify_workspace_docs_drift(target)?;
        if json {
            println!("{}", serde_json::to_string_pretty(&ws_report)?);
        } else {
            println!("\n=== Gaugeblock 9-Document Architecture Audit (Workspace) ===");
            println!("Workspace Root: {}\n", target.display());
            println!(
                "{:<14} {:<6} {:<7} {:<6} {:<6} {:<6} {:<8} {:<9} {:<11} {:<8} {:<10}",
                "Repo",
                "README",
                "CONTRIB",
                "PROD",
                "ARCH",
                "SPEC",
                "DESIGN",
                "ROADMAP",
                "CHANGELOG",
                "AGENTS",
                "Status"
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
                let overall = if report.is_clean() {
                    "CLEAN"
                } else {
                    "VIOLATIONS"
                };
                println!(
                    "{:<14} {:<6} {:<7} {:<6} {:<6} {:<6} {:<8} {:<9} {:<11} {:<8} {:<10}",
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
            println!(
                "\nSummary: {} targets audited, {} present, {} missing, {} drifted\n",
                ws_report.targets.len(),
                ws_report.total_present(),
                ws_report.total_missing(),
                ws_report.total_drifted(),
            );
        }
        if !ws_report.is_clean() {
            return Err(anyhow!(
                "Workspace documentation audit failed with violations"
            ));
        }
        return Ok(());
    }

    let config = DocsConfig::load_or_default(target)?;
    let inv = inspect_workspace(target).map_err(|e| anyhow!("Failed to inspect workspace: {e}"))?;
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
            println!(
                "{} {:<16} ({})",
                badge,
                format!("{:?}", item.doc_type),
                item.path.display()
            );
            for v in &item.violations {
                println!("           * {v}");
            }
        }
        println!(
            "\nSummary: {} present, {} missing, {} drifted\n",
            report
                .items
                .iter()
                .filter(|i| i.status == DocStatus::Present)
                .count(),
            report.missing_count(),
            report.drifted_count(),
        );

        // Decision records are audited separately from the nine documents (`GB-276`): they are a
        // growing set with their own rules, not a tenth fixed document. Absence is not a failure.
        let decisions = chalkline_core::docs::decisions::audit_decision_records(target);
        if !decisions.is_empty() {
            println!("Decision records (MADR):");
            for finding in &decisions {
                println!("  * {}: {}", finding.path, finding.message);
            }
            println!();
        }
    }

    // A drifted or missing document is a failure, not a note: callers gate on the exit
    // code, and a green exit with drift in the summary makes the check decorative.
    let decision_findings = chalkline_core::docs::decisions::audit_decision_records(target);
    if !report.is_clean() || !decision_findings.is_empty() {
        return Err(anyhow!(
            "Documentation audit failed: {} missing, {} drifted, {} decision-record finding(s)",
            report.missing_count(),
            report.drifted_count(),
            decision_findings.len()
        ));
    }

    Ok(())
}
