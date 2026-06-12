# h41

A lightweight tool that discovers all listening TCP ports on your machine with a TUI and optional web interface.

## Features

- 🔍 **Discover** all listening TCP ports via `lsof`
- 🖥️ **TUI** (default) — interactive terminal interface with vim-like navigation
- 🌐 **Web UI** — optional browser-based interface (`--web`)
- 🔎 **Live filtering** by path, process name, port, or command args
- 🏠 **Smart filtering** hides system processes by default (toggle to show all)
- 💀 **Kill processes** with confirmation
- 🔗 **Open in browser** — launch a port's URL directly
- 📦 **JSON output** mode for scripting (`--json`)
- 🔒 **Secure by default** — web server binds to localhost only

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
# Start the TUI (default)
h41

# Start the web UI on port 8941
h41 --web

# Use a custom port for the web UI
h41 --web --port 8080

# Output JSON to stdout
h41 --json
```

## TUI Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `o` / `Enter` | Open in browser |
| `x` | Kill process (with confirmation) |
| `/` | Filter (type to search, Esc to close) |
| `a` | Toggle show all / user-only |
| `r` | Refresh |
| `q` / `Esc` | Quit |

## Requirements

- macOS or Linux with `lsof` available

## License

MIT
