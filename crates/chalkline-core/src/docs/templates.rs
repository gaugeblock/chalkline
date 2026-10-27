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
    let mut out =
        format!("# {title}\n\n> Deterministic, high-reliability systems engineering.\n\n");
    out.push_str("## Features\n\n- Modular architecture\n- Formal AST grounding\n- Tiered verification gates\n\n");
    out.push_str("## Quickstart\n\n```sh\ncargo install\n```\n");
    out
}

fn render_contributing(inv: &WorkspaceInventory) -> String {
    let mut out = String::from("# Contributing Guide\n\nThank you for contributing!\n\n");
    out.push_str("## Prerequisites\n\n");
    if let Some(rust) = &inv.toolchain.rust_version {
        out.push_str(&format!("- **Rust:** `{rust}`\n"));
    }
    if let Some(node) = &inv.toolchain.node_version {
        out.push_str(&format!("- **Node.js:** `{node}`\n"));
    }
    if let Some(pm) = &inv.toolchain.package_manager {
        out.push_str(&format!("- **Package Manager:** `{pm}`\n"));
    }
    out.push_str("\n## Development Workflow\n\n1. Fork and create a branch\n2. Run tests: `cargo test`\n3. Submit a Pull Request\n");
    out
}

fn render_product(inv: &WorkspaceInventory) -> String {
    let title = inv.workspace_name.as_deref().unwrap_or("Product");
    format!(
        "# {title} — Product Vision\n\n## Personas & Problems\n\n- **Target Audience:** Systems engineers, developers, autonomous agents\n- **Core Value Proposition:** Deterministic, contract-driven code generation\n\n## Jobs to Be Done\n\n1. Automate task DAG resolution\n2. Prevent semantic documentation drift\n"
    )
}

fn render_architecture(inv: &WorkspaceInventory) -> String {
    let mut out = String::from("# System Architecture\n\n## Crate & Package Topology\n\n");
    if inv.crates.is_empty() {
        out.push_str("- Standalone root crate\n");
    } else {
        for cr in &inv.crates {
            let desc = cr
                .description
                .as_deref()
                .unwrap_or("Workspace member crate");
            out.push_str(&format!(
                "### `{}`\n- Path: `{}`\n- Version: `{}`\n- Description: {}\n\n",
                cr.name,
                cr.relative_path.display(),
                cr.version,
                desc
            ));
        }
    }
    out
}

fn render_spec(inv: &WorkspaceInventory) -> String {
    let mut out =
        String::from("# Behavioral Specifications & Invariants\n\n## Formal Type Contracts\n\n");
    if inv.public_types.is_empty() {
        out.push_str("*(No public types discovered)*\n");
    } else {
        for t in inv.public_types.iter().take(20) {
            let doc = t
                .doc_summary
                .as_deref()
                .unwrap_or("Public interface entity");
            out.push_str(&format!(
                "- **`{}::{}`** (`{}`): {}\n",
                t.module_path, t.type_name, t.kind, doc
            ));
        }
    }
    out
}

fn render_design(inv: &WorkspaceInventory) -> String {
    let title = inv.workspace_name.as_deref().unwrap_or("System");
    format!(
        "# {title} — Architectural Design & Decisions\n\n## Architectural Decision Records (ADRs)\n\n### ADR-001: Deterministic 9-Document Topology\n- **Status:** Accepted\n- **Context:** Resolves documentation drift across human and LLM workflows\n- **Consequences:** Clear boundary separation, AST-grounded audits\n"
    )
}

fn render_roadmap(inv: &WorkspaceInventory) -> String {
    let title = inv.workspace_name.as_deref().unwrap_or("Project");
    format!(
        "# {title} — Roadmap & Horizons\n\n## Active Milestone (v0.1.0)\n\n- [x] Initial architecture definition\n- [ ] Full AST cross-checks\n- [ ] UI integration\n\n## Future Horizon\n\n- Autonomous multi-repo synchronization\n"
    )
}

fn render_changelog(inv: &WorkspaceInventory) -> String {
    let mut out = String::from(
        "# Changelog\n\nAll notable changes to this project will be documented in this file.\n\n## [Unreleased]\n\n### Added\n",
    );
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
        "# AGENTS.md — Operational Directives for {title}\n\n## 1. Non-Negotiable Engineering Laws\n\n1. **Hard 250-Line File Limit:** No `.rs`, `.ts`, or `.tsx` source file may exceed 250 lines.\n2. **Unix Philosophy:** Standalone crate value with clean abstractions.\n3. **Zero Inline Scope Creep:** Defer discoveries to `storypole defer`.\n4. **Tiered Verification:** Ensure Tier 1-3 gates pass.\n"
    )
}
