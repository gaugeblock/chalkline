//! Model Context Protocol (MCP) stdio server for `chalkline`.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod dispatch;
pub mod schema;

use dispatch::dispatch_tool;
use schema::list_tools;
use serde_json::{Value, json};
use std::io::{self, BufRead, Write};
use std::path::Path;

/// MCP server version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Runs the stdio JSON-RPC 2.0 MCP server loop over stdin and stdout.
pub fn run_stdio_server(_root_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    tracing::info!("Starting chalkline-mcp stdio server loop");

    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("Failed to read stdin line: {e}");
                break;
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let msg: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Failed to parse JSON-RPC message: {e}");
                continue;
            }
        };

        if let Some(resp) = handle_message(&msg) {
            let resp_str = serde_json::to_string(&resp)?;
            writeln!(stdout, "{resp_str}")?;
            stdout.flush()?;
        }
    }

    Ok(())
}

/// Handles a single JSON-RPC 2.0 message and generates a response.
#[must_use]
pub fn handle_message(msg: &Value) -> Option<Value> {
    let id = msg.get("id").cloned();
    let method = msg.get("method").and_then(|v| v.as_str())?;

    match method {
        "initialize" => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "chalkline-mcp",
                    "version": VERSION
                }
            }
        })),
        "notifications/initialized" => None,
        "tools/list" => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": list_tools()
            }
        })),
        "tools/call" => {
            let params = msg.get("params");
            let tool_name = params
                .and_then(|p| p.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let arguments = params
                .and_then(|p| p.get("arguments"))
                .cloned()
                .unwrap_or_else(|| json!({}));

            match dispatch_tool(tool_name, &arguments) {
                Ok(content) => Some(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&content).unwrap_or_default()
                        }]
                    }
                })),
                Err(err) => Some(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "isError": true,
                        "content": [{
                            "type": "text",
                            "text": err
                        }]
                    }
                })),
            }
        }
        _ => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32601,
                "message": format!("Method not found: {method}")
            }
        })),
    }
}
