//! Decision-record audit: MADR recognition for `docs/decisions/` (`GB-276`).
//!
//! Kept separate from the nine-document audit on purpose. The 9-doc set is a fixed contract with one file
//! per type; decision records are a growing set with their own rules. A tenth `DocType` would change
//! `DocType::ALL` and every count derived from it to describe something structurally different.
//!
//! Absence is not a violation. A repository with no `docs/decisions/` has made no recorded decisions yet,
//! and reporting that would make the audit noisy about repositories that are simply young.

use std::path::Path;

/// A problem with a decision record or the record set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionFinding {
    /// Path relative to the repository root.
    pub path: String,
    /// What is wrong and what would fix it.
    pub message: String,
}

/// Front-matter keys MADR expects before the body.
const REQUIRED_FRONT_MATTER: [&str; 2] = ["status", "date"];

/// Headings MADR requires in the body.
const REQUIRED_SECTIONS: [&str; 2] = ["Context and Problem Statement", "Decision Outcome"];

/// Audits `docs/decisions/` against the MADR shape, returning findings.
///
/// Returns an empty vec when the directory is absent - silence about decisions a repository has not made,
/// rather than a warning it cannot act on.
#[must_use]
pub fn audit_decision_records(root: &Path) -> Vec<DecisionFinding> {
    let dir = root.join("docs").join("decisions");
    if !dir.is_dir() {
        return Vec::new();
    }

    let mut findings = Vec::new();
    let mut numbers: Vec<(String, String)> = Vec::new();
    let mut records: Vec<String> = Vec::new();

    let Ok(entries) = std::fs::read_dir(&dir) else {
        return vec![DecisionFinding {
            path: "docs/decisions".to_string(),
            message: "cannot be read".to_string(),
        }];
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.eq_ignore_ascii_case("README.md") || name.eq_ignore_ascii_case("INDEX.md") {
            continue;
        }
        records.push(name.clone());

        let Some((number, _)) = name.split_once('-') else {
            findings.push(finding(
                &name,
                "is not named NNNN-slug.md, so it cannot be ordered",
            ));
            continue;
        };
        if number.len() != 4 || !number.chars().all(|c| c.is_ascii_digit()) {
            findings.push(finding(
                &name,
                &format!("numbering `{number}` is not four digits"),
            ));
        }
        if let Some((_, other)) = numbers.iter().find(|(existing, _)| existing == number) {
            findings.push(finding(
                &name,
                &format!("reuses number {number}, already used by {other}"),
            ));
        } else {
            numbers.push((number.to_string(), name.clone()));
        }

        let Ok(content) = std::fs::read_to_string(&path) else {
            findings.push(finding(&name, "cannot be read"));
            continue;
        };

        let front_matter_end = content
            .strip_prefix("---")
            .and_then(|rest| rest.find("\n---"))
            .map(|offset| offset + 3);
        match front_matter_end {
            None => findings.push(finding(
                &name,
                "has no --- front matter block, so its status and date are unknowable",
            )),
            Some(end) => {
                let front_matter = &content[..end];
                for key in REQUIRED_FRONT_MATTER {
                    if !front_matter.contains(key) {
                        findings.push(finding(&name, &format!("front matter is missing `{key}`")));
                    }
                }
            }
        }

        for section in REQUIRED_SECTIONS {
            if !content.contains(section) {
                findings.push(finding(
                    &name,
                    &format!("is missing the `{section}` section"),
                ));
            }
        }
    }

    // An index that omits a record is how a decision becomes unfindable.
    let index = ["README.md", "INDEX.md"]
        .iter()
        .map(|name| dir.join(name))
        .find(|path| path.is_file());
    if let Some(index_path) = index {
        let index_content = std::fs::read_to_string(&index_path).unwrap_or_default();
        for record in &records {
            if !index_content.contains(record.as_str()) {
                findings.push(DecisionFinding {
                    path: "docs/decisions".to_string(),
                    message: format!("the index does not list {record}, so it will not be found"),
                });
            }
        }
    } else if !records.is_empty() {
        findings.push(DecisionFinding {
            path: "docs/decisions".to_string(),
            message: format!(
                "holds {} record(s) but no README.md or INDEX.md listing them",
                records.len()
            ),
        });
    }

    findings
}

fn finding(name: &str, message: &str) -> DecisionFinding {
    DecisionFinding {
        path: format!("docs/decisions/{name}"),
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn repo() -> tempfile::TempDir {
        tempfile::TempDir::new().unwrap()
    }

    fn record(dir: &Path, name: &str, body: &str) {
        let decisions = dir.join("docs").join("decisions");
        std::fs::create_dir_all(&decisions).unwrap();
        std::fs::write(decisions.join(name), body).unwrap();
    }

    const GOOD: &str = "---\nstatus: accepted\ndate: 2026-11-06\n---\n\n## Context and Problem Statement\n\nWhy.\n\n## Decision Outcome\n\nWhat.\n";

    #[test]
    fn a_repository_with_no_decisions_is_not_a_violation() {
        assert!(audit_decision_records(repo().path()).is_empty());
    }

    #[test]
    fn a_well_formed_listed_record_reports_nothing() {
        let dir = repo();
        record(dir.path(), "0001-use-madr.md", GOOD);
        record(
            dir.path(),
            "README.md",
            "- [0001-use-madr.md](0001-use-madr.md)\n",
        );

        assert!(audit_decision_records(dir.path()).is_empty());
    }

    #[test]
    fn an_unlisted_record_is_reported_because_nobody_will_find_it() {
        let dir = repo();
        record(dir.path(), "0001-use-madr.md", GOOD);
        record(dir.path(), "README.md", "no links here\n");

        let findings = audit_decision_records(dir.path());
        assert!(
            findings
                .iter()
                .any(|finding| finding.message.contains("will not be found"))
        );
    }

    #[test]
    fn missing_front_matter_and_sections_are_reported() {
        let dir = repo();
        record(dir.path(), "0001-bare.md", "# Title only\n");
        record(dir.path(), "README.md", "0001-bare.md\n");

        let findings = audit_decision_records(dir.path());
        assert!(findings.iter().any(|f| f.message.contains("front matter")));
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("Decision Outcome"))
        );
    }

    #[test]
    fn bad_numbering_and_reuse_are_reported() {
        let dir = repo();
        record(dir.path(), "7-short.md", GOOD);
        record(dir.path(), "0007-taken.md", GOOD);
        record(dir.path(), "0007-again.md", GOOD);
        record(
            dir.path(),
            "README.md",
            "7-short.md 0007-taken.md 0007-again.md\n",
        );

        let findings = audit_decision_records(dir.path());
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("not four digits"))
        );
        assert!(findings.iter().any(|f| f.message.contains("reuses number")));
    }
}
