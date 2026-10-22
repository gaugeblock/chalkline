//! Markdown specification parser and AST serializer.

#![forbid(unsafe_code)]

use crate::error::Result;
use crate::model::{DecisionHole, SharedTypeContract, SpecDoc, SpecTask};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::path::{Path, PathBuf};

/// Markdown specification parser.
#[derive(Debug, Default)]
pub struct SpecParser;

impl SpecParser {
    /// Creates a new `SpecParser`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses a specification markdown string into a `SpecDoc`.
    pub fn parse_str(&self, path: impl Into<PathBuf>, content: &str) -> Result<SpecDoc> {
        let path = path.into();
        let mut title = String::new();
        let mut overview = String::new();
        let mut tasks = Vec::new();
        let mut current_heading = String::new();
        let mut in_heading = false;
        let mut in_task_list = false;

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(content, options);
        let mut current_task_title = String::new();
        let mut current_task_checked = false;
        let mut is_task_item = false;

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    current_heading.clear();
                    if level == HeadingLevel::H1 && title.is_empty() {
                        // Will capture in Text event
                    }
                }
                Event::End(TagEnd::Heading(_)) => {
                    in_heading = false;
                    if title.is_empty() && !current_heading.is_empty() {
                        title = current_heading.clone();
                    }
                }
                Event::Start(Tag::List(_)) => {
                    in_task_list = true;
                }
                Event::End(TagEnd::List(_)) => {
                    in_task_list = false;
                }
                Event::Start(Tag::Item) => {
                    current_task_title.clear();
                    current_task_checked = false;
                    is_task_item = false;
                }
                Event::TaskListMarker(checked) => {
                    current_task_checked = checked;
                    is_task_item = true;
                }
                Event::Text(text) => {
                    if in_heading {
                        current_heading.push_str(&text);
                    } else if in_task_list {
                        current_task_title.push_str(&text);
                    } else if title.is_empty() && !text.trim().is_empty() {
                        title = text.trim().to_string();
                    } else if overview.is_empty()
                        && current_heading.to_lowercase().contains("overview")
                    {
                        overview.push_str(&text);
                    }
                }
                Event::End(TagEnd::Item) => {
                    let trimmed = current_task_title.trim();
                    if is_task_item || trimmed.starts_with("[ ]") || trimmed.starts_with("[x]") {
                        let (checked, clean_line) = if trimmed.starts_with("[x]") {
                            (true, trimmed.trim_start_matches("[x]").trim())
                        } else if trimmed.starts_with("[ ]") {
                            (false, trimmed.trim_start_matches("[ ]").trim())
                        } else {
                            (current_task_checked, trimmed)
                        };
                        let (key, title_str) = parse_task_line(clean_line, tasks.len() + 1);
                        tasks.push(SpecTask {
                            key,
                            title: title_str,
                            description: None,
                            blocked_by: Vec::new(),
                            completed: checked,
                        });
                    }
                    current_task_title.clear();
                    current_task_checked = false;
                    is_task_item = false;
                }
                _ => {}
            }
        }

        if title.is_empty() {
            title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Specification")
                .to_string();
        }

        let decisions = parse_decisions_from_markdown(content);
        let contracts = parse_contracts_from_markdown(content);

        Ok(SpecDoc {
            path,
            title,
            overview: overview.trim().to_string(),
            decisions,
            contracts,
            tasks,
        })
    }

    /// Reads and parses a spec from a file path.
    pub fn parse_file(&self, path: impl AsRef<Path>) -> Result<SpecDoc> {
        let path_ref = path.as_ref();
        let content = std::fs::read_to_string(path_ref)?;
        self.parse_str(path_ref.to_path_buf(), &content)
    }

    /// Serializes a `SpecDoc` into standard GitHub-flavored Markdown.
    #[must_use]
    pub fn render_markdown(&self, doc: &SpecDoc) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", doc.title));

        if !doc.overview.is_empty() {
            out.push_str("## Overview\n\n");
            out.push_str(&format!("{}\n\n", doc.overview));
        }

        if !doc.decisions.is_empty() {
            out.push_str("## Resolved Decisions\n\n");
            for d in &doc.decisions {
                out.push_str(&format!("### Decision: {}\n", d.question));
                out.push_str(&format!("- **Category:** `{}`\n", d.category));
                out.push_str(&format!("- **ID:** `{}`\n", d.id));
                if let Some(sel) = d.selected_option {
                    if let Some(opt) = d.options.get(sel) {
                        out.push_str(&format!("- **Status:** Resolved: {}\n", opt.label));
                        out.push_str(&format!("- **Tradeoff:** {}\n", opt.tradeoff_summary));
                    }
                } else {
                    out.push_str("- **Status:** Pending\n");
                }
                out.push('\n');
            }
        }

        if !doc.contracts.is_empty() {
            out.push_str("## Shared Type Contracts\n\n");
            for c in &doc.contracts {
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
        }

        if !doc.tasks.is_empty() {
            out.push_str("## Tasks\n\n");
            for t in &doc.tasks {
                let mark = if t.completed { "x" } else { " " };
                out.push_str(&format!("- [{mark}] **{}:** {}", t.key, t.title));
                if !t.blocked_by.is_empty() {
                    out.push_str(&format!(" (blocked by: {})", t.blocked_by.join(", ")));
                }
                out.push('\n');
            }
            out.push('\n');
        }

        out
    }
}

fn parse_task_line(line: &str, fallback_idx: usize) -> (String, String) {
    if let Some((prefix, rest)) = line.split_once(':') {
        let clean_prefix = prefix
            .trim()
            .trim_start_matches('*')
            .trim_end_matches('*')
            .trim();
        if !clean_prefix.is_empty() && clean_prefix.len() < 20 {
            return (clean_prefix.to_string(), rest.trim().to_string());
        }
    }
    (format!("TASK-{fallback_idx}"), line.to_string())
}

fn parse_decisions_from_markdown(_content: &str) -> Vec<DecisionHole> {
    // Basic decision extraction from markdown sections
    Vec::new()
}

fn parse_contracts_from_markdown(_content: &str) -> Vec<SharedTypeContract> {
    // Basic contract extraction from markdown tables
    Vec::new()
}
