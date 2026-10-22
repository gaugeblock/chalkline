#![allow(clippy::unwrap_used, clippy::expect_used)]

use chalkline_mcp::handle_message;
use serde_json::json;

#[test]
fn test_mcp_initialize() {
    let req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {}
    });

    let resp = handle_message(&req).unwrap();
    assert_eq!(resp["id"], 1);
    assert_eq!(resp["result"]["serverInfo"]["name"], "chalkline-mcp");
}

#[test]
fn test_mcp_list_tools() {
    let req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    });

    let resp = handle_message(&req).unwrap();
    assert_eq!(resp["id"], 2);
    let tools = resp["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 4);
    assert!(tools.iter().any(|t| t["name"] == "chalkline_recon"));
    assert!(tools.iter().any(|t| t["name"] == "chalkline_grill"));
    assert!(tools.iter().any(|t| t["name"] == "chalkline_compile_dag"));
    assert!(tools.iter().any(|t| t["name"] == "chalkline_sync"));
}

#[test]
fn test_mcp_call_recon() {
    let req = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "chalkline_recon",
            "arguments": {
                "repo_path": "."
            }
        }
    });

    let resp = handle_message(&req).unwrap();
    assert_eq!(resp["id"], 3);
    assert!(
        resp["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("primary_language")
    );
}
