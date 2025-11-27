// Import necessary items from our dependencies
use rmcp::{
    RoleServer,
    ServiceExt,
    handler::server::ServerHandler,
    model::{
        CallToolRequestParam, CallToolResult, Content, ErrorData, Implementation,
        InitializeRequestParam, InitializeResult, ListToolsResult, PaginatedRequestParam,
        ProtocolVersion, ServerCapabilities, Tool,
    },
    schemars, // For generating the "menu"
    service::RequestContext,
    transport::stdio, // The stdio communication channel
};
use serde::Deserialize; // For our tool's inputs

// Esplora API base URLs
const BITCOIN_API_BASE: &str = "https://blockstream.info/api";
const LIQUID_API_BASE: &str = "https://blockstream.info/liquid/api";

// 1. DEFINE YOUR TOOL'S INPUT PARAMETERS
// The AI will see this and know what to provide.
// 'schemars::JsonSchema' automatically builds the "menu" for the AI.
#[derive(Deserialize, schemars::JsonSchema)]
struct GetBitcoinTxParams {
    #[schemars(description = "The transaction ID (txid) hash to look up.")]
    txid: String,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct GetLiquidTxParams {
    #[schemars(description = "The transaction ID (txid) hash to look up.")]
    txid: String,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct GetBitcoinBlockParams {
    #[schemars(description = "The block hash to look up.")]
    hash: String,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct GetLiquidBlockParams {
    #[schemars(description = "The block hash to look up.")]
    hash: String,
}

// 2. DEFINE YOUR SERVER
// This struct will hold any state your server needs (like API keys, etc.)
// For "Hello World," it's empty.
#[derive(Clone)]
struct MyServer;

// Helper function to create a schema map from a JsonSchema type
fn make_schema<T: schemars::JsonSchema>()
-> Result<std::sync::Arc<rmcp::serde_json::Map<String, rmcp::serde_json::Value>>, ErrorData> {
    use std::sync::Arc;
    let schema = schemars::schema_for!(T);
    let input_schema = rmcp::serde_json::to_value(schema)
        .map_err(|e| ErrorData::internal_error(format!("Failed to serialize schema: {e}"), None))?;
    if let rmcp::serde_json::Value::Object(map) = input_schema {
        Ok(Arc::new(map))
    } else {
        Err(ErrorData::internal_error("Schema is not an object", None))
    }
}

// Fetch data from Esplora API
fn fetch_esplora(url: &str) -> Result<String, String> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| format!("HTTP request failed: {e}"))?;
    response
        .into_string()
        .map_err(|e| format!("Failed to read response: {e}"))
}

fn fetch_transaction(base_url: &str, txid: &str) -> Result<String, String> {
    fetch_esplora(&format!("{base_url}/tx/{txid}"))
}

fn fetch_block(base_url: &str, hash: &str) -> Result<String, String> {
    fetch_esplora(&format!("{base_url}/block/{hash}"))
}

// 3. IMPLEMENT THE TOOL HANDLER
// This is the core of your server. We implement the `ServerHandler` trait.
impl ServerHandler for MyServer {
    // This function lists all available tools that the server provides
    async fn list_tools(
        &self,
        _params: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult {
            tools: vec![
                Tool {
                    name: "get_bitcoin_tx".into(),
                    title: None,
                    description: Some("Get a Bitcoin transaction by its txid from the Esplora API. Returns full transaction data including confirmation status and block height.".into()),
                    input_schema: make_schema::<GetBitcoinTxParams>()?,
                    output_schema: None,
                    annotations: None,
                    icons: None,
                },
                Tool {
                    name: "get_liquid_tx".into(),
                    title: None,
                    description: Some("Get a Liquid transaction by its txid from the Esplora API. Returns full transaction data including confirmation status and block height.".into()),
                    input_schema: make_schema::<GetLiquidTxParams>()?,
                    output_schema: None,
                    annotations: None,
                    icons: None,
                },
                Tool {
                    name: "get_bitcoin_block".into(),
                    title: None,
                    description: Some("Get a Bitcoin block by its hash from the Esplora API. Returns block data including height, timestamp, tx_count, size, and weight.".into()),
                    input_schema: make_schema::<GetBitcoinBlockParams>()?,
                    output_schema: None,
                    annotations: None,
                    icons: None,
                },
                Tool {
                    name: "get_liquid_block".into(),
                    title: None,
                    description: Some("Get a Liquid block by its hash from the Esplora API. Returns block data including height, timestamp, tx_count, size, and weight.".into()),
                    input_schema: make_schema::<GetLiquidBlockParams>()?,
                    output_schema: None,
                    annotations: None,
                    icons: None,
                },
            ],
            next_cursor: None,
        })
    }

    // This function is called when the AI decides to *use* our tool.
    async fn call_tool(
        &self,
        params: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_name = params.name.as_ref();
        let args = params.arguments.unwrap_or_default();
        let args_value = rmcp::serde_json::Value::Object(args);

        // This 'match' is how you handle multiple tools.
        match tool_name {
            "get_bitcoin_tx" => {
                let tx_params: GetBitcoinTxParams = rmcp::serde_json::from_value(args_value)
                    .map_err(|e| {
                        ErrorData::invalid_request(format!("Invalid parameters: {e}"), None)
                    })?;
                let result = fetch_transaction(BITCOIN_API_BASE, &tx_params.txid)
                    .map_err(|e| ErrorData::internal_error(e, None))?;
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            "get_liquid_tx" => {
                let tx_params: GetLiquidTxParams = rmcp::serde_json::from_value(args_value)
                    .map_err(|e| {
                        ErrorData::invalid_request(format!("Invalid parameters: {e}"), None)
                    })?;
                let result = fetch_transaction(LIQUID_API_BASE, &tx_params.txid)
                    .map_err(|e| ErrorData::internal_error(e, None))?;
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            "get_bitcoin_block" => {
                let block_params: GetBitcoinBlockParams = rmcp::serde_json::from_value(args_value)
                    .map_err(|e| {
                        ErrorData::invalid_request(format!("Invalid parameters: {e}"), None)
                    })?;
                let result = fetch_block(BITCOIN_API_BASE, &block_params.hash)
                    .map_err(|e| ErrorData::internal_error(e, None))?;
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            "get_liquid_block" => {
                let block_params: GetLiquidBlockParams = rmcp::serde_json::from_value(args_value)
                    .map_err(|e| {
                    ErrorData::invalid_request(format!("Invalid parameters: {e}"), None)
                })?;
                let result = fetch_block(LIQUID_API_BASE, &block_params.hash)
                    .map_err(|e| ErrorData::internal_error(e, None))?;
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            _ => Err(ErrorData::invalid_request(
                format!("Unknown tool: {tool_name}"),
                None,
            )),
        }
    }

    // This function is called during initialization to set up the server
    async fn initialize(
        &self,
        _params: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, ErrorData> {
        Ok(InitializeResult {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities {
                tools: Some(Default::default()),
                ..Default::default()
            },
            server_info: Implementation {
                name: "Bitcoin Data MCP Server".to_string(),
                title: None,
                version: "0.1.0".to_string(),
                icons: None,
                website_url: None,
            },
            instructions: None,
        })
    }
}

// 4. CREATE THE MAIN FUNCTION TO RUN THE SERVER
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Create an instance of our server
    let server = MyServer;

    // This is the crucial part:
    // 1. 'stdio()' creates the stdio transport.
    // 2. '.serve()' attaches our server logic to the transport.
    // 3. '.waiting()' keeps the server running until it's shut down.
    let running_service = server.serve(stdio()).await?;
    let _quit_reason = running_service.waiting().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rmcp::serde_json;
    use std::io::{BufRead, BufReader, Write};
    use std::process::{Command, Stdio};

    // Run with: cargo test test_mcp_protocol -- --ignored --nocapture
    #[test]
    #[ignore]
    fn test_mcp_protocol_initialize_and_list_tools() {
        // Build the binary first
        let build_result = Command::new("cargo")
            .args(&["build"])
            .output()
            .expect("Failed to build binary");

        assert!(build_result.status.success(), "Build should succeed");

        // Start the MCP server process
        let mut child = Command::new("./target/debug/bitcoin-data-mcp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start MCP server");

        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let mut reader = BufReader::new(stdout);

        // Test 1: Send initialize request
        let initialize_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "test-client",
                    "version": "1.0.0"
                }
            }
        });

        writeln!(stdin, "{}", initialize_request.to_string())
            .expect("Failed to write initialize request");
        stdin.flush().expect("Failed to flush");

        // Read initialize response
        let mut response_line = String::new();
        reader
            .read_line(&mut response_line)
            .expect("Failed to read initialize response");

        println!("Initialize response: {}", response_line);

        let init_response: serde_json::Value =
            serde_json::from_str(&response_line).expect("Failed to parse initialize response");

        assert_eq!(init_response["jsonrpc"], "2.0");
        assert_eq!(init_response["id"], 1);
        assert!(
            init_response["result"].is_object(),
            "Should have result object"
        );
        assert_eq!(
            init_response["result"]["serverInfo"]["name"],
            "Bitcoin Data MCP Server"
        );
        println!("✓ Initialize test passed");

        // Test 2: Send initialized notification
        let initialized_notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        });

        writeln!(stdin, "{}", initialized_notification.to_string())
            .expect("Failed to write initialized notification");
        stdin.flush().expect("Failed to flush");

        // Test 3: Send list_tools request
        let list_tools_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        });

        writeln!(stdin, "{}", list_tools_request.to_string())
            .expect("Failed to write list_tools request");
        stdin.flush().expect("Failed to flush");

        // Read list_tools response
        let mut tools_response_line = String::new();
        reader
            .read_line(&mut tools_response_line)
            .expect("Failed to read list_tools response");

        println!("List tools response: {}", tools_response_line);

        let tools_response: serde_json::Value = serde_json::from_str(&tools_response_line)
            .expect("Failed to parse list_tools response");

        assert_eq!(tools_response["jsonrpc"], "2.0");
        assert_eq!(tools_response["id"], 2);
        assert!(
            tools_response["result"].is_object(),
            "Should have result object"
        );
        assert!(
            tools_response["result"]["tools"].is_array(),
            "Should have tools array"
        );

        let tools = tools_response["result"]["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 4, "Should have exactly 4 tools");

        // Check all tools exist with proper schema
        for tool_name in [
            "get_bitcoin_tx",
            "get_liquid_tx",
            "get_bitcoin_block",
            "get_liquid_block",
        ] {
            let tool = tools
                .iter()
                .find(|t| t["name"] == tool_name)
                .unwrap_or_else(|| panic!("Should have {tool_name} tool"));
            assert!(
                tool["inputSchema"].is_object(),
                "{tool_name} should have inputSchema"
            );
        }

        println!("✓ List tools test passed");
        for tool in tools {
            println!("  Tool: {} - {}", tool["name"], tool["description"]);
        }

        // Clean up
        child.kill().expect("Failed to kill child process");
    }
}
