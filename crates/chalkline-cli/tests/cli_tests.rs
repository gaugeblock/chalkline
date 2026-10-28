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

#[test]
fn test_cli_docs_check_workspace() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"repo-alpha\"]\n",
    )
    .unwrap();

    // Scaffold root docs
    let scaffold_root = Command::new(env!("CARGO_BIN_EXE_chalkline"))
        .arg("--dir")
        .arg(root)
        .arg("docs")
        .arg("scaffold")
        .output()
        .unwrap();
    if !scaffold_root.status.success() {
        eprintln!(
            "scaffold stderr: {}",
            String::from_utf8_lossy(&scaffold_root.stderr)
        );
    }
    assert!(scaffold_root.status.success());

    // Scaffold a member repo
    let member = root.join("repo-alpha");
    std::fs::create_dir_all(&member).unwrap();
    std::fs::write(member.join("Cargo.toml"), "[package]\nname = \"repo-alpha\"\n").unwrap();
    let scaffold_member = Command::new(env!("CARGO_BIN_EXE_chalkline"))
        .arg("--dir")
        .arg(&member)
        .arg("docs")
        .arg("scaffold")
        .output()
        .unwrap();
    assert!(scaffold_member.status.success());

    // Append member crate mention to root ARCHITECTURE.md so it is not drifted
    let arch_path = root.join("ARCHITECTURE.md");
    let mut arch = std::fs::read_to_string(&arch_path).unwrap();
    arch.push_str("\n### `repo-alpha`\nWorkspace member.\n");
    std::fs::write(&arch_path, arch).unwrap();

    // Run docs check --workspace in json mode
    let check_output = Command::new(env!("CARGO_BIN_EXE_chalkline"))
        .arg("--dir")
        .arg(root)
        .arg("--json")
        .arg("docs")
        .arg("check")
        .arg("--workspace")
        .output()
        .unwrap();

    if !check_output.status.success() {
        eprintln!(
            "check stdout: {}",
            String::from_utf8_lossy(&check_output.stdout)
        );
        eprintln!(
            "check stderr: {}",
            String::from_utf8_lossy(&check_output.stderr)
        );
    }
    assert!(check_output.status.success());
    let stdout = String::from_utf8(check_output.stdout).unwrap();
    assert!(stdout.contains("repo-alpha"));
    assert!(stdout.contains("workspace"));
}
