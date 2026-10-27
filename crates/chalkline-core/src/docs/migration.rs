//! Brownfield documentation section disentangler and migration planner.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::docs::config::{DocType, DocsConfig};
use crate::error::Result;

/// A parsed markdown section with heading and text block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkdownSection {
    /// Section heading title.
    pub heading: String,
    /// Heading level (1 for `#`, 2 for `##`, etc.).
    pub level: usize,
    /// Body content including sub-paragraphs and code fences.
    pub body: String,
}

/// Proposed plan for disentangling a monolithic document into the 9 canonical documents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocMigrationPlan {
    /// Source document path.
    pub source_path: PathBuf,
    /// Classified sections grouped by target document type.
    pub classified_sections: HashMap<DocType, Vec<MarkdownSection>>,
}

impl DocMigrationPlan {
    /// Generates a migration plan from an existing file.
    pub fn plan_from_file(source_path: &Path, config: &DocsConfig) -> Result<Self> {
        let content = fs::read_to_string(source_path)?;
        Ok(Self::plan_from_str(source_path, &content, config))
    }

    /// Generates a migration plan from a markdown string.
    pub fn plan_from_str(source_path: &Path, content: &str, _config: &DocsConfig) -> Self {
        let sections = parse_markdown_sections(content);
        let mut classified = HashMap::new();

        for section in sections {
            let target_doc = classify_section(&section);
            classified
                .entry(target_doc)
                .or_insert_with(Vec::new)
                .push(section);
        }

        Self {
            source_path: source_path.to_path_buf(),
            classified_sections: classified,
        }
    }

    /// Executes the migration plan non-destructively by writing/appending target files.
    pub fn execute(&self, root: &Path, config: &DocsConfig) -> Result<Vec<PathBuf>> {
        let mut written_files = Vec::new();

        for (doc_type, sections) in &self.classified_sections {
            let entry = config.entry_for(*doc_type);
            let target_path = root.join(&entry.path);

            let mut content = if target_path.is_file() {
                fs::read_to_string(&target_path)?
            } else {
                String::new()
            };

            for sec in sections {
                let hashes = "#".repeat(sec.level);
                if !content.is_empty() {
                    content.push_str("\n\n");
                }
                content.push_str(&format!(
                    "{} {}\n\n{}",
                    hashes,
                    sec.heading,
                    sec.body.trim()
                ));
            }

            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&target_path, content)?;
            written_files.push(target_path);
        }

        Ok(written_files)
    }
}

fn parse_markdown_sections(content: &str) -> Vec<MarkdownSection> {
    let mut sections = Vec::new();
    let mut current_heading = String::from("Introduction");
    let mut current_level = 1;
    let mut current_body = Vec::new();

    for line in content.lines() {
        if line.starts_with('#') {
            let count = line.chars().take_while(|c| *c == '#').count();
            let title = line.trim_start_matches('#').trim();

            if !current_body.is_empty() || !current_heading.is_empty() {
                sections.push(MarkdownSection {
                    heading: current_heading,
                    level: current_level,
                    body: current_body.join("\n"),
                });
                current_body.clear();
            }

            current_heading = title.to_string();
            current_level = count;
        } else {
            current_body.push(line);
        }
    }

    if !current_heading.is_empty() || !current_body.is_empty() {
        sections.push(MarkdownSection {
            heading: current_heading,
            level: current_level,
            body: current_body.join("\n"),
        });
    }

    sections
}

fn classify_section(section: &MarkdownSection) -> DocType {
    let text = format!("{} {}", section.heading, section.body).to_lowercase();

    if text.contains("autonomous agent")
        || text.contains("file limit")
        || text.contains("engineering laws")
    {
        DocType::Agents
    } else if text.contains("keep a changelog")
        || text.contains("[unreleased]")
        || text.contains("release notes")
    {
        DocType::Changelog
    } else if text.contains("prerequisite")
        || text.contains("contributing")
        || text.contains("pull request")
    {
        DocType::Contributing
    } else if text.contains("subsystem")
        || text.contains("crate topology")
        || text.contains("architecture")
    {
        DocType::Architecture
    } else if text.contains("decision record") || text.contains("trade-off") || text.contains("adr")
    {
        DocType::Design
    } else if text.contains("wire schema")
        || text.contains("invariants")
        || text.contains("specification")
    {
        DocType::Spec
    } else if text.contains("milestone") || text.contains("horizon") || text.contains("roadmap") {
        DocType::Roadmap
    } else if text.contains("persona")
        || text.contains("jobs to be done")
        || text.contains("product vision")
    {
        DocType::Product
    } else {
        DocType::Readme
    }
}
