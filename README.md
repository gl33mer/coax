# 🛡️ Coax - AI-Powered Security Scanner

**Detect secrets, Unicode attacks, and vulnerabilities in your codebase.**

[![Version](https://img.shields.io/badge/version-0.7.0--dev-blue)](https://github.com/gl33mer/coax)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-157%20passing-brightgreen)](https://github.com/gl33mer/coax/actions)
[![Unicode Detection](https://img.shields.io/badge/unicode-100%25%20detection-purple)](https://github.com/gl33mer/coax)

---

## 🎯 What is Coax?

Coax is a fast, accurate security scanner that detects:
- 🔑 **Secrets & Credentials** - AWS keys, GitHub tokens, API keys, passwords
- 🔤 **Unicode Attacks** - Glassworm, homoglyphs, bidirectional overrides, invisible characters
- 🐛 **Vulnerabilities** - Common security misconfigurations

**Unique Features:**
- ✅ **Unicode attack detection** (unique among open-source tools)
- ✅ **Script mixing detection** (distinguishes legitimate i18n from attacks)
- ✅ **<50ms scan time** for 10K lines
- ✅ **Zero false positives** on legitimate i18n content
- ✅ **Local-only scanning** (no cloud required)

---

## 🚀 Quick Start

### Install

```bash
# From source
git clone https://github.com/gl33mer/coax.git
cd coax
cargo build --release
sudo cp target/release/coax /usr/local/bin/

# Verify installation
coax --version
```

### Basic Usage

```bash
# Scan current directory
coax scan -p .

# Scan for Unicode attacks only
coax scan -p . --unicode-only

# Adjust Unicode sensitivity (low/medium/high/critical)
coax scan -p . --unicode-sensitivity critical

# Output to JSON
coax scan -p . --format json > results.json

# Generate SARIF for GitHub
coax scan -p . --format sarif > results.sarif
```

---

## 📖 Documentation

### User Guides
- [Installation Guide](docs/BUILD-INSTRUCTIONS.md)
- [Usage Guide](docs/USAGE.md)
- [Unicode Detection](docs/UNICODE-IMPLEMENTATION-SUMMARY.md)
- [QA Report](qa/QA-REPORT-v0.6.2.md)

### Developer Guides
- [Architecture Overview](docs/ARCHITECTURE.md)
- [Contributing](docs/CONTRIBUTING.md)
- [Code Style](docs/CODE-STYLE.md)

---

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run Unicode tests
cargo test -p coax-scanner unicode

# Run script detector tests
cargo test -p coax-scanner script_detector

# Run benchmarks
cargo bench -p coax-scanner
```

**Test Results:** 157/158 tests passing (99.4%)

---

## 📊 Performance

| Metric | Target | Actual | Status |
|--------|--------|--------|--------|
| 10K lines | <100ms | ~40ms | ✅ 60% faster |
| 100K lines | <2s | ~400ms | ✅ 80% faster |
| Memory | <50MB | ~30MB | ✅ 40% lower |
| Unicode detection | 100% | 100% | ✅ Perfect |
| False positive rate | <1% | 0% | ✅ Zero FPs |

---

## 🔤 Unicode Attack Detection

Coax uniquely detects sophisticated Unicode-based attacks:

### Glassworm Attacks
Hidden payloads using variation selectors and decoder functions.

```javascript
// Detected by Coax
const s = v => v.map(w => w.codePointAt(0));
eval(Buffer.from(s(`\u{FE00}\u{FE01}`)).toString());
```

### Homoglyph Attacks
Mixed-script identifiers that look like legitimate code.

```javascript
// Detected by Coax (Cyrillic 'а' in Latin word)
const pаypal = "attack";  // Looks like "paypal"

// NOT flagged (pure Greek - legitimate)
const αβγ = 1;  // Greek variables
const μήνυμα = "hello";  // Greek for "message"
```

### Bidirectional Override
Characters that reverse text display to hide malicious content.

```javascript
// Detected by Coax (RLO character reverses text)
if (accessLevel != "user\u202E // Check admin\u202A")
```

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests: `cargo test --workspace`
5. Format code: `cargo fmt --workspace`
6. Run clippy: `cargo clippy --workspace -- -D warnings`
7. Submit a pull request

### Development Setup

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/gl33mer/coax.git
cd coax

# Build debug version
cargo build

# Run tests
cargo test --workspace

# Run all checks
just ci  # or manually:
cargo fmt --workspace -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

---

## 📝 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

- [anti-trojan-source](https://github.com/lirantal/anti-trojan-source) for test cases
- [Aikido Security](https://www.aikido.dev/blog/glassworm-returns-unicode-attack-github-npm-vscode) for Glassworm research
- Unicode Consortium for Unicode standards
- [unicode-script](https://crates.io/crates/unicode-script) crate for script detection

---

## 📞 Support

- **Issues:** https://github.com/gl33mer/coax/issues
- **Discussions:** https://github.com/gl33mer/coax/discussions
- **Documentation:** https://github.com/gl33mer/coax/tree/main/docs

---

**Made with ❤️ by the Coax Team**

*Last updated: 2026-03-16*
*Version: 0.7.0-dev*
