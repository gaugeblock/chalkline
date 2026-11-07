//! The finding type and the audit entry point for decision records.
//!
//! Split from `mod.rs` when that file passed the 250-line limit (`GB-291`): one job per file, and the two
//! jobs here are reporting a violation and recognising a record.

use super::finding;
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

    // History must read in order. A record's number is how it is cited, so backfilled decisions are numbered
    // by when they were made rather than when they were written down - and numbers must therefore never
    // increase while dates go backwards. Without this, a later backfill silently produces an index that
    // tells a false story about the project.
    let mut dated: Vec<(String, String, String)> = Vec::new();
    for (number, name) in &numbers {
        let Ok(content) = std::fs::read_to_string(dir.join(name)) else {
            continue;
        };
        let date = content.lines().find_map(|line| {
            line.strip_prefix("date:")
                .map(|value| value.trim().to_string())
        });
        if let Some(date) = date {
            dated.push((number.clone(), date, name.clone()));
        }
    }
    dated.sort_by(|a, b| a.0.cmp(&b.0));
    for pair in dated.windows(2) {
        if pair[1].1 < pair[0].1 {
            findings.push(DecisionFinding {
                path: format!("docs/decisions/{}", pair[1].2),
                message: format!(
                    "is numbered after {} but dated earlier ({} < {}), so the sequence no longer reads in \
                     historical order",
                    pair[0].2, pair[1].1, pair[0].1
                ),
            });
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
