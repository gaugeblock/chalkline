//! Model Context Protocol (MCP) stdio server for `chalkline`.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod dispatch;
pub mod schema;

use gaugeblock_mcp::{McpHandler, RpcRequest, RpcResponse, ToolOutcome};
use serde_json::Value;
use std::io::{self};
use std::path::Path;

/// MCP server version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// `chalkline`'s tool surface.
#[derive(Debug, Default)]
pub struct ChalklineHandler;

impl McpHandler for ChalklineHandler {
    fn server_name(&self) -> &str {
        "chalkline-mcp"
    }

    fn list_tools(&self) -> Value {
        Value::Array(schema::list_tools())
    }

    fn call_tool(&self, name: &str, arguments: &Value) -> ToolOutcome {
        match dispatch::dispatch_tool(name, arguments) {
            // Pretty-printed, as this server has always reported its reports.
            Ok(content) => {
                ToolOutcome::Text(serde_json::to_string_pretty(&content).unwrap_or_default())
            }
            Err(message) => ToolOutcome::Failed(message),
        }
    }
}

/// Runs the stdio JSON-RPC 2.0 MCP server loop over stdin and stdout.
///
/// # Errors
///
/// Returns the I/O failure when stdin cannot be read or stdout cannot be written.
pub fn run_stdio_server(_root_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Starting chalkline-mcp stdio server loop");
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    gaugeblock_mcp::serve(stdin.lock(), &mut stdout, &ChalklineHandler)?;
    Ok(())
}

/// Handles a single message.
///
/// Kept because the existing tests drive the server this way; the routing lives in `gaugeblock-mcp`.
#[must_use]
pub fn handle_message(msg: &Value) -> Option<Value> {
    let request: RpcRequest = serde_json::from_value(msg.clone()).ok()?;
    gaugeblock_mcp::dispatch(&ChalklineHandler, &request).map(RpcResponse::into_value)
}
