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
