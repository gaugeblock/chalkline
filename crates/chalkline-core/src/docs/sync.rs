//! Native Markdown AST heading-targeted documentation synchronizer via pulldown-cmark.

use std::fs;
use std::path::{Path, PathBuf};
use miter_engine::WorkspaceInventory;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

use crate::error::Result;

/// Replaces the content under a Markdown heading until the next heading of equal or higher level.
pub fn replace_heading_section(
    content: &str,
    target_heading: &str,
    replacement: &str,
) -> Option<String> {
    let parser = Parser::new(content);
    let mut target_level: Option<HeadingLevel> = None;
    let mut current_heading_text = String::new();
    let mut in_target_heading = false;
    let mut section_content_start: Option<usize> = None;
    let mut section_content_end: Option<usize> = None;

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                if let Some(t_level) = target_level {
                    if level <= t_level {
                        section_content_end = Some(range.start);
                        break;
                    }
                } else {
                    current_heading_text.clear();
                    in_target_heading = true;
                }
            }
            Event::Text(t) | Event::Code(t) if in_target_heading => {
                current_heading_text.push_str(&t);
            }
            Event::End(TagEnd::Heading(level)) if in_target_heading => {
                in_target_heading = false;
                let normalized = current_heading_text.trim().to_lowercase();
                if normalized.contains(&target_heading.to_lowercase()) {
                    target_level = Some(level);
                    section_content_start = Some(range.end);
                }
            }
            _ => {}
        }
    }

    let start = section_content_start?;
    let end = section_content_end.unwrap_or(content.len());

    let mut out = String::with_capacity(content.len() + replacement.len());
    out.push_str(&content[..start]);
    out.push_str("\n\n");
    out.push_str(replacement.trim());
    out.push_str("\n\n");
    out.push_str(content[end..].trim_start_matches('\n'));
    Some(out)
}

/// Renders crate topology list for `ARCHITECTURE.md`.
pub fn render_crates_section(inv: &WorkspaceInventory) -> String {
    let mut out = String::new();
    if inv.crates.is_empty() {
        out.push_str("- Standalone root crate\n");
    } else {
        for cr in &inv.crates {
            let desc = cr.description.as_deref().unwrap_or("Workspace member crate");
            out.push_str(&format!(
                "### `{}`\n- **Path:** `{}`\n- **Version:** `{}`\n- **Description:** {}\n\n",
                cr.name, cr.relative_path.display(), cr.version, desc
            ));
        }
    }
    out
}

/// Renders CLI reference table for `README.md`.
pub fn render_cli_reference_section(inv: &WorkspaceInventory) -> String {
    let mut out = String::from("| Command | Description |\n|---|---|\n");
    if inv.cli_commands.is_empty() {
        out.push_str("| *(none)* | No CLI subcommands registered |\n");
    } else {
        for cmd in &inv.cli_commands {
            let path = cmd.subcommand_path.join(" ");
            let desc = cmd.description.as_deref().unwrap_or("-");
            out.push_str(&format!("| `{} {}` | {} |\n", cmd.binary_name, path, desc));
        }
    }
    out
}

/// Renders formal boundary contracts for `SPEC.md`.
pub fn render_boundary_contracts_section(inv: &WorkspaceInventory) -> String {
    let mut out = String::from("| Type | Kind | Module | Summary |\n|---|---|---|---|\n");
    let mut count = 0;
    for t in &inv.public_types {
        let name = &t.type_name;
        if name.ends_with("Error")
            || name.ends_with("Config")
            || name.ends_with("Schema")
            || name.ends_with("Report")
            || name.ends_with("Plan")
            || name.ends_with("Status")
        {
            let doc = t.doc_summary.as_deref().unwrap_or("-");
            out.push_str(&format!("| `{}` | `{}` | `{}` | {} |\n", t.type_name, t.kind, t.module_path, doc));
            count += 1;
        }
    }
    if count == 0 {
        out.push_str("| *(none)* | - | - | No public boundary types detected |\n");
    }
    out
}

/// Synchronizes AST-grounded sections into canonical documents.
pub fn sync_repo_docs(
    root: &Path,
    inv: &WorkspaceInventory,
    apply: bool,
) -> Result<Vec<PathBuf>> {
    let mut modified = Vec::new();

    // 1. ARCHITECTURE.md -> Crate Topology
    let arch_path = root.join("ARCHITECTURE.md");
    if arch_path.is_file() {
        let content = fs::read_to_string(&arch_path)?;
        let rendered = render_crates_section(inv);
        if let Some(updated) = replace_heading_section(&content, "crate", &rendered)
            && updated != content
        {
            if apply {
                fs::write(&arch_path, updated)?;
            }
            modified.push(arch_path);
        }
    }

    // 2. README.md -> CLI Reference
    let readme_path = root.join("README.md");
    if readme_path.is_file() && !inv.cli_commands.is_empty() {
        let content = fs::read_to_string(&readme_path)?;
        let rendered = render_cli_reference_section(inv);
        if let Some(updated) = replace_heading_section(&content, "cli reference", &rendered)
            && updated != content
        {
            if apply {
                fs::write(&readme_path, updated)?;
            }
            modified.push(readme_path);
        }
    }

    // 3. SPEC.md -> Formal Type Contracts
    let spec_path = root.join("SPEC.md");
    if spec_path.is_file() {
        let content = fs::read_to_string(&spec_path)?;
        let rendered = render_boundary_contracts_section(inv);
        if let Some(updated) = replace_heading_section(&content, "contract", &rendered)
            && updated != content
        {
            if apply {
                fs::write(&spec_path, updated)?;
            }
            modified.push(spec_path);
        }
    }

    Ok(modified)
}
