//! Parsing for the `## Shared Type Contracts` section of a specification.
//!
//! The format is the one `SpecParser::render_markdown` emits, so a rendered spec parses back into the
//! same contracts:
//!
//! ```text
//! ### Contract: `TaskReceipt`
//!
//! | Field | Type | Required |
//! | :--- | :--- | :--- |
//! | `task_id` | `uuid` | true |
//! ```
//!
//! Anything outside such a section is ignored, which keeps prose that mentions a contract from
//! inventing one.

use crate::model::{ContractField, SharedTypeContract};

const CONTRACT_PREFIX: &str = "### Contract:";

/// Extracts every shared type contract declared in `content`.
pub(crate) fn parse_contracts_from_markdown(content: &str) -> Vec<SharedTypeContract> {
    let mut contracts: Vec<SharedTypeContract> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix(CONTRACT_PREFIX) {
            if let Some(name) = contract_name(rest) {
                contracts.push(SharedTypeContract {
                    name,
                    fields: Vec::new(),
                });
            }
            continue;
        }

        let Some(current) = contracts.last_mut() else {
            continue;
        };
        if !trimmed.starts_with('|') {
            continue;
        }
        if let Some(field) = contract_field(trimmed) {
            current.fields.push(field);
        }
    }

    contracts
}

/// Reads the backticked (or bare) type name from a contract heading.
fn contract_name(rest: &str) -> Option<String> {
    let trimmed = rest.trim();
    let name = trimmed
        .strip_prefix('`')
        .and_then(|inner| inner.split('`').next())
        .unwrap_or(trimmed)
        .trim();
    (!name.is_empty() && !name.contains('|')).then(|| name.to_string())
}

/// Reads one `| field | type | required |` row, skipping the header and separator rows.
fn contract_field(row: &str) -> Option<ContractField> {
    let cells: Vec<&str> = row
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim())
        .collect();
    if cells.len() < 3 {
        return None;
    }

    let name = cells[0].trim_matches('`').trim();
    let canonical_type = cells[1].trim_matches('`').trim();
    let required_cell = cells[2].to_ascii_lowercase();
    if name.is_empty() || name.eq_ignore_ascii_case("field") || name.starts_with(':') {
        return None;
    }
    if required_cell.starts_with(':') || required_cell == "required" {
        return None;
    }

    Some(ContractField {
        name: name.to_string(),
        canonical_type: canonical_type.to_string(),
        required: matches!(required_cell.as_str(), "true" | "yes" | "required"),
    })
}
