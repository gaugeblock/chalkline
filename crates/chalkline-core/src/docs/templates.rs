//! Markdown templates for the 9 canonical repository documents.

use miter_engine::WorkspaceInventory;
use crate::docs::config::DocType;

/// Renders a scaffolded template for any of the 9 canonical documentation types.
pub fn render_template(doc_type: DocType, inv: &WorkspaceInventory) -> String {
    match doc_type {
        DocType::Readme => render_readme(inv),
        DocType::Contributing => render_contributing(inv),
        DocType::Product => render_product(inv),
        DocType::Architecture => render_architecture(inv),
        DocType::Spec => render_spec(inv),
        DocType::Design => render_design(inv),
        DocType::Roadmap => render_roadmap(inv),
        DocType::Changelog => render_changelog(inv),
        DocType::Agents => render_agents(inv),
    }
}

fn render_readme(inv: &WorkspaceInventory) -> String {
    let title = inv.workspace_name.as_deref().unwrap_or("Project");
    let mut out = format!("# `{title}`\n\n> High-reliability, deterministic neurosymbolic systems engineering.\n\n");
    out.push_str("## Ecosystem Architecture\n\nPart of the **[`gaugeblock`](https://gaugeblock.dev)** ecosystem.\n\n");
    out.push_str("## CLI Reference\n\n| Command | Description |\n|---|---|\n");
    if inv.cli_commands.is_empty() {
        out.push_str("| *(none)* | Standalone library / service crate |\n");
    } else {
        for cmd in &inv.cli_commands {
            let p = cmd.subcommand_path.join(" ");
            let d = cmd.description.as_deref().unwrap_or("-");
            out.push_str(&format!("| `{} {}` | {} |\n", cmd.binary_name, p, d));
        }
    }
    out.push_str("\n## Quickstart\n\n```bash\ncargo build --release\n```\n");
    out
}

fn render_contributing(inv: &WorkspaceInventory) -> String {
    let mut out = String::from("# Contributing Guide\n\n## Prerequisites\n\n");
    let rust = inv.toolchain.rust_version.as_deref().unwrap_or("1.90");
    out.push_str(&format!("- **Rust:** `{rust}` (Edition 2024)\n"));
    out.push_str("- **Runner:** `cargo-nextest` preferred\n\n");
    out.push_str("## Inner Development Loop\n\n");
    out.push_str("1. **Tier 1 (Inner Loop):** `cargo check -p <crate>`\n");
    out.push_str("2. **Tier 2 (Commit Gate):** `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`\n");
    out.push_str("3. **Tier 3 (Task Gate):** Full workspace verification before closing tasks\n");
    out
}

fn render_product(inv: &WorkspaceInventory) -> String {
    let title = inv.workspace_name.as_deref().unwrap_or("Product");
    format!(
        "# {title} — Product Vision\n\n## Personas & Problems\n\n\
        - **Systems Engineers:** Require deterministic, contract-driven pipelines without silent regressions.\n\
        - **Autonomous Agents:** Require machine-readable AST slices and append-only task ledgers.\n\n\
        ## Jobs to Be Done\n\n\
        1. Eliminate semantic documentation drift across code AST, CLI, and wire types.\n\
        2. Provide fast, sub-second deterministic feedback in the inner loop.\n"
    )
}

fn render_architecture(inv: &WorkspaceInventory) -> String {
    let mut out = String::from("# System Architecture\n\n## Subsystem Topology\n\n```mermaid\nflowchart TD\n    CLI[\"CLI Binary\"] --> Core[\"Domain Core Engine\"]\n    MCP[\"MCP Stdio Server\"] --> Core\n```\n\n## Crate & Package Topology\n\n");
    if inv.crates.is_empty() {
        out.push_str("- Standalone root crate\n");
    } else {
        for cr in &inv.crates {
            let desc = cr.description.as_deref().unwrap_or("Workspace member crate");
            out.push_str(&format!("### `{}`\n- **Path:** `{}`\n- **Version:** `{}`\n- **Description:** {}\n\n",
                cr.name, cr.relative_path.display(), cr.version, desc));
        }
    }
    out
}

fn render_spec(inv: &WorkspaceInventory) -> String {
    let mut out = String::from("# Behavioral Specifications & Invariants\n\n## Invariant Rules\n\n1. **Deterministic Output:** Pure function of input tree and arguments.\n2. **Safe Error Handling:** Strong `thiserror` enums, zero unwrap/expect in library paths.\n\n## Formal Type Contracts\n\n| Type | Kind | Module | Summary |\n|---|---|---|---|\n");
    let mut count = 0;
    for t in &inv.public_types {
        if t.type_name.ends_with("Error") || t.type_name.ends_with("Config") || t.type_name.ends_with("Report") {
            let d = t.doc_summary.as_deref().unwrap_or("-");
            out.push_str(&format!("| `{}` | `{}` | `{}` | {} |\n", t.type_name, t.kind, t.module_path, d));
            count += 1;
        }
    }
    if count == 0 {
        out.push_str("| *(none)* | - | - | Boundary contracts defined in domain core |\n");
    }
    out
}

fn render_design(inv: &WorkspaceInventory) -> String {
    let title = inv.workspace_name.as_deref().unwrap_or("System");
    format!(
        "# {title} — Architectural Design & Decisions\n\n\
        ## Architectural Decision Records (ADRs)\n\n\
        ### ADR-001: Deterministic 9-Document Topology\n\
        - **Status:** Accepted\n\
        - **Context:** Resolves documentation drift across human and LLM workflows.\n\
        - **Consequences:** Clear boundary separation, AST-grounded audits.\n\n\
        ### ADR-002: Strongly Typed Error Boundaries with Thiserror\n\
        - **Status:** Accepted\n\
        - **Context:** Avoid unwrap panics in library core crates.\n\
        - **Consequences:** Zero runtime panics; explicit error taxonomy.\n"
    )
}

fn render_roadmap(inv: &WorkspaceInventory) -> String {
    let title = inv.workspace_name.as_deref().unwrap_or("Project");
    format!(
        "# {title} — Roadmap & Horizons\n\n## Active Milestone (v0.1.0-beta)\n\n\
        - [x] Initial architecture definition\n\
        - [x] Core engine and CLI implementation\n\
        - [ ] Comprehensive AST drift verification\n\n\
        ## Future Horizon\n\n- Cross-repository telemetry and automated patch synthesis\n"
    )
}

fn render_changelog(inv: &WorkspaceInventory) -> String {
    let mut out = String::from("# Changelog\n\nAll notable changes will be documented in this file.\n\n## [Unreleased]\n\n### Added\n");
    if let Some(git) = &inv.git {
        for commit in git.unreleased_commits.iter().take(10) {
            out.push_str(&format!("- {commit}\n"));
        }
    }
    out
}

fn render_agents(inv: &WorkspaceInventory) -> String {
    let title = inv.workspace_name.as_deref().unwrap_or("Project");
    format!(
        "# AGENTS.md — Operational Directives for {title}\n\n\
        ## 1. Non-Negotiable Engineering Laws\n\n\
        1. **Hard 250-Line File Limit:** No `.rs`, `.ts`, or `.tsx` source file may exceed 250 lines of code.\n\
        2. **Unix Philosophy & Standalone Value:** Pure modularity with strict wire contracts.\n\
        3. **Zero Inline Scope Creep (storypole defer):** Unrelated defects must be recorded via `storypole defer`.\n\
        4. **Tiered Verification Matrix:** Tier 1 (check), Tier 2 (fmt, clippy -D warnings, test), Tier 3 (workspace).\n\
        5. **Strict Error Handling:** Strongly typed `thiserror` enums; zero unwrap/expect in library crates.\n\
        6. **Atomic Conventional Commits:** `<type>(<scope>): <summary>` format.\n"
    )
}
