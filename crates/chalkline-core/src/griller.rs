//! Specification griller: identifies ambiguities and drives structured Q&A.

#![forbid(unsafe_code)]

use crate::error::Result;
use crate::model::{DecisionHole, DecisionOption, SpecDoc};
use sawhorse_core::ModelRole;
use sawhorse_core::types::InferenceRequest;

/// Ambiguity griller and decision engine.
#[derive(Debug, Default)]
pub struct Griller;

impl Griller {
    /// Creates a new `Griller`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Identifies architectural ambiguities and missing decisions in a feature prompt or spec.
    pub fn identify_decision_holes(
        &self,
        prompt: &str,
        existing_doc: Option<&SpecDoc>,
    ) -> Result<Vec<DecisionHole>> {
        let mut holes = Vec::new();

        // 1. Check storage/persistence hole
        if !prompt.contains("sqlite") && !prompt.contains("jsonl") && !prompt.contains("memory") {
            let mut hole = DecisionHole::new(
                "persistence_strategy",
                "How should runtime state and cache be persisted?",
            );
            hole.options.push(DecisionOption::new(
                "Git-Native JSONL + Ephemeral SQLite",
                "Deterministic, mergeable, and zero server process required.",
                vec!["storage::ledger".to_string(), "storage::index".to_string()],
            ));
            hole.options.push(DecisionOption::new(
                "Pure In-Memory with Atomic Snapshots",
                "Simplest implementation; loses ephemeral state between restarts.",
                vec!["storage::memory".to_string()],
            ));
            holes.push(hole);
        }

        // 2. Check communication/transport boundary
        if !prompt.contains("mcp") && !prompt.contains("cli") && !prompt.contains("http") {
            let mut hole = DecisionHole::new(
                "transport_boundary",
                "What interface protocol should external agents use to interact with this component?",
            );
            hole.options.push(DecisionOption::new(
                "Stdio JSON-RPC 2.0 (MCP Protocol)",
                "Native compatibility with Claude, Cursor, and gaugeblock agents.",
                vec!["mcp::server".to_string()],
            ));
            hole.options.push(DecisionOption::new(
                "Local HTTP/REST + WebSocket Daemon",
                "Enables browser UIs and cross-language microservice calls.",
                vec!["daemon::http".to_string()],
            ));
            holes.push(hole);
        }

        // 3. Check error handling strategy
        let mut err_hole = DecisionHole::new(
            "error_strategy",
            "What error handling pattern should library crates follow?",
        );
        err_hole.options.push(DecisionOption::new(
            "Strongly Typed thiserror Enums",
            "Zero unwrap/expect in production library paths; explicit domain errors.",
            vec!["error::Error".to_string()],
        ));
        err_hole.options.push(DecisionOption::new(
            "Dynamic anyhow::Result at boundary",
            "Simpler authoring, but caller cannot exhaustively match error variants.",
            vec!["error::Result".to_string()],
        ));
        holes.push(err_hole);

        // Filter out already resolved decisions from existing_doc
        if let Some(doc) = existing_doc {
            holes.retain(|h| {
                !doc.decisions
                    .iter()
                    .any(|existing| existing.category == h.category)
            });
        }

        Ok(holes)
    }

    /// Formulates an LLM inference request for `sawhorse` to evaluate deeper edge cases.
    #[must_use]
    pub fn build_planner_request(&self, spec_summary: &str) -> InferenceRequest {
        InferenceRequest {
            role: ModelRole::Planner,
            prompt: format!(
                "Analyze this technical specification and list unresolved architectural ambiguities:\n\n{spec_summary}"
            ),
            system_prompt: Some(
                "You are an expert systems engineering griller. Identify missing technical decisions and tradeoffs."
                    .to_string(),
            ),
            grammar_constraint: None,
            grammar_format: None,
            max_tokens: 512,
            temperature: 0.2,
            reasoning_effort: None,
        }
    }
}
