#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::process::Command;
use storypole_core::StorypoleConfig;
use storypole_core::store::StorypoleStore;
use tempfile::tempdir;

#[test]
fn test_cli_recon_json() {
    let dir = tempdir().unwrap();
    let cargo_toml = dir.path().join("Cargo.toml");
    std::fs::write(
        &cargo_toml,
        "[package]\nname = \"foo\"\nedition = \"2024\"\n",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_chalkline"))
        .arg("--dir")
        .arg(dir.path())
        .arg("--json")
        .arg("recon")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("primary_language"));
    assert!(stdout.contains("2024"));
}

#[test]
fn test_cli_grill_and_compile_cycle() {
    let dir = tempdir().unwrap();
    let config = StorypoleConfig::default();
    let _store = StorypoleStore::init(dir.path(), config).unwrap();

    let spec_path = dir.path().join("SPEC.md");
    let spec_content = r#"# Sample Service

## Tasks

- [ ] **TASK-1:** Initialize schema
- [ ] **TASK-2:** Wire handlers
"#;
    std::fs::write(&spec_path, spec_content).unwrap();

    // 1. Grill non-interactively
    let output = Command::new(env!("CARGO_BIN_EXE_chalkline"))
        .arg("--dir")
        .arg(dir.path())
        .arg("grill")
        .arg("--non-interactive")
        .output()
        .unwrap();

    assert!(output.status.success());
    let updated_spec = std::fs::read_to_string(&spec_path).unwrap();
    assert!(updated_spec.contains("Resolved Decisions"));

    // 2. Compile DAG
    let compile_output = Command::new(env!("CARGO_BIN_EXE_chalkline"))
        .arg("--dir")
        .arg(dir.path())
        .arg("compile-dag")
        .output()
        .unwrap();

    assert!(compile_output.status.success());
    let compile_stdout = String::from_utf8(compile_output.stdout).unwrap();
    assert!(compile_stdout.contains("Tasks Compiled:   2"));

    // 3. Sync spec
    let sync_output = Command::new(env!("CARGO_BIN_EXE_chalkline"))
        .arg("--dir")
        .arg(dir.path())
        .arg("sync")
        .output()
        .unwrap();

    assert!(sync_output.status.success());
}
