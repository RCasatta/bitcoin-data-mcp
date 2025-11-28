# bitcoin-data-mcp

A lightweight [MCP (Model Context Protocol)](https://modelcontextprotocol.io/) server that gives AI assistants real-time access to Bitcoin and Liquid blockchain data via the [Esplora API](https://blockstream.info).

Built with Rust 🦀 and [rmcp](https://github.com/anthropics/mcp-rs).

## Features

Query blockchain data directly from your AI assistant:

| Tool | Description |
|------|-------------|
| `get_bitcoin_tx` | Fetch Bitcoin transaction by txid |
| `get_liquid_tx` | Fetch Liquid transaction by txid |
| `get_bitcoin_block` | Fetch Bitcoin block by hash |
| `get_liquid_block` | Fetch Liquid block by hash |
| `get_bitcoin_tip_height` | Get current Bitcoin chain height |
| `get_liquid_tip_height` | Get current Liquid chain height |
| `get_bitcoin_mempool` | Get Bitcoin mempool statistics |
| `get_liquid_mempool` | Get Liquid mempool statistics |

## Installation

### Cursor / VS Code (with Nix)

If you have Nix installed, setup is trivial. Add this to your MCP configuration:

```json
{
  "mcpServers": {
    "bitcoin-data-mcp": {
      "command": "nix",
      "args": [
        "run",
        "github:RCasatta/bitcoin-data-mcp?rev=03d414992246941296931b5b4f1622bf722e057e"
      ]
    }
  }
}
```

That's it. No build steps, no dependency management, no version conflicts.

The `?rev=` parameter pins to a specific commit for reproducibility.

## Usage Examples

Once configured, you can ask your AI assistant things like:

- *"What's the current Bitcoin block height?"*
- *"Show me the details of transaction `abc123...`"*
- *"How many transactions are in the Bitcoin mempool right now?"*
- *"Get block `000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f`"*

## Data Source

All data is fetched from [blockstream.info](https://blockstream.info) Esplora API:
- Bitcoin: `https://blockstream.info/api`
- Liquid: `https://blockstream.info/liquid/api`

## License

MIT

