//! Cross-language contract generator and JSON schema exporter.

#![forbid(unsafe_code)]

use crate::error::Result;
use crate::model::SharedTypeContract;
use std::path::Path;

/// Cross-language contract generator and exporter.
#[derive(Debug, Default)]
pub struct ContractsGenerator;

impl ContractsGenerator {
    /// Creates a new `ContractsGenerator`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Serializes a contract to formatted JSON.
    pub fn to_json(&self, contract: &SharedTypeContract) -> Result<String> {
        let json = serde_json::to_string_pretty(contract)?;
        Ok(json)
    }

    /// Exports a set of shared type contracts to `.chalkline/contracts/<name>.json`.
    pub fn export_contracts(
        &self,
        repo_root: impl AsRef<Path>,
        contracts: &[SharedTypeContract],
    ) -> Result<Vec<std::path::PathBuf>> {
        let dir = repo_root.as_ref().join(".chalkline").join("contracts");
        std::fs::create_dir_all(&dir)?;

        let mut written_files = Vec::new();
        for contract in contracts {
            let file_name = format!("{}.json", contract.name.to_lowercase());
            let file_path = dir.join(file_name);
            let json = self.to_json(contract)?;
            std::fs::write(&file_path, json)?;
            written_files.push(file_path);
        }

        Ok(written_files)
    }

    /// Renders a Markdown table representing the contracts.
    #[must_use]
    pub fn render_markdown_table(&self, contracts: &[SharedTypeContract]) -> String {
        let mut out = String::new();
        for c in contracts {
            out.push_str(&format!("### Contract: `{}`\n\n", c.name));
            out.push_str("| Field | Type | Required |\n");
            out.push_str("| :--- | :--- | :--- |\n");
            for f in &c.fields {
                out.push_str(&format!(
                    "| `{}` | `{}` | {} |\n",
                    f.name, f.canonical_type, f.required
                ));
            }
            out.push('\n');
        }
        out
    }
}
