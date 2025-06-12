# {{project-name}}

{{description}}

A command-line MCP (Model Context Protocol) client that can connect to MCP servers and interact with their tools, resources, and prompts.

## Quick Start

### Prerequisites

- Rust 1.75 or higher
- Cargo
- An MCP server to connect to

### Installation

1. Clone this repository:
```bash
git clone https://github.com/{{github-username}}/{{project-name}}.git
cd {{project-name}}
```

2. Build the project:
```bash
cargo build --release
```

3. Run the client:
```bash
./target/release/{{project-name}} --help
```

## Usage

### Basic Commands

List available tools from a server:
```bash
./target/release/{{project-name}} --server ./path/to/mcp-server list-tools
```

Call a tool:
```bash
./target/release/{{project-name}} --server ./path/to/mcp-server call-tool echo --args '{"message": "Hello World"}'
```

List resources:
```bash
./target/release/{{project-name}} --server ./path/to/mcp-server list-resources
```

Read a resource:
```bash
./target/release/{{project-name}} --server ./path/to/mcp-server read-resource "file:///path/to/file.txt"
```

### Interactive Mode

Start an interactive session:
```bash
./target/release/{{project-name}} --server ./path/to/mcp-server interactive
```

In interactive mode, you can use these commands:
- `tools` - List available tools
- `resources` - List available resources  
- `prompts` - List available prompts
- `call <tool> [args]` - Call a tool
- `read <uri>` - Read a resource
- `prompt <n> [args]` - Get a prompt
- `help` - Show help
- `exit` - Exit interactive mode

### Examples

#### Working with File Servers

```bash
# List available files
./{{project-name}} --server ./file-server list-resources

# Read a specific file
./{{project-name}} --server ./file-server read-resource "file:///documents/readme.txt"
```

#### Working with Database Servers

```bash
# List available database tools
./{{project-name}} --server ./db-server list-tools

# Execute a query
./{{project-name}} --server ./db-server call-tool query --args '{"sql": "SELECT * FROM users"}'
```

#### Working with API Servers

```bash
# List API tools
./{{project-name}} --server ./api-server list-tools

# Make an API call
./{{project-name}} --server ./api-server call-tool http-get --args '{"url": "https://api.example.com/data"}'
```

## Configuration

### Command Line Options

- `--server <path>` - Path to the MCP server executable (default: `./server`)
- `--verbose` - Enable verbose logging
- `--help` - Show help information
- `--version` - Show version information

### Environment Variables

- `RUST_LOG` - Set logging level (e.g., `RUST_LOG=debug`)

### Transport Types

The client supports different transport methods for connecting to servers:

#### STDIO (Default)
Connect to servers that use STDIO transport (most common):
```bash
./{{project-name}} --server ./path/to/server list-tools
```

#### HTTP (Feature: http)
For servers with HTTP transport:
```bash
# Build with HTTP support
cargo build --features http --release

# Use HTTP transport (modify source code to use HttpClientTransport)
```

#### WebSocket (Feature: websocket)
For servers with WebSocket transport:
```bash
# Build with WebSocket support  
cargo build --features websocket --release

# Use WebSocket transport (modify source code to use WebSocketClientTransport)
```

## Development

### Adding New Commands

1. Add a new variant to the `Commands` enum:

```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing commands
    MyNewCommand {
        /// Command description
        parameter: String,
    },
}
```

2. Handle the command in the main match statement:

```rust
match cli.command {
    // ... existing handlers
    Commands::MyNewCommand { parameter } => my_new_command(&session, &parameter).await?,
}
```

3. Implement the command function:

```rust
async fn my_new_command(session: &ClientSession, parameter: &str) -> Result<()> {
    // Your command implementation
    Ok(())
}
```

### Testing

Run the test suite:
```bash
cargo test
```

Test with a real server:
```bash
# Start a test server in one terminal
cargo run --example echo_server

# Test the client in another terminal
./target/release/{{project-name}} --server ./target/debug/examples/echo_server list-tools
```

### Building for Different Transports

```bash
# STDIO only (minimal build)
cargo build --no-default-features --features stdio

# With HTTP support
cargo build --features http

# With WebSocket support
cargo build --features websocket

# All features
cargo build --all-features
```

## Troubleshooting

### Connection Issues

1. **Server not found**: Ensure the server path is correct and the file is executable
2. **Permission denied**: Make sure the server binary has execute permissions
3. **Protocol errors**: Check that the server implements the MCP protocol correctly

### Common Error Messages

- `Failed to connect`: Server binary not found or not executable
- `Protocol error`: Server doesn't implement MCP correctly
- `Tool not found`: The requested tool name doesn't exist on the server
- `Invalid arguments`: Tool arguments don't match the expected schema

### Debug Mode

Enable verbose logging to see detailed protocol messages:
```bash
./{{project-name}} --verbose --server ./server list-tools
```

Or set the environment variable:
```bash
RUST_LOG=debug ./{{project-name}} --server ./server list-tools
```

## Integration Examples

### Shell Scripts

Create a shell function for easy access:

```bash
# Add to ~/.bashrc or ~/.zshrc
mcp-call() {
    local server="$1"
    local tool="$2"
    local args="${3:-{}}"
    ./{{project-name}} --server "$server" call-tool "$tool" --args "$args"
}

# Usage
mcp-call ./file-server read-file '{"path": "/tmp/test.txt"}'
```

### CI/CD Integration

Use in GitHub Actions:

```yaml
- name: Test MCP Server
  run: |
    ./mcp-server &
    SERVER_PID=$!
    sleep 2
    ./{{project-name}} --server ./mcp-server list-tools
    kill $SERVER_PID
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run `cargo test` and `cargo clippy`
6. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Resources

- [MCP Specification](https://modelcontextprotocol.io/)
- [MCP Rust SDK Documentation](https://docs.rs/mcp-protocol-sdk)
- [Claude Desktop MCP Guide](https://docs.anthropic.com/claude/docs/mcp)

## Support

- Create an issue for bug reports or feature requests
- Check existing issues before creating new ones
- Provide minimal reproduction examples for bugs
