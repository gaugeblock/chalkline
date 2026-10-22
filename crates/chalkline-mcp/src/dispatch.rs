//! Tool call dispatcher for `chalkline` MCP server.

#![forbid(unsafe_code)]

use chalkline_core::{DagCompiler, Griller, ReconEngine, SpecParser};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

/// Dispatches an MCP tool call to the appropriate `chalkline-core` implementation.
pub fn dispatch_tool(name: &str, args: &Value) -> Result<Value, String> {
    match name {
        "chalkline_recon" => {
            let repo_path = get_path_arg(args, "repo_path").unwrap_or_else(|| PathBuf::from("."));
            let recon = ReconEngine::new();
            let report = recon
                .scan_repo(&repo_path)
                .map_err(|e| format!("Recon error: {e}"))?;
            Ok(json!({
                "report": report
            }))
        }
        "chalkline_grill" => {
            let prompt = args
                .get("prompt")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing required parameter 'prompt'".to_string())?;
            let repo_path = get_path_arg(args, "repo_path").unwrap_or_else(|| PathBuf::from("."));
            let spec_path = get_path_arg(args, "spec_path");

            let spec_parser = SpecParser::new();
            let existing_doc = spec_path
                .as_ref()
                .and_then(|p| spec_parser.parse_file(p).ok());

            let recon = ReconEngine::new();
            let report = recon.scan_repo(&repo_path).ok();

            let griller = Griller::new();
            let mut holes = griller
                .identify_decision_holes(prompt, existing_doc.as_ref())
                .map_err(|e| format!("Griller error: {e}"))?;

            if let Some(rep) = report {
                recon.auto_resolve_decisions(&rep, &mut holes);
            }

            Ok(json!({
                "prompt": prompt,
                "decision_holes": holes,
                "unresolved_count": holes.iter().filter(|h| !h.is_resolved()).count(),
            }))
        }
        "chalkline_compile_dag" => {
            let spec_path_str = args
                .get("spec_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing required parameter 'spec_path'".to_string())?;
            let repo_path = get_path_arg(args, "repo_path").unwrap_or_else(|| PathBuf::from("."));

            let spec_parser = SpecParser::new();
            let doc = spec_parser
                .parse_file(Path::new(spec_path_str))
                .map_err(|e| format!("Failed to parse spec file: {e}"))?;

            let compiler = DagCompiler::new();
            let receipt = compiler
                .compile(&repo_path, &doc)
                .map_err(|e| format!("Failed to compile DAG into storypole: {e}"))?;

            Ok(json!({
                "receipt": receipt
            }))
        }
        "chalkline_sync" => {
            let spec_path_str = args
                .get("spec_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing required parameter 'spec_path'".to_string())?;
            let spec_path = Path::new(spec_path_str);

            let spec_parser = SpecParser::new();
            let doc = spec_parser
                .parse_file(spec_path)
                .map_err(|e| format!("Failed to parse spec file: {e}"))?;

            // Render updated markdown back to disk
            let rendered = spec_parser.render_markdown(&doc);
            std::fs::write(spec_path, rendered)
                .map_err(|e| format!("Failed to write synced spec: {e}"))?;

            Ok(json!({
                "status": "synced",
                "tasks_count": doc.tasks.len()
            }))
        }
        unknown => Err(format!("Unknown tool: {unknown}")),
    }
}

fn get_path_arg(args: &Value, field: &str) -> Option<PathBuf> {
    args.get(field).and_then(|v| v.as_str()).map(PathBuf::from)
}
