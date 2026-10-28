//! Unit tests for 9-Document architecture manager.

use std::path::PathBuf;

use miter_engine::{ToolchainInventory, WorkspaceInventory};

use super::*;

fn mock_inventory() -> WorkspaceInventory {
    WorkspaceInventory {
        root_dir: PathBuf::from("/mock"),
        workspace_name: Some("test-project".into()),
        crates: Vec::new(),
        cli_commands: Vec::new(),
        public_types: Vec::new(),
        git: None,
        toolchain: ToolchainInventory {
            rust_version: Some("1.90".into()),
            rust_edition: Some("2024".into()),
            node_version: Some(">=20.0.0".into()),
            package_manager: Some("pnpm@9.x".into()),
        },
    }
}

#[test]
fn test_render_all_templates() {
    let inv = mock_inventory();
    for doc in DocType::ALL {
        let rendered = render_template(doc, &inv);
        assert!(!rendered.is_empty());
        assert!(rendered.starts_with('#'));
    }
}

#[test]
fn test_docs_config_save_load() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    let root = tmp.path();

    let config = DocsConfig::default();
    config.save(root)?;

    let loaded = DocsConfig::load_or_default(root)?;
    assert_eq!(loaded.docs.readme.path, "README.md");
    assert!(loaded.policies.enforce_crate_topology);
    Ok(())
}

#[test]
fn test_drift_verifier_missing_files() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    let root = tmp.path();
    let inv = mock_inventory();
    let config = DocsConfig::default();

    let report = verify_docs_drift(root, &config, &inv);
    assert!(!report.is_clean());
    assert!(report.missing_count() > 0);
    Ok(())
}

#[test]
fn test_migration_plan_classification() {
    let content = r#"# Big Project

Welcome to Big Project.

## Autonomous Agents
Rules for agents:
1. Hard 250-line file limit.

## Prerequisites
Please install Rust 1.90 and pnpm.
"#;

    let config = DocsConfig::default();
    let plan =
        DocMigrationPlan::plan_from_str(PathBuf::from("README.md").as_path(), content, &config);

    assert!(plan.classified_sections.contains_key(&DocType::Agents));
    assert!(
        plan.classified_sections
            .contains_key(&DocType::Contributing)
    );
    assert!(plan.classified_sections.contains_key(&DocType::Readme));
}

#[test]
fn test_discover_workspace_repos() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    let root = tmp.path();

    std::fs::create_dir_all(root.join("sub-repo-a"))?;
    std::fs::write(root.join("sub-repo-a").join("Cargo.toml"), "[package]\nname=\"sub-repo-a\"")?;
    std::fs::write(root.join("sub-repo-a").join("README.md"), "# Sub Repo A")?;

    std::fs::create_dir_all(root.join("sub-repo-b"))?;
    std::fs::write(root.join("sub-repo-b").join("README.md"), "# Sub Repo B")?;
    std::fs::create_dir_all(root.join("sub-repo-b").join(".git"))?;

    let repos = discover_workspace_repos(root);
    assert_eq!(repos.len(), 3); // workspace root + sub-repo-a + sub-repo-b
    assert_eq!(repos[0].name, "workspace");
    assert!(repos[0].is_root);
    assert_eq!(repos[1].name, "sub-repo-a");
    assert_eq!(repos[2].name, "sub-repo-b");
    Ok(())
}

#[test]
fn test_spec_contracts_boundary_types() {
    let mut inv = mock_inventory();
    inv.public_types.push(miter_engine::PublicTypeInventory {
        module_path: "chalkline_core::error".into(),
        type_name: "ChalklineError".into(),
        kind: "enum".into(),
        doc_summary: Some("Error enum".into()),
    });
    inv.public_types.push(miter_engine::PublicTypeInventory {
        module_path: "chalkline_core::internal".into(),
        type_name: "TrivialHelper".into(),
        kind: "struct".into(),
        doc_summary: None,
    });

    let mut violations = Vec::new();
    rules::audit_spec_contracts("# Behavioral Spec\n\nNo types mentioned.", &inv, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].contains("ChalklineError"));

    let mut clean_violations = Vec::new();
    rules::audit_spec_contracts("# Behavioral Spec\n\nMentions ChalklineError explicitly.", &inv, &mut clean_violations);
    assert!(clean_violations.is_empty());
}

#[test]
fn test_agents_directives_and_links() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut violations = Vec::new();
    rules::audit_agents_directives("# Bad Agents\nNo rules here.", &mut violations);
    assert!(violations.len() >= 3);

    let tmp = tempfile::tempdir()?;
    let root = tmp.path();
    std::fs::write(root.join("real.md"), "# Real")?;

    let content = "See [real file](./real.md) and [fake file](./fake.md).";
    let mut link_violations = Vec::new();
    rules::audit_markdown_links(content, root, &mut link_violations);
    assert_eq!(link_violations.len(), 1);
    assert!(link_violations[0].contains("fake.md"));
    Ok(())
}

#[test]
fn test_heading_section_replacement() {
    let original = "# Title\n\nIntro.\n\n## Crate & Package Topology\n\nOld crate listing.\n\n## Next Section\n\nPreserved.\n";
    let replacement = "- New Crate A\n- New Crate B";
    let updated = replace_heading_section(original, "crate", replacement).expect("Heading replaced");
    assert!(updated.contains("- New Crate A"));
    assert!(updated.contains("## Next Section"));
    assert!(updated.contains("Preserved."));
    assert!(!updated.contains("Old crate listing."));
}
