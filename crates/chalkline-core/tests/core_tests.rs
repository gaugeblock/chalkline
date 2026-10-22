#![allow(clippy::unwrap_used, clippy::expect_used)]

use chalkline_core::model::{
    ContractField, DecisionHole, DecisionOption, SharedTypeContract, SpecDoc, SpecTask,
};
use chalkline_core::{ContractsGenerator, DagCompiler, Griller, ReconEngine, SpecParser};
use std::path::PathBuf;
use storypole_core::StorypoleConfig;
use storypole_core::store::StorypoleStore;
use tempfile::tempdir;

#[test]
fn test_spec_parser_roundtrip() {
    let parser = SpecParser::new();
    let sample = r#"# User Authentication Service

## Overview

A high-performance session authentication service.

## Tasks

- [ ] **TASK-1:** Implement auth token validation
- [x] **TASK-2:** Add rate limiting middleware
"#;

    let doc = parser.parse_str("SPEC.md", sample).unwrap();
    assert_eq!(doc.title, "User Authentication Service");
    assert_eq!(doc.tasks.len(), 2);
    assert_eq!(doc.tasks[0].key, "TASK-1");
    assert!(!doc.tasks[0].completed);
    assert_eq!(doc.tasks[1].key, "TASK-2");
    assert!(doc.tasks[1].completed);

    let rendered = parser.render_markdown(&doc);
    assert!(rendered.contains("# User Authentication Service"));
    assert!(rendered.contains("- [ ] **TASK-1:** Implement auth token validation"));
    assert!(rendered.contains("- [x] **TASK-2:** Add rate limiting middleware"));
}

#[test]
fn test_recon_and_auto_resolution() {
    let dir = tempdir().unwrap();
    let cargo_toml = dir.path().join("Cargo.toml");
    let manifest = r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"

[dependencies]
thiserror = "2.0"
tokio = { version = "1.0", features = ["full"] }
serde = "1.0"
tracing = "0.1"
"#;
    std::fs::write(&cargo_toml, manifest).unwrap();

    let recon = ReconEngine::new();
    let report = recon.scan_repo(dir.path()).unwrap();
    assert_eq!(report.rust_edition.as_deref(), Some("2024"));
    assert_eq!(report.error_strategy, "thiserror");
    assert_eq!(report.async_runtime, "tokio");

    let mut hole = DecisionHole::new("error_strategy", "What error crate?");
    hole.options.push(DecisionOption::new(
        "Strongly typed thiserror",
        "Recommended for libraries",
        vec!["Error".to_string()],
    ));
    hole.options.push(DecisionOption::new(
        "Dynamic anyhow",
        "Recommended for bins",
        vec![],
    ));

    let mut holes = vec![hole];
    recon.auto_resolve_decisions(&report, &mut holes);
    assert_eq!(holes[0].auto_resolved_by_miter, Some(0));
    assert_eq!(holes[0].selected_option, Some(0));
    assert!(holes[0].is_resolved());
}

#[tokio::test]
async fn test_griller_ambiguity_detection() {
    let griller = Griller::new();
    let prompt = "Build a fast user service";
    let holes = griller.identify_decision_holes(prompt, None).await.unwrap();

    assert!(!holes.is_empty());
    assert!(holes.iter().any(|h| h.category == "persistence_strategy"));
    assert!(holes.iter().any(|h| h.category == "transport_boundary"));
}

#[test]
fn test_contracts_generator_export() {
    let dir = tempdir().unwrap();
    let generator = ContractsGenerator::new();

    let contract = SharedTypeContract {
        name: "UserSession".to_string(),
        fields: vec![
            ContractField {
                name: "session_id".to_string(),
                canonical_type: "uuid".to_string(),
                required: true,
            },
            ContractField {
                name: "username".to_string(),
                canonical_type: "string".to_string(),
                required: true,
            },
        ],
    };

    let written = generator.export_contracts(dir.path(), &[contract]).unwrap();
    assert_eq!(written.len(), 1);
    assert!(written[0].exists());

    let content = std::fs::read_to_string(&written[0]).unwrap();
    assert!(content.contains("UserSession"));
    assert!(content.contains("session_id"));
}

#[test]
fn test_dag_compiler_with_storypole() {
    let dir = tempdir().unwrap();
    let config = StorypoleConfig::default();
    let _store = StorypoleStore::init(dir.path(), config).unwrap();

    let doc = SpecDoc {
        path: PathBuf::from("SPEC.md"),
        title: "Test Feature".to_string(),
        overview: "Test feature overview".to_string(),
        decisions: vec![],
        contracts: vec![],
        tasks: vec![
            SpecTask {
                key: "TASK-1".to_string(),
                title: "Setup schema".to_string(),
                description: None,
                blocked_by: vec![],
                completed: false,
            },
            SpecTask {
                key: "TASK-2".to_string(),
                title: "Setup handlers".to_string(),
                description: None,
                blocked_by: vec!["TASK-1".to_string()],
                completed: false,
            },
        ],
    };

    let compiler = DagCompiler::new();
    let receipt = compiler.compile(dir.path(), &doc).unwrap();
    assert_eq!(receipt.tasks_compiled, 2);
    assert_eq!(receipt.edges_linked, 1);
    assert_eq!(receipt.ticket_keys.len(), 2);
}
