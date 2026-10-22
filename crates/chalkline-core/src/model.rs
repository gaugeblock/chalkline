//! Shared domain models, decision holes, and specification contracts.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// An architectural decision hole identifying an ambiguity to resolve.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionHole {
    /// Unique identifier for this decision.
    pub id: Uuid,
    /// Category tag (e.g., "error_strategy", "auth_boundary", "schema_type").
    pub category: String,
    /// The question posed to the user or planner.
    pub question: String,
    /// Available options with tradeoffs.
    pub options: Vec<DecisionOption>,
    /// Index of option pre-resolved by codebase reconnaissance, if any.
    pub auto_resolved_by_miter: Option<usize>,
    /// Index of selected option once resolved.
    pub selected_option: Option<usize>,
}

impl DecisionHole {
    /// Creates a new unresolved decision hole.
    #[must_use]
    pub fn new(category: impl Into<String>, question: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            category: category.into(),
            question: question.into(),
            options: Vec::new(),
            auto_resolved_by_miter: None,
            selected_option: None,
        }
    }

    /// Adds an option to the decision hole.
    #[must_use]
    pub fn with_option(mut self, option: DecisionOption) -> Self {
        self.options.push(option);
        self
    }

    /// Resolves the decision with the given option index.
    pub fn resolve(&mut self, option_idx: usize) -> bool {
        if option_idx < self.options.len() {
            self.selected_option = Some(option_idx);
            true
        } else {
            false
        }
    }

    /// Returns true if this decision hole is resolved.
    #[must_use]
    pub fn is_resolved(&self) -> bool {
        self.selected_option.is_some()
    }
}

/// A specific choice for resolving a decision hole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionOption {
    /// Human-readable label for the choice.
    pub label: String,
    /// Pros, cons, and architectural tradeoffs.
    pub tradeoff_summary: String,
    /// Codebase symbols affected by this choice.
    pub impacted_symbols: Vec<String>,
}

impl DecisionOption {
    /// Creates a new decision option.
    #[must_use]
    pub fn new(
        label: impl Into<String>,
        tradeoff_summary: impl Into<String>,
        impacted_symbols: Vec<String>,
    ) -> Self {
        Self {
            label: label.into(),
            tradeoff_summary: tradeoff_summary.into(),
            impacted_symbols,
        }
    }
}

/// Neutral wire schema locked by `chalkline` so `dado` can emit matching Rust + TS types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTypeContract {
    /// Name of the contract type (e.g., "UserSession", "TaskReceipt").
    pub name: String,
    /// Typed fields comprising the contract.
    pub fields: Vec<ContractField>,
}

/// A single field inside a `SharedTypeContract`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractField {
    /// Field identifier.
    pub name: String,
    /// Canonical neutral type: "uuid", "string", "i64", "vec<string>", "option<i32>".
    pub canonical_type: String,
    /// Whether the field is mandatory.
    pub required: bool,
}

/// A parsed task item from a specification document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecTask {
    /// Task key or ID (e.g., "TASK-1").
    pub key: String,
    /// Summary title of the task.
    pub title: String,
    /// Optional description or details.
    pub description: Option<String>,
    /// Keys of tasks that block this task.
    pub blocked_by: Vec<String>,
    /// Whether the task is marked completed in the spec markdown.
    pub completed: bool,
}

/// In-memory representation of a parsed specification document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecDoc {
    /// Path on disk.
    pub path: PathBuf,
    /// Spec document title.
    pub title: String,
    /// High-level overview description.
    pub overview: String,
    /// List of decision holes (both pending and resolved).
    pub decisions: Vec<DecisionHole>,
    /// Locked shared type contracts.
    pub contracts: Vec<SharedTypeContract>,
    /// Ordered list of tasks.
    pub tasks: Vec<SpecTask>,
}
