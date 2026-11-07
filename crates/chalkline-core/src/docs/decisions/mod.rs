//! Decision-record audit: MADR recognition for `docs/decisions/` (`GB-276`).
//!
//! Kept separate from the nine-document audit on purpose. The 9-doc set is a fixed contract with one file
//! per type; decision records are a growing set with their own rules. A tenth `DocType` would change
//! `DocType::ALL` and every count derived from it to describe something structurally different.
//!
//! Absence is not a violation. A repository with no `docs/decisions/` has made no recorded decisions yet,
//! and reporting that would make the audit noisy about repositories that are simply young.

mod decision_finding;
pub use decision_finding::{DecisionFinding, audit_decision_records};

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
    use std::path::Path;

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
    fn a_record_numbered_after_a_later_one_but_dated_earlier_is_reported() {
        // The backfill case: numbering by write order instead of decision date tells a false story.
        let dir = repo();
        record(
            dir.path(),
            "0001-later.md",
            "---
status: accepted
date: 2026-11-06
---

## Context and Problem Statement

## Decision Outcome
",
        );
        record(
            dir.path(),
            "0002-earlier.md",
            "---
status: accepted
date: 2026-10-01
---

## Context and Problem Statement

## Decision Outcome
",
        );
        record(
            dir.path(),
            "README.md",
            "0001-later.md 0002-earlier.md
",
        );

        let findings = audit_decision_records(dir.path());
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("historical order")),
            "expected an ordering finding, got {findings:?}"
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
