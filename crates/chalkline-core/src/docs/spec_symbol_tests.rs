//! Tests for SPEC contract parsing and the reverse-direction AST audit (GB-138).

use std::path::PathBuf;

use miter_engine::{PublicTypeInventory, ToolchainInventory, WorkspaceInventory};

use crate::docs::spec_symbols::audit_spec_declared_contracts;
use crate::spec_contracts::parse_contracts_from_markdown;

fn inventory_with(types: &[&str]) -> WorkspaceInventory {
    WorkspaceInventory {
        root_dir: PathBuf::from("/mock"),
        workspace_name: Some("test-project".into()),
        crates: Vec::new(),
        cli_commands: Vec::new(),
        public_types: types
            .iter()
            .map(|name| PublicTypeInventory {
                module_path: "test_project::contracts".into(),
                type_name: (*name).to_string(),
                kind: "struct".into(),
                doc_summary: None,
            })
            .collect(),
        git: None,
        toolchain: ToolchainInventory {
            rust_version: Some("1.90".into()),
            rust_edition: Some("2024".into()),
            node_version: None,
            package_manager: None,
        },
    }
}

const SPEC: &str = r#"# Specification

## Shared Type Contracts

### Contract: `TaskReceipt`

| Field | Type | Required |
| :--- | :--- | :--- |
| `task_id` | `uuid` | true |
| `verifier` | `string` | false |

### Contract: `RetiredReport`

| Field | Type | Required |
| :--- | :--- | :--- |
| `id` | `uuid` | true |
"#;

#[test]
fn parses_the_contract_block_that_render_emits() {
    let contracts = parse_contracts_from_markdown(SPEC);

    assert_eq!(contracts.len(), 2, "both declared contracts are read");
    assert_eq!(contracts[0].name, "TaskReceipt");
    assert_eq!(contracts[0].fields.len(), 2);
    assert_eq!(contracts[0].fields[0].name, "task_id");
    assert_eq!(contracts[0].fields[0].canonical_type, "uuid");
    assert!(contracts[0].fields[0].required);
    assert!(!contracts[0].fields[1].required, "false is not required");
    assert_eq!(contracts[1].name, "RetiredReport");
}

#[test]
fn prose_mentioning_a_contract_declares_nothing() {
    let prose = "# Spec\n\nWe once had a `TaskReceipt` and a\n| table | that is not a contract |\n";
    assert!(
        parse_contracts_from_markdown(prose).is_empty(),
        "only a contract heading opens a contract"
    );
}

#[test]
fn a_declared_contract_absent_from_the_ast_is_drift() {
    let inv = inventory_with(&["TaskReceipt"]);
    let mut violations = Vec::new();

    audit_spec_declared_contracts(SPEC, &inv, &mut violations);

    assert_eq!(violations.len(), 1, "only the retired contract drifts");
    assert!(
        violations[0].contains("RetiredReport"),
        "the violation names the missing contract: {}",
        violations[0]
    );
    assert!(
        !violations[0].contains("TaskReceipt"),
        "a contract the AST still defines is not drift"
    );
}

#[test]
fn a_spec_in_step_with_the_ast_reports_nothing() {
    let inv = inventory_with(&["TaskReceipt", "RetiredReport"]);
    let mut violations = Vec::new();

    audit_spec_declared_contracts(SPEC, &inv, &mut violations);

    assert!(violations.is_empty(), "got: {violations:?}");
}
