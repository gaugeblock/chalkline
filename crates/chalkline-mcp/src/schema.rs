//! Tool schemas for the `chalkline` Model Context Protocol (MCP) server.

#![forbid(unsafe_code)]

use serde_json::{Value, json};

/// Returns the list of MCP tools exposed by `chalkline`.
#[must_use]
pub fn list_tools() -> Vec<Value> {
    vec![
        json!({
            "name": "chalkline_recon",
            "description": "Inspect target codebase for architectural conventions (language edition, error crate, async runtime, tracing) using miter-engine.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "repo_path": {
                        "type": "string",
                        "description": "Absolute or relative path to target repository root (defaults to current dir)."
                    }
                }
            }
        }),
        json!({
            "name": "chalkline_grill",
            "description": "Analyze feature prompt or spec, identify Decision Holes, auto-resolve known conventions, and return remaining questions with tradeoffs.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "prompt": {
                        "type": "string",
                        "description": "Feature requirement, user prompt, or spec draft to analyze."
                    },
                    "repo_path": {
                        "type": "string",
                        "description": "Target repository path."
                    },
                    "spec_path": {
                        "type": "string",
                        "description": "Optional path to existing SPEC.md or PRODUCT.md file."
                    }
                },
                "required": ["prompt"]
            }
        }),
        json!({
            "name": "chalkline_compile_dag",
            "description": "Parse tasks and blockers from a specification file, and compile them into a topological task DAG in storypole.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "spec_path": {
                        "type": "string",
                        "description": "Path to SPEC.md or specification file."
                    },
                    "repo_path": {
                        "type": "string",
                        "description": "Target repository path containing .storypole/."
                    }
                },
                "required": ["spec_path"]
            }
        }),
        json!({
            "name": "chalkline_sync",
            "description": "Sync task completion states between storypole tasks and specification checklist items.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "spec_path": {
                        "type": "string",
                        "description": "Path to specification markdown file."
                    },
                    "repo_path": {
                        "type": "string",
                        "description": "Target repository root."
                    }
                },
                "required": ["spec_path"]
            }
        }),
    ]
}
