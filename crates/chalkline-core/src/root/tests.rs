//! `--dir` accepts a path or a member name, and says which it used (`GB-323`).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;

use super::resolve_root;

/// A suite root with a declared member `alpha`, both canonical.
fn suite(root: &Path) -> std::path::PathBuf {
    fs::create_dir_all(root.join(".gaugeblock")).expect("suite state");
    fs::write(
        root.join(".gaugeblock").join("config.toml"),
        "[suite]\nmembers = [\"alpha\"]\n",
    )
    .expect("declaration");
    fs::create_dir_all(root.join("alpha").join(".git")).expect("member");
    std::fs::canonicalize(root).expect("canonical")
}

#[test]
fn a_directory_relative_to_the_working_directory_wins() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = suite(tmp.path());
    let member = root.join("alpha");

    // From the suite root, `alpha` is a directory here - the historical meaning.
    assert_eq!(
        resolve_root(Some(Path::new("alpha")), &root).unwrap(),
        member
    );
}

#[test]
fn a_member_name_resolves_from_inside_that_member() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = suite(tmp.path());
    let member = root.join("alpha");

    // The case that failed: from inside `alpha`, `alpha` is not a subdirectory, and it used to resolve
    // to `alpha/alpha` and report "Missing Cargo.toml at alpha".
    assert_eq!(
        resolve_root(Some(Path::new("alpha")), &member).unwrap(),
        member
    );
}

#[test]
fn no_value_means_the_working_directory() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = suite(tmp.path());
    assert_eq!(resolve_root(None, &root).unwrap(), root);
}

#[test]
fn an_unknown_value_names_both_things_it_could_have_meant() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = suite(tmp.path());

    let error = resolve_root(Some(Path::new("beta")), &root).expect_err("beta is neither");
    let message = error.to_string();
    assert!(message.contains("neither a directory"), "{message}");
    assert!(message.contains("member of the suite"), "{message}");
}
