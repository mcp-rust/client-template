//! {{project-name}}
//!
//! {{description}}

use anyhow::Result;
use clap::{Parser, Subcommand};
use mcp_protocol_sdk::{
    client::{ClientSession, McpClient},
    transport::stdio::StdioClientTransport,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::{error, info};

#[derive(Parser)]
#[command(name = "{{project-name}}")]
#[command(about = "{{description}}")]
#[command(version)]
struct Cli {
    /// Server command to execute
    #[arg(short, long, default_value = "./server")]
    server: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available tools from the server
    ListTools,
    /// List available resources from the server
    ListResources,
    /// List available prompts from the server
    ListPrompts,
    /// Call a tool with the given arguments
    CallTool {
        /// Tool name to call
        tool: String,
        /// JSON arguments for the tool
        #[arg(short, long, default_value = "{}")]
        args: String,
    },
    /// Read a resource
    ReadResource {
        /// Resource URI to read
        uri: String,
    },
    /// Get a prompt
    GetPrompt {
        /// Prompt name to get
        name: String,
        /// JSON arguments for the prompt
        #[arg(short, long, default_value = "{}")]
        args: String,
    },
    /// Interactive mode
    Interactive,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("{{project-name}}={},mcp_protocol_sdk=info", log_level))
        .init();

    info!("Starting {{project-name}} MCP client...");

    // Create client and session
    let client = McpClient::new("{{project-name}}".to_string(), "0.1.0".to_string());
    let session = ClientSession::new(client);

    // Connect to server
    info!("Connecting to server: {}", cli.server);
    let transport = StdioClientTransport::new(cli.server).await?;
    let init_result = session.connect(transport).await?;

    info!(
        "Connected to server: {} v{}",
        init_result.server_info.name, init_result.server_info.version
    );

    // Execute command
    match cli.command {
        Commands::ListTools => list_tools(&session).await?,
        Commands::ListResources => list_resources(&session).await?,
        Commands::ListPrompts => list_prompts(&session).await?,
        Commands::CallTool { tool, args } => call_tool(&session, &tool, &args).await?,
        Commands::ReadResource { uri } => read_resource(&session, &uri).await?,
        Commands::GetPrompt { name, args } => get_prompt(&session, &name, &args).await?,
        Commands::Interactive => interactive_mode(&session).await?,
    }

    info!("{{project-name}} client finished");
    Ok(())
}

async fn list_tools(session: &ClientSession) -> Result<()> {
    let client = session.client();
    let client_guard = client.lock().await;

    info!("Listing available tools...");
    let tools = client_guard.list_tools().await?;

    if tools.tools.is_empty() {
        println!("No tools available");
    } else {
        println!("Available tools:");
        for tool in tools.tools {
            println!("  - {}: {}", tool.name, tool.description.unwrap_or_default());
        }
    }

    Ok(())
}

async fn list_resources(session: &ClientSession) -> Result<()> {
    let client = session.client();
    let client_guard = client.lock().await;

    info!("Listing available resources...");
    let resources = client_guard.list_resources().await?;

    if resources.resources.is_empty() {
        println!("No resources available");
    } else {
        println!("Available resources:");
        for resource in resources.resources {
            println!("  - {}: {}", resource.uri, resource.description.unwrap_or_default());
        }
    }

    Ok(())
}

async fn list_prompts(session: &ClientSession) -> Result<()> {
    let client = session.client();
    let client_guard = client.lock().await;

    info!("Listing available prompts...");
    let prompts = client_guard.list_prompts().await?;

    if prompts.prompts.is_empty() {
        println!("No prompts available");
    } else {
        println!("Available prompts:");
        for prompt in prompts.prompts {
            println!("  - {}: {}", prompt.name, prompt.description.unwrap_or_default());
        }
    }

    Ok(())
}

async fn call_tool(session: &ClientSession, tool_name: &str, args_json: &str) -> Result<()> {
    let client = session.client();
    let client_guard = client.lock().await;

    info!("Calling tool: {} with args: {}", tool_name, args_json);

    // Parse arguments
    let args: HashMap<String, Value> = if args_json.trim().is_empty() || args_json == "{}" {
        HashMap::new()
    } else {
        serde_json::from_str(args_json)?
    };

    let result = client_guard
        .call_tool(tool_name.to_string(), if args.is_empty() { None } else { Some(args) })
        .await?;

    println!("Tool result:");
    for content in result.content {
        match content {
            mcp_protocol_sdk::protocol::types::Content::Text { text } => {
                println!("  Text: {}", text);
            }
            mcp_protocol_sdk::protocol::types::Content::Image { data, mime_type } => {
                println!("  Image: {} bytes, type: {}", data.len(), mime_type);
            }
            mcp_protocol_sdk::protocol::types::Content::Resource { .. } => {
                println!("  Resource content");
            }
        }
    }

    if let Some(is_error) = result.is_error {
        if is_error {
            error!("Tool returned an error");
        }
    }

    Ok(())
}

async fn read_resource(session: &ClientSession, uri: &str) -> Result<()> {
    let client = session.client();
    let client_guard = client.lock().await;

    info!("Reading resource: {}", uri);

    let result = client_guard
        .read_resource(uri.to_string(), None)
        .await?;

    println!("Resource content:");
    for content in result.contents {
        println!("  URI: {}", content.uri);
        if let Some(mime_type) = content.mime_type {
            println!("  MIME type: {}", mime_type);
        }
        if let Some(text) = content.text {
            println!("  Text content: {}", text);
        }
        if let Some(blob) = content.blob {
            println!("  Binary content: {} bytes", blob.len());
        }
    }

    Ok(())
}

async fn get_prompt(session: &ClientSession, prompt_name: &str, args_json: &str) -> Result<()> {
    let client = session.client();
    let client_guard = client.lock().await;

    info!("Getting prompt: {} with args: {}", prompt_name, args_json);

    // Parse arguments
    let args: HashMap<String, Value> = if args_json.trim().is_empty() || args_json == "{}" {
        HashMap::new()
    } else {
        serde_json::from_str(args_json)?
    };

    let result = client_guard
        .get_prompt(prompt_name.to_string(), if args.is_empty() { None } else { Some(args) })
        .await?;

    println!("Prompt result:");
    if let Some(description) = result.description {
        println!("  Description: {}", description);
    }

    for message in result.messages {
        println!("  {} role: {}", message.role, 
            match message.content {
                mcp_protocol_sdk::protocol::types::PromptContent::Text { text } => text,
                mcp_protocol_sdk::protocol::types::PromptContent::Image { .. } => "[Image content]".to_string(),
                mcp_protocol_sdk::protocol::types::PromptContent::Resource { .. } => "[Resource content]".to_string(),
            }
        );
    }

    Ok(())
}

async fn interactive_mode(session: &ClientSession) -> Result<()> {
    println!("Entering interactive mode. Type 'help' for commands, 'exit' to quit.");

    loop {
        print!("> ");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            break;
        }

        if input == "help" {
            println!("Available commands:");
            println!("  tools - List available tools");
            println!("  resources - List available resources");
            println!("  prompts - List available prompts");
            println!("  call <tool> [args] - Call a tool");
            println!("  read <uri> - Read a resource");
            println!("  prompt <name> [args] - Get a prompt");
            println!("  help - Show this help");
            println!("  exit - Exit interactive mode");
            continue;
        }

        let parts: Vec<&str> = input.splitn(3, ' ').collect();
        let command = parts[0];

        match command {
            "tools" => {
                if let Err(e) = list_tools(session).await {
                    error!("Error listing tools: {}", e);
                }
            }
            "resources" => {
                if let Err(e) = list_resources(session).await {
                    error!("Error listing resources: {}", e);
                }
            }
            "prompts" => {
                if let Err(e) = list_prompts(session).await {
                    error!("Error listing prompts: {}", e);
                }
            }
            "call" => {
                if parts.len() < 2 {
                    println!("Usage: call <tool> [args]");
                } else {
                    let tool = parts[1];
                    let args = parts.get(2).unwrap_or("{}");
                    if let Err(e) = call_tool(session, tool, args).await {
                        error!("Error calling tool: {}", e);
                    }
                }
            }
            "read" => {
                if parts.len() < 2 {
                    println!("Usage: read <uri>");
                } else {
                    let uri = parts[1];
                    if let Err(e) = read_resource(session, uri).await {
                        error!("Error reading resource: {}", e);
                    }
                }
            }
            "prompt" => {
                if parts.len() < 2 {
                    println!("Usage: prompt <name> [args]");
                } else {
                    let name = parts[1];
                    let args = parts.get(2).unwrap_or("{}");
                    if let Err(e) = get_prompt(session, name, args).await {
                        error!("Error getting prompt: {}", e);
                    }
                }
            }
            _ => {
                println!("Unknown command: {}. Type 'help' for available commands.", command);
            }
        }
    }

    println!("Exiting interactive mode");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::try_parse_from(&["test", "list-tools"]).unwrap();
        assert!(!cli.verbose);
        assert_eq!(cli.server, "./server");
        assert!(matches!(cli.command, Commands::ListTools));
    }

    #[test]
    fn test_args_parsing() {
        let args_json = r#"{"key": "value", "number": 42}"#;
        let args: HashMap<String, Value> = serde_json::from_str(args_json).unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(args.get("key").unwrap().as_str().unwrap(), "value");
        assert_eq!(args.get("number").unwrap().as_i64().unwrap(), 42);
    }
}
