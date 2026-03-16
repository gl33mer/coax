# Coax Security Scanner for VS Code

**Version:** 0.8.0  
**Marketplace:** [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=gl33mer.coax)

Real-time Unicode confusable character and secret detection integrated directly into VS Code.

## Features

- 🔒 **Secret Detection** - AWS keys, GitHub tokens, API keys, private keys
- 🔤 **Unicode Attack Detection** - Homoglyphs, invisible characters, bidirectional overrides
- ⚡ **Real-time Scanning** - Scan on save with 500ms debounce
- 🎯 **Inline Warnings** - Squiggly underlines with severity colors
- 📋 **Problems Panel** - All findings in VS Code Problems panel
- 💡 **Quick Fixes** - Remove secrets, replace with environment variables
- 📊 **Status Bar** - Finding count and scan status indicator

## Installation

1. Open VS Code
2. Press `Ctrl+Shift+X` (Extensions)
3. Search for "Coax"
4. Click Install

Or install from VSIX:
```bash
code --install-extension coax-0.8.0.vsix
```

## Usage

### Automatic Scanning

- **On Save:** Files are automatically scanned when saved (default: enabled)
- **On Open:** Files are scanned when opened (default: enabled)

### Manual Scanning

Open Command Palette (`Ctrl+Shift+P`) and run:
- `Coax: Scan Current File` - Scan the active file
- `Coax: Scan Workspace` - Scan entire workspace
- `Coax: Show Findings` - View all findings in Problems panel
- `Coax: Clear Findings` - Clear all diagnostics

### Quick Fixes

Click the lightbulb (💡) next to a finding to see available actions:
- Remove the finding
- Replace with environment variable
- Ignore for this session

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `coax.enabled` | `true` | Enable/disable scanning |
| `coax.scanOnSave` | `true` | Scan files on save |
| `coax.scanOnOpen` | `true` | Scan files on open |
| `coax.unicode.enabled` | `true` | Enable Unicode detection |
| `coax.unicode.sensitivity` | `high` | Unicode sensitivity (low/medium/high/critical) |
| `coax.severityThreshold` | `medium` | Minimum severity to display |
| `coax.exclude` | `**/node_modules/**`, etc. | File patterns to exclude |
| `coax.maxFileSize` | `10485760` | Max file size (10MB) |
| `coax.scanTimeout` | `30000` | Scan timeout (30s) |
| `coax.debounceDelay` | `500` | Debounce delay for scan-on-save |

## Severity Colors

| Severity | Color | VS Code Level |
|----------|-------|---------------|
| Critical | 🔴 Red | Error |
| High | 🟠 Orange | Error |
| Medium | 🟡 Yellow | Warning |
| Low | 🟢 Green | Information |

## Requirements

- VS Code >= 1.85.0
- No external dependencies (Coax CLI bundled)

## Supported Platforms

- Windows x64
- macOS x64 (Intel)
- macOS arm64 (Apple Silicon)
- Linux x64
- Linux arm64

## Troubleshooting

### "Coax binary not found"

The extension bundles the Coax CLI for all platforms. If you see this error:

1. Check that the extension installed correctly
2. Try setting a custom binary path: `coax.binaryPath`
3. Build Coax CLI from source: `cargo build --release`

### No findings showing

1. Check `coax.enabled` is `true`
2. Check `coax.severityThreshold` isn't too restrictive
3. Check file isn't in `coax.exclude` patterns
4. Check Output panel for errors

### Performance issues

1. Increase `coax.debounceDelay` (default: 500ms)
2. Reduce `coax.maxFileSize`
3. Add more patterns to `coax.exclude`

## Development

```bash
# Install dependencies
npm install

# Compile
npm run compile

# Watch mode
npm run watch

# Package
npm run package
```

## License

MIT License - see [LICENSE](LICENSE)

## Repository

https://github.com/gl33mer/coax

## Support

- **Issues:** https://github.com/gl33mer/coax/issues
- **Documentation:** https://github.com/gl33mer/coax/tree/main/docs
