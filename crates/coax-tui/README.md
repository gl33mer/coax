# Coax TUI

Terminal User Interface (TUI) dashboard for the Coax security scanner.

## Features

- 🛡️ **Dashboard View**: Overview of scan results with severity statistics
- 📋 **Finding List**: Scrollable list with filtering and sorting
- 🔍 **Finding Detail**: Detailed view with code preview and recommendations
- ⚙️ **Settings**: View and manage current configuration
- ⌨️ **Full Keyboard Navigation**: Vim-style keybindings for efficient navigation

## Installation

The TUI is included with the `coax` CLI tool. Build it with:

```bash
cd /home/shva/QwenDev/coax
cargo build --release
```

## Usage

### Launch TUI with auto-scan

```bash
coax tui
```

This will automatically scan the current directory on startup.

### Scan specific path

```bash
coax tui -p /path/to/project
```

### Launch without auto-scan

```bash
coax tui --no-auto-scan
```

You can then manually trigger a scan by pressing `R`.

## Key Bindings

### Global

| Key | Action |
|-----|--------|
| `q` / `Q` | Quit application |
| `Ctrl+C` | Quit application |
| `?` | Toggle help popup |

### Navigation

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `Enter` | Select / View detail |
| `←` / `h` | Go back |

### Dashboard View

| Key | Action |
|-----|--------|
| `R` | Rescan |
| `L` | Go to list view |
| `S` | Go to settings |
| `/` | Search |

### List View

| Key | Action |
|-----|--------|
| `1` | Show all findings |
| `2` | Filter: Critical |
| `3` | Filter: High |
| `4` | Filter: Medium |
| `5` | Filter: Low |
| `s` | Sort by severity |
| `f` | Sort by file |
| `l` | Sort by line |
| `p` | Sort by pattern |
| `D` | Go to dashboard |
| `/` | Search |
| `R` | Rescan |

### Detail View

| Key | Action |
|-----|--------|
| `←` / `h` / `Esc` | Go back |
| `I` | Ignore finding |
| `F` | Mark as false positive |
| `R` | Show rotate instructions |
| `C` | Clear status |

### Search Mode

| Key | Action |
|-----|--------|
| `/` | Enter search mode |
| `Esc` | Exit search / Clear search |
| `Backspace` | Delete character |
| Any other key | Add to search query |

## Views

### Dashboard

The dashboard provides an overview of your scan results:

```
╔══════════════════════════════════════════════════════════╗
║  🛡️  Coax Security Dashboard                  [q] Quit  ║
╠══════════════════════════════════════════════════════════╣
║  Repository: my-project                                  ║
║  Last Scan: 2026-03-15 16:00 (2 min ago)                ║
║                                                          ║
║  ┌─────────────────────────────────────────────────┐    ║
║  │  Scan Results                                   │    ║
║  │  ┌──────────┬──────────┬──────────┬──────────┐ │    ║
║  │  │ Critical │   High   │  Medium  │   Low    │ │    ║
║  │  ├──────────┼──────────┼──────────┼──────────┤ │    ║
║  │  │    3     │    7     │    12    │    5     │ │    ║
║  │  └──────────┴──────────┴──────────┴──────────┘ │    ║
║  └─────────────────────────────────────────────────┘    ║
║                                                          ║
║  ┌─────────────────────────────────────────────────┐    ║
║  │  Recent Findings                                │    ║
║  │  🚨 AWS_ACCESS_KEY      config.yml:45   [View] │    ║
║  │  🚨 GITHUB_PAT          .env:12         [View] │    ║
║  │  ⚠️  GENERIC_SECRET     src/utils.py:89 [View] │    ║
║  └─────────────────────────────────────────────────┘    ║
║                                                          ║
║  [↑↓] Navigate  [Enter] View  [/] Search  [R] Rescan   ║
╚══════════════════════════════════════════════════════════╝
```

### Finding List

The list view shows all findings with filtering and sorting:

```
╔══════════════════════════════════════════════════════════╗
║  📋  Finding List - 27 findings (showing 5)              ║
╠══════════════════════════════════════════════════════════╣
║  [All]|[Critical]|[High]|[Medium]|[Low]                  ║
║  Sort by: Severity ↓  |  Keys: s=Severity  f=File...    ║
║                                                          ║
║  ┌──────────────────────────────────────────────────┐   ║
║  │  Pattern              File              Line     │   ║
║  ├──────────────────────────────────────────────────┤   ║
║  │▶ 🚨 AWS_ACCESS_KEY    config.yml        45       │   ║
║  │  ⚠️  GITHUB_TOKEN     .env              12       │   ║
║  │  ⚡ GENERIC_SECRET    src/utils.py      89       │   ║
║  └──────────────────────────────────────────────────┘   ║
║                                                          ║
║  [↑↓] Navigate  [Enter] View  [/] Search  [←] Back     ║
╚══════════════════════════════════════════════════════════╝
```

### Finding Detail

The detail view shows comprehensive information about a finding:

```
╔══════════════════════════════════════════════════════════╗
║  Finding Detail                               [←] Back  ║
╠══════════════════════════════════════════════════════════╣
║  🚨 AWS_ACCESS_KEY (Critical)                            ║
║                                                          ║
║  Location: config.yml:45:1                               ║
║  Confidence: 99%                                         ║
║                                                          ║
║  ┌──────────────────────────────────────────────────┐   ║
║  │  Code Preview                                    │   ║
║  │  44: # AWS Configuration                        │   ║
║  │  45: aws_access_key_id = AKIAIOSFODNN7EXAMPLE  │ ← ║
║  │  46: aws_secret_key = wJalrXUtnFEMI...          │   ║
║  │  47: region = us-east-1                         │   ║
║  └──────────────────────────────────────────────────┘   ║
║                                                          ║
║  Recommendation: Remove immediately and rotate the key  ║
║  via AWS IAM Console.                                   ║
║                                                          ║
║  [←] Back  |  [I] Ignore  |  [F] False Positive  | ... ║
╚══════════════════════════════════════════════════════════╝
```

## Architecture

```
coax-tui/
├── src/
│   ├── lib.rs           # Module exports
│   ├── app.rs           # Main application state
│   ├── ui.rs            # UI rendering
│   ├── views/
│   │   ├── mod.rs
│   │   ├── dashboard.rs     # Main dashboard view
│   │   ├── finding_list.rs  # Finding list view
│   │   ├── finding_detail.rs # Finding detail view
│   │   └── settings.rs      # Settings panel
│   ├── components/
│   │   ├── mod.rs
│   │   ├── header.rs        # Header component
│   │   └── footer.rs        # Footer with keybindings
│   └── events/
│       ├── mod.rs
│       └── handler.rs       # Keyboard event handler
├── tests/
│   └── integration_tests.rs
└── README.md
```

## Testing

Run tests with:

```bash
cargo test -p coax-tui
```

## Development

### Adding a new view

1. Create the view module in `src/views/`
2. Add the view to the `View` enum in `app.rs`
3. Implement the render function
4. Add the view to the match statement in `ui.rs::render_main_content()`
5. Add keybindings in `events/handler.rs`

### Adding a new keybinding

1. Find the appropriate view handler in `events/handler.rs`
2. Add the keybinding to the match statement
3. Update this README with the new binding

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please read the main [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.
