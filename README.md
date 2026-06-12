# h41

A lightweight tool that discovers all listening TCP ports on your machine and serves a clean web UI to browse, filter, and manage them.

## Features

- 🔍 **Discover** all listening TCP ports via `lsof`
- 🌐 **Web UI** with sortable table showing PID, working directory, command, and addresses
- 🔎 **Live filtering** by path, process name, port, or command args
- 🏠 **Smart filtering** hides system processes by default (toggle to show all)
- 📋 **Copy to clipboard** on any cell value (command copies full args)
- ⓘ **Args tooltip** shows full command arguments on hover
- 🔄 **Auto-refresh** with configurable interval (30s / 1min / 5min / 15min)
- 💀 **Kill processes** directly from the UI with confirmation
- 📦 **JSON output** mode for scripting (`--json`)
- 🔒 **Secure by default** — binds to localhost only

## Installation

### Homebrew

```bash
brew tap 9h41/h41 https://github.com/9h41/h41
brew install h41
```

### From source

```bash
cargo install --path .
```

## Usage

```bash
# Start the web UI (default: http://localhost:8941)
h41

# Use a custom port
h41 --port 8080

# Output JSON to stdout (no server)
h41 --json
```

## Requirements

- macOS or Linux with `lsof` available

## License

MIT
