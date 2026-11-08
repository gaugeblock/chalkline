//! Resolving `--dir` to a repository root (`GB-323`).
//!
//! The flag accepted a directory and a member name interchangeably, and the same value meant different
//! things depending on where it was typed: `—dir common` from the suite root named a directory, `--dir
//! common` from inside `common` named `common/common` and failed with "Missing Cargo.toml at common".
//! The eleven verification chains were written the second way and had never been executed, so the
//! ambiguity survived until a chain finally ran.
//!
//! Both readings are kept, resolved in order and never guessed silently: a directory relative to the
//! working directory, then a member of the suite that directory belongs to. When neither holds, the
//! error states both interpretations.

use std::path::{Path, PathBuf};

use crate::error::ChalklineError;

/// Resolves `input` (the `--dir` value) against `cwd`.
///
/// `None` means "the working directory", which is what the flag means when it is absent.
///
/// # Errors
///
/// [`ChalklineError::Validation`] when the value is neither a directory nor a member of the enclosing
/// suite, naming both things it could have meant.
pub fn resolve_root(input: Option<&Path>, cwd: &Path) -> Result<PathBuf, ChalklineError> {
    let Some(input) = input else {
        return Ok(cwd.to_path_buf());
    };
    // An absolute path is unambiguous.
    if input.is_absolute() {
        return input.is_dir().then(|| input.to_path_buf()).ok_or_else(|| {
            ChalklineError::Validation(format!("--dir '{}' is not a directory", input.display()))
        });
    }
    // A directory relative to the working directory, which is what the flag has always documented.
    let relative = cwd.join(input);
    if relative.is_dir() {
        return Ok(relative);
    }
    // A member name, resolved against the suite this directory belongs to. Canonicalised first, because
    // `Path::ancestors` on a relative path stops at the current directory and membership would find no
    // suite above it - which is a bug this same resolution produced once already (`GB-327`).
    let absolute = std::fs::canonicalize(cwd).unwrap_or_else(|_| cwd.to_path_buf());
    if let Some(membership) = gaugeblock_workspace::Membership::containing(&absolute) {
        let as_member = membership.suite_root.join(input);
        if as_member.is_dir() {
            return Ok(as_member);
        }
    }
    Err(ChalklineError::Validation(format!(
        "--dir '{}' is neither a directory ({} does not exist) nor a member of the suite containing {}",
        input.display(),
        relative.display(),
        absolute.display()
    )))
}

#[cfg(test)]
mod tests;
