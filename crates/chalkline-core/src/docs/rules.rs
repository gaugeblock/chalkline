//! Semantic validation and AST drift audit rules for canonical documentation.

use std::path::Path;
use miter_engine::WorkspaceInventory;
use pulldown_cmark::{Event, Parser, Tag};

/// Audits `ARCHITECTURE.md` crate topology against code AST.
pub fn audit_architecture(
    content: &str,
    inv: &WorkspaceInventory,
    violations: &mut Vec<String>,
) {
    for cr in &inv.crates {
        if !content.contains(&cr.name) {
            violations.push(format!("Missing mention of member crate '{}'", cr.name));
        }
    }
}

/// Audits `SPEC.md` boundary and contract types (refinement 2: wire models, error enums, CLI args, lib facade).
pub fn audit_spec_contracts(
    content: &str,
    inv: &WorkspaceInventory,
    violations: &mut Vec<String>,
) {
    for t in &inv.public_types {
        if !is_boundary_or_contract_type(t) {
            continue;
        }
        if !content.contains(&t.type_name) {
            violations.push(format!(
                "Missing formal specification for boundary type '{}' ({}::{})",
                t.type_name, t.module_path, t.type_name
            ));
        }
    }
}

fn is_boundary_or_contract_type(t: &miter_engine::PublicTypeInventory) -> bool {
    let name = &t.type_name;
    let mod_path = &t.module_path;

    // Error enums
    if name.ends_with("Error") || name.ends_with("ErrorKind") {
        return true;
    }
    // Wire models, configs, schemas, and receipts
    if name.ends_with("Config")
        || name.ends_with("Schema")
        || name.ends_with("Request")
        || name.ends_with("Response")
        || name.ends_with("Report")
        || name.ends_with("Receipt")
        || name.ends_with("Plan")
        || name.ends_with("Status")
        || name.ends_with("Item")
    {
        return true;
    }
    // Boundary and facade modules
    if mod_path.ends_with("lib")
        || mod_path.contains("model")
        || mod_path.contains("contracts")
        || mod_path.contains("wire")
        || mod_path.contains("api")
    {
        return true;
    }
    false
}

/// Audits `README.md` subcommands and options against Clap CLI commands.
pub fn audit_readme_cli(
    content: &str,
    inv: &WorkspaceInventory,
    violations: &mut Vec<String>,
) {
    for cmd in &inv.cli_commands {
        for sub in &cmd.subcommand_path {
            if !content.contains(sub) {
                violations.push(format!(
                    "README missing documentation for CLI command '{}'",
                    sub
                ));
            }
        }
        for arg in &cmd.args {
            if let Some(long) = &arg.long_name
                && long.len() > 3
                && !content.contains(long)
            {
                violations.push(format!(
                    "README missing documentation for CLI flag '--{}'",
                    long
                ));
            }
        }
    }
}

/// Audits `AGENTS.md` for mandatory non-negotiable operational laws.
pub fn audit_agents_directives(
    content: &str,
    violations: &mut Vec<String>,
) {
    let lower = content.to_lowercase();
    if !lower.contains("250") && !lower.contains("file limit") {
        violations.push("AGENTS.md missing mandatory rule: Hard 250-Line File Limit".into());
    }
    if !lower.contains("storypole defer") && !lower.contains("scope creep") {
        violations.push("AGENTS.md missing mandatory rule: Zero Inline Scope Creep / storypole defer".into());
    }
    if !lower.contains("tiered") && !lower.contains("verification") {
        violations.push("AGENTS.md missing mandatory rule: Tiered Verification Matrix".into());
    }
    if !lower.contains("conventional") && !lower.contains("commit") {
        violations.push("AGENTS.md missing mandatory rule: Conventional Commits specification".into());
    }
}

/// Audits relative markdown links to ensure target files exist on disk.
pub fn audit_markdown_links(
    content: &str,
    doc_dir: &Path,
    violations: &mut Vec<String>,
) {
    let parser = Parser::new(content);
    for event in parser {
        if let Event::Start(Tag::Link { dest_url, .. }) = event {
            let url_str = dest_url.as_ref();
            // Skip external URLs, mailto, anchor fragments, and custom schemes
            if url_str.starts_with("http://")
                || url_str.starts_with("https://")
                || url_str.starts_with("mailto:")
                || url_str.starts_with('#')
                || url_str.contains("://")
            {
                continue;
            }

            let path_part = url_str.split('#').next().unwrap_or(url_str);
            if path_part.is_empty() {
                continue;
            }

            let target_path = doc_dir.join(path_part);
            if !target_path.exists() {
                violations.push(format!("Broken relative link: '{}'", url_str));
            }
        }
    }
}

/// Audits `CHANGELOG.md` for active Unreleased heading.
pub fn audit_changelog(content: &str, violations: &mut Vec<String>) {
    if !content.contains("## [Unreleased]") && !content.contains("## Unreleased") {
        violations.push("Missing '## [Unreleased]' section in CHANGELOG.md".into());
    }
}

/// Audits `CONTRIBUTING.md` for configured toolchains.
pub fn audit_contributing(
    content: &str,
    inv: &WorkspaceInventory,
    violations: &mut Vec<String>,
) {
    if let Some(rust) = &inv.toolchain.rust_version
        && !content.contains(rust)
    {
        violations.push(format!("Missing configured Rust channel/version '{rust}'"));
    }
}
