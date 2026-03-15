# Coax - Development Guide

**Version:** 0.2.0  
**Last Updated:** 2026-03-14  
**Status:** Active Development  

Complete setup guide for developers cloning the Coax repository on a fresh machine.

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Prerequisites](#prerequisites)
3. [Installation Steps](#installation-steps)
4. [Development Workflow](#development-workflow)
5. [Troubleshooting](#troubleshooting)
6. [Project Structure](#project-structure)

---

## Quick Start

**For experienced developers who just want to get running.**

### Minimum Requirements

- **Rust:** 1.75.0 or later (stable)
- **OS:** Linux (Ubuntu 20.04+), macOS (12+), or Windows 11 with WSL2
- **Disk:** 500MB for toolchain + 100MB for repository
- **RAM:** 2GB minimum, 4GB recommended for builds

### 3-Command Setup

```bash
# 1. Clone the repository
git clone https://github.com/gl33mer/coax.git
cd coax

# 2. Build the workspace (release mode for best performance)
cargo build --workspace --release

# 3. Run the test suite
cargo test --workspace
```

### Verify Installation

```bash
# Check that all crates build successfully
cargo check --workspace

# Run the CLI to verify it works
cargo run --bin opendev-cli -- --help

# Expected output:
# Coax Security Scanner v0.2.0
# Usage: opendev <COMMAND>
#
# Commands:
#   scan      Scan for secrets and vulnerabilities
#   report    Generate security reports
#   help      Print this message
```

**If all commands succeed, you're ready to develop!**

---

## Prerequisites

### Rust Toolchain

**Required Version:** Rust 1.75.0 or later

#### Install Rust (Linux/macOS)

```bash
# Install rustup (Rust installer and version manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts, then reload your shell
source $HOME/.cargo/env

# Verify installation
rustc --version  # Should show: rustc 1.75.0 or later
cargo --version  # Should show: cargo 1.75.0 or later
```

#### Install Rust (Windows)

```powershell
# Download and run rustup-init.exe from:
# https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

# Or use winget (Windows Package Manager)
winget install Rustlang.Rustup

# Restart your terminal, then verify
rustc --version
cargo --version
```

#### Install Rust (WSL2)

```bash
# Inside WSL2 terminal, use the Linux installation method
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### System Dependencies

#### Linux (Ubuntu/Debian)

```bash
# Update package lists
sudo apt update

# Install build essentials and SSL development libraries
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    cmake \
    git \
    curl

# For tree-sitter parsing (Phase 2)
sudo apt install -y \
    libclang-dev \
    llvm-dev
```

#### Linux (Fedora/RHEL)

```bash
# Install development tools and SSL
sudo dnf install -y \
    gcc \
    gcc-c++ \
    make \
    pkg-config \
    openssl-devel \
    cmake \
    git \
    curl

# For tree-sitter parsing
sudo dnf install -y \
    clang-devel \
    llvm-devel
```

#### macOS

```bash
# Install Xcode Command Line Tools (includes clang, make, etc.)
xcode-select --install

# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install additional dependencies via Homebrew
brew install \
    pkg-config \
    openssl \
    cmake \
    llvm

# Add OpenSSL to PATH (Apple Silicon Macs)
export PATH="/opt/homebrew/opt/openssl/bin:$PATH"
```

#### Windows (Native)

```powershell
# Install Visual Studio Build Tools 2022
# Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
# Select: "Desktop development with C++" workload

# Install vcpkg (C++ package manager)
git clone https://github.com/microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg integrate install

# Install OpenSSL via vcpkg
.\vcpkg install openssl:x64-windows

# Set environment variable (add to System Properties > Environment Variables)
# OPENSSL_DIR = C:\path\to\vcpkg\installed\x64-windows
```

#### Windows (WSL2 - Recommended)

```bash
# Use WSL2 with Ubuntu 22.04
wsl --install -d Ubuntu-22.04

# Then follow Linux installation instructions inside WSL
```

### Optional Tools

These tools enhance the development experience but are not required.

#### Just (Command Runner)

```bash
# Install just (rust-based make alternative)
cargo install just

# Verify installation
just --version

# View available commands
just --list
```

#### Cargo Watch (Auto-rebuild on file changes)

```bash
# Install cargo-watch
cargo install cargo-watch

# Usage example: rebuild on every file change
cargo watch -x 'build --release'
```

#### Cargo Nextest (Faster test runner)

```bash
# Install cargo-nextest
cargo install cargo-nextest

# Run tests faster (parallel execution)
cargo nextest run --workspace
```

#### Rust Analyzer (IDE Integration)

**VS Code:**
1. Install the "rust-analyzer" extension
2. Reload VS Code
3. rust-analyzer will automatically download and configure

**Neovim:**
```bash
# Using lazy.nvim (example)
# Add to your plugin config:
{
  "rust-lang/rust-analyzer",
  ft = "rust",
  build = ":CargotInstall",
  config = {
    check = { command = "clippy" },
  },
}
```

**Emacs:**
```elisp
;; Using use-package
(use-package rust-mode
  :ensure t)

(use-package rust-analyzer
  :ensure t
  :hook ((rust-mode . rust-analyzer-mode)))
```

#### Pre-commit Hooks

```bash
# Install pre-commit framework
pip install pre-commit  # or: brew install pre-commit

# Install git hooks
pre-commit install

# Run manually
pre-commit run --all-files
```

---

## Installation Steps

### Step 1: Clone Repository

```bash
# Clone via HTTPS
git clone https://github.com/gl33mer/coax.git
cd coax

# Or clone via SSH (if you have SSH keys configured)
git clone git@github.com:gl33mer/coax.git
cd coax
```

### Step 2: Install Rust (if needed)

```bash
# Check if Rust is already installed
rustc --version

# If not installed, use rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version  # Should be 1.75.0 or later
```

### Step 3: Install System Dependencies

**Choose the appropriate command for your OS:**

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install -y build-essential pkg-config libssl-dev cmake git curl

# Fedora/RHEL
sudo dnf install -y gcc gcc-c++ make pkg-config openssl-devel cmake git curl

# macOS
xcode-select --install
brew install pkg-config openssl cmake

# Windows (PowerShell as Administrator)
winget install Microsoft.VisualStudio.2022.BuildTools
```

### Step 4: Install Optional Tools (Recommended)

```bash
# Install just for convenient command shortcuts
cargo install just

# Install cargo-watch for auto-rebuilding
cargo install cargo-watch

# Install cargo-nextest for faster tests
cargo install cargo-nextest

# Verify installations
just --version
cargo watch --version
cargo nextest --version
```

### Step 5: Build Workspace

```bash
# Navigate to repository
cd /path/to/coax

# Build all crates in debug mode (faster, includes debug symbols)
cargo build --workspace

# Or build in release mode (slower, optimized for performance)
cargo build --workspace --release

# Check for compilation errors without building
cargo check --workspace
```

**Build Times (approximate):**

| Machine | Debug Build | Release Build |
|---------|-------------|---------------|
| M3 MacBook Pro | ~30s | ~2min |
| Intel i7 (16GB) | ~1min | ~4min |
| WSL2 (8GB RAM) | ~2min | ~6min |

### Step 6: Run Tests

```bash
# Run all tests (default test harness)
cargo test --workspace

# Run tests with output visible
cargo test --workspace -- --nocapture

# Run tests with nextest (faster)
cargo nextest run --workspace

# Run tests for specific crate
cargo test -p opendev-scanner

# Run only integration tests
cargo test --workspace --test '*'

# Run tests and generate coverage report (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --workspace --out Html
```

### Step 7: Verify Everything Works

```bash
# Run the CLI help command
cargo run --bin opendev-cli -- --help

# Run a test scan on the repository itself
cargo run --bin opendev-cli -- scan secrets --path .

# Check code formatting
cargo fmt --workspace -- --check

# Run clippy (Rust linter)
cargo clippy --workspace -- -D warnings
```

**Expected output:** All commands should complete without errors.

---

## Development Workflow

### Common Commands (Using Just)

If you installed `just`, use these convenient shortcuts:

```bash
# View all available commands
just

# Build workspace
just build

# Build in release mode
just release

# Run all tests
just test

# Run tests with output
just test-verbose

# Run clippy
just lint

# Format code
just fmt

# Check formatting
just fmt-check

# Run all checks (build + test + lint + fmt-check)
just ci

# Run CLI
just run -- scan secrets --path .

# Watch for changes and rebuild
just watch
```

### Justfile Contents

The `justfile` in the repository root contains:

```just
# Coax Justfile

# Default target
default:
    just build

# Build workspace
build:
    cargo build --workspace

# Build in release mode
release:
    cargo build --workspace --release

# Run all tests
test:
    cargo test --workspace

# Run tests with output
test-verbose:
    cargo test --workspace -- --nocapture

# Run clippy with strict warnings
lint:
    cargo clippy --workspace -- -D warnings

# Format all code
fmt:
    cargo fmt --workspace

# Check formatting
fmt-check:
    cargo fmt --workspace -- --check

# Run all CI checks
ci: build test lint fmt-check

# Run the CLI
run *ARGS:
    cargo run --bin opendev-cli -- {{ARGS}}

# Watch for changes and rebuild
watch:
    cargo watch -x 'build --workspace'

# Clean build artifacts
clean:
    cargo clean --workspace

# Update dependencies
update:
    cargo update

# Generate documentation
docs:
    cargo doc --workspace --no-deps
```

### Running Tests

#### Unit Tests

```bash
# Run all unit tests
cargo test --workspace --lib

# Run tests for specific module
cargo test -p opendev-scanner secrets

# Run a specific test
cargo test -p opendev-scanner test_aws_key_detection

# Run tests in parallel (default)
cargo test --workspace

# Run tests sequentially (for debugging)
cargo test --workspace -- --test-threads=1
```

#### Integration Tests

```bash
# Run all integration tests
cargo test --workspace --test '*'

# Run specific integration test
cargo test --test secrets_test

# Run integration tests with output
cargo test --test '*' -- --nocapture
```

#### Test Coverage

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate HTML coverage report
cargo tarpaulin --workspace --out Html --output-dir coverage

# Generate coverage for specific crate
cargo tarpaulin -p opendev-scanner --out Html

# View coverage in browser (Linux)
xdg-open coverage/tarpaulin-report.html

# View coverage in browser (macOS)
open coverage/tarpaulin-report.html
```

### Code Formatting

```bash
# Format all code in workspace
cargo fmt --workspace

# Format specific crate
cargo fmt -p opendev-scanner

# Check formatting without changing files
cargo fmt --workspace -- --check

# Format with custom configuration
cargo fmt -- --config edition=2021
```

**Formatting Configuration:**

The project uses standard Rust formatting. Configuration in `rustfmt.toml`:

```toml
# rustfmt.toml
edition = "2021"
max_width = 100
tab_spaces = 4
newline_style = "Unix"
```

### Running Clippy

```bash
# Run clippy on all crates
cargo clippy --workspace

# Run clippy with strict warnings (treat warnings as errors)
cargo clippy --workspace -- -D warnings

# Run clippy with specific lints
cargo clippy --workspace -- -W clippy::perf -W clippy::correctness

# Auto-fix clippy warnings
cargo clippy --workspace --fix --allow-dirty

# Run clippy on specific crate
cargo clippy -p opendev-scanner
```

**Clippy Configuration:**

The project uses `clippy.toml` for custom lint settings:

```toml
# clippy.toml
too-many-arguments-threshold = 8
cognitive-complexity-threshold = 25
```

### Building Release

```bash
# Build release binary
cargo build --workspace --release

# Find the release binary
ls -lh target/release/opendev-cli

# Strip binary for smaller size
strip target/release/opendev-cli

# Check binary size
ls -lh target/release/opendev-cli

# Typical sizes:
# - Linux:   ~8-12 MB
# - macOS:   ~10-15 MB
# - Windows: ~12-18 MB
```

#### Cross-Compilation

```bash
# Install cross (cross-compilation tool)
cargo install cross

# Build for Linux (x86_64)
cross build --release --target x86_64-unknown-linux-gnu

# Build for macOS (Apple Silicon)
cross build --release --target aarch64-apple-darwin

# Build for Windows (x86_64)
cross build --release --target x86_64-pc-windows-gnu
```

### Continuous Integration

The project uses GitHub Actions for CI. Workflows are defined in `.github/workflows/`:

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --workspace
      - run: cargo test --workspace
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo fmt --workspace -- --check
```

**CI checks run automatically on:**
- Every push to `main`
- Every pull request
- Every release tag

---

## Troubleshooting

### Build Errors

#### Error: `rustc` version too old

```
error: Rustc 1.75.0 is not supported
```

**Solution:** Update Rust to the latest stable version.

```bash
rustup update stable
rustup default stable
rustc --version  # Verify: should be 1.75.0 or later
```

#### Error: Missing `pkg-config`

```
error: failed to run custom build command for `openssl-sys`
Could not find directory of OpenSSL installation
```

**Solution:** Install system dependencies.

```bash
# Ubuntu/Debian
sudo apt install pkg-config libssl-dev

# macOS
brew install pkg-config openssl

# Fedora/RHEL
sudo dnf install pkg-config openssl-devel
```

#### Error: OpenSSL not found

```
error: could not find native static library `ssl`
```

**Solution:** Set OpenSSL environment variables.

```bash
# macOS (Apple Silicon)
export OPENSSL_DIR="/opt/homebrew/opt/openssl"
export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl/lib/pkgconfig"

# macOS (Intel)
export OPENSSL_DIR="/usr/local/opt/openssl"
export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"

# Linux (if installed via apt)
export OPENSSL_DIR="/usr"

# Add to ~/.bashrc or ~/.zshrc for persistence
echo 'export OPENSSL_DIR="/opt/homebrew/opt/openssl"' >> ~/.zshrc
source ~/.zshrc
```

#### Error: Linker not found

```
error: linker `cc` not found
```

**Solution:** Install build essentials.

```bash
# Ubuntu/Debian
sudo apt install build-essential

# macOS
xcode-select --install

# Fedora/RHEL
sudo dnf install gcc gcc-c++ make
```

#### Error: Permission denied on target directory

```
error: failed to create directory `/path/to/target`
Permission denied (os error 13)
```

**Solution:** Fix permissions or clean build.

```bash
# Fix ownership (Linux/macOS)
sudo chown -R $USER:$USER target

# Or clean and rebuild
cargo clean
cargo build --workspace
```

### Test Failures

#### Test fails with timeout

```
test tests::test_large_file_scan has been running for over 60 seconds
```

**Solution:** Increase test timeout or skip slow tests.

```bash
# Run with increased timeout
cargo test --workspace -- --test-threads=1

# Skip slow tests
cargo test --workspace -- --skip large_file

# Run specific fast tests
cargo test -p opendev-scanner --lib
```

#### Test fails due to missing test fixtures

```
thread 'test_secret_detection' panicked at 'test file not found'
```

**Solution:** Ensure test corpus is present.

```bash
# Check if test fixtures exist
ls tests/corpus/secrets/

# If missing, restore from git
git checkout tests/corpus/

# Or regenerate test fixtures
just generate-fixtures
```

### Missing Dependencies

#### Cargo dependency resolution fails

```
error: failed to select a version for `clap`
```

**Solution:** Update Cargo.lock.

```bash
# Remove lock file and regenerate
rm Cargo.lock
cargo update

# Or update specific dependency
cargo update -p clap
```

#### Dependency requires newer Rust version

```
error: package `regex v1.12.0` requires rustc 1.76.0 or newer
```

**Solution:** Update Rust or pin dependency version.

```bash
# Update Rust
rustup update stable

# Or pin to older version in Cargo.toml
regex = "1.10.0"  # Compatible with Rust 1.75.0
```

### Platform-Specific Issues

#### Linux: libssl version mismatch

```
error: undefined reference to `SSL_CTX_new'
```

**Solution:** Rebuild with correct OpenSSL version.

```bash
# Clean build artifacts
cargo clean

# Rebuild with fresh dependencies
cargo build --workspace

# If still failing, specify OpenSSL path
export OPENSSL_DIR="/usr"
cargo build --workspace
```

#### macOS: Code signing issues

```
error: code object is not signed at all
```

**Solution:** Ad-hoc sign the binary.

```bash
# Ad-hoc sign (for development)
codesign --force --deep --sign - target/release/opendev-cli

# Or disable Gatekeeper temporarily
sudo spctl --master-disable
```

#### macOS: Apple Silicon compatibility

```
warning: linking with `cc` failed; exit status: 1
```

**Solution:** Use native ARM64 toolchain.

```bash
# Ensure you're using ARM64 Rust
rustup target add aarch64-apple-darwin

# Build for native architecture
cargo build --target aarch64-apple-darwin

# Or use Rosetta 2 (not recommended)
arch -x86_64 cargo build
```

#### Windows: MSVC runtime missing

```
error: The code execution cannot proceed because VCRUNTIME140.dll was not found
```

**Solution:** Install Visual C++ Redistributable.

```powershell
# Download and install from Microsoft:
# https://aka.ms/vs/17/release/vc_redist.x64.exe

# Or install via winget
winget install Microsoft.VCRedist.2015+.x64
```

#### Windows: Path length exceeded

```
error: failed to read file: Path too long
```

**Solution:** Enable long paths in Windows.

```powershell
# Enable long paths (requires Administrator)
New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" `
  -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force

# Or use shorter path for repository
# Move from C:\Users\Name\Projects\... to C:\dev\
```

#### WSL2: File watching issues

```
error: inotify watch limit reached
```

**Solution:** Increase inotify limits.

```bash
# Increase watch limit
echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf
sudo sysctl -p

# Verify new limit
cat /proc/sys/fs/inotify/max_user_watches
```

### General Debugging Tips

#### Enable verbose output

```bash
# Verbose cargo output
cargo build -vv

# Verbose test output
cargo test -- --nocapture --verbose

# Backtrace on panic (Linux/macOS)
export RUST_BACKTRACE=1
cargo run

# Full backtrace
export RUST_BACKTRACE=full
```

#### Debug build vs release build

```bash
# Debug build (faster compilation, slower execution)
cargo build --workspace

# Release build (slower compilation, faster execution)
cargo build --workspace --release

# Profile-guided optimization (advanced)
cargo build --workspace --release --features pgo
```

#### Check for common issues

```bash
# Run cargo doctor (if installed)
cargo install cargo-doctor
cargo doctor

# Check for security advisories
cargo install cargo-audit
cargo audit

# Check for outdated dependencies
cargo install cargo-outdated
cargo outdated
```

---

## Project Structure

### Workspace Layout

```
coax/
├── Cargo.toml              # Workspace root configuration
├── Cargo.lock              # Dependency lock file (auto-generated)
├── justfile                # Command shortcuts
├── rustfmt.toml            # Code formatting configuration
├── clippy.toml             # Clippy lint configuration
├── .gitignore              # Git ignore patterns
├── README.md               # Project overview
├── DEVELOPMENT.md          # This file
├── CONTRIBUTING.md         # Contribution guidelines
├── LICENSE                 # MIT OR Apache-2.0
│
├── crates/                 # Rust workspace crates
│   ├── opendev-cli/        # CLI entry point
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs     # CLI argument parsing, command dispatch
│   │
│   ├── opendev-scanner/    # Core scanning engine
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs      # Library entry point
│   │       ├── scanner.rs  # File traversal, gitignore handling
│   │       ├── secrets.rs  # Secret detection patterns
│   │       ├── entropy.rs  # Entropy-based secret detection
│   │       ├── config.rs   # YAML configuration loading
│   │       ├── output.rs   # Text/JSON/SARIF output
│   │       └── error.rs    # Error types and handling
│   │
│   ├── opendev-parser/     # Phase 2: Code parser (AST/CFG)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── parser/     # Tree-sitter integration
│   │       └── languages.rs # Language support
│   │
│   ├── opendev-threat-model/ # Phase 2: Threat modeling
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── generator.rs # Threat model generation
│   │       └── templates.rs # STRIDE, PLOT4 templates
│   │
│   ├── coax-tui/           # TUI dashboard
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs      # TUI library entry point
│   │       ├── app.rs      # Application state management
│   │       ├── ui.rs       # UI rendering
│   │       └── events.rs   # Event handling
│   │
│   ├── opendev-binary/     # Phase 3: Binary analysis
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── ghidra.rs   # Ghidra integration
│           └── verifier.rs # Source↔binary correlation
│
├── config/                 # Configuration files
│   ├── patterns/
│   │   ├── secrets.yml     # Secret detection patterns (YAML)
│   │   ├── vulnerabilities.yml # Vulnerability signatures
│   │   └── malware.yml     # Malware signatures (Phase 3)
│   └── templates/
│       ├── threagile.yml   # Threat model templates
│       └── sarif.yml       # SARIF report templates
│
├── tests/                  # Test suites
│   ├── integration/        # Integration tests
│   │   ├── secrets_test.rs
│   │   ├── parser_test.rs
│   │   └── threat_model_test.rs
│   ├── benchmarks/         # Performance benchmarks
│   │   ├── scan_speed.rs
│   │   └── memory_usage.rs
│   └── corpus/             # Test fixtures
│       ├── secrets/        # Files with known secrets
│       ├── vulnerabilities/ # Files with known vulnerabilities
│       └── binaries/       # Test binaries (gitignored)
│
├── docs/                   # Documentation
│   ├── ARCHITECTURE.md     # System architecture
│   ├── HANDBOOK.md         # Developer handbook
│   ├── ROADMAP.md          # Development roadmap
│   ├── RESEARCH/           # Research summaries
│   │   ├── binmetric-summary.md
│   │   ├── kong-analysis.md
│   │   └── vulnllm-r-7b.md
│   └── HANDOFF/            # Session handoffs
│
├── scripts/                # Utility scripts
│   ├── setup.sh            # Development setup
│   ├── benchmark.sh        # Run benchmarks
│   ├── release.sh          # Release automation
│   └── update-patterns.sh  # Pattern database updates
│
└── .github/                # GitHub configuration
    ├── workflows/
    │   ├── ci.yml          # Continuous integration
    │   ├── release.yml     # Automated releases
    │   └── benchmark.yml   # Performance regression
    └── ISSUE_TEMPLATE/
        ├── bug_report.md
        └── feature_request.md
```

### Key Crates

#### opendev-cli

**Location:** `crates/opendev-cli/`

**Purpose:** Command-line interface for Coax

**Main Commands:**
- `opendev scan secrets` - Scan for secrets
- `opendev scan vulnerabilities` - Scan for vulnerabilities (Phase 2)
- `opendev threat-model` - Generate threat models (Phase 2)
- `opendev report` - Generate reports (SARIF, YAML, JSON)

**Key Files:**
- `src/main.rs` - CLI entry point, argument parsing with Clap

**Dependencies:**
- `clap` - Argument parsing
- `opendev-scanner` - Core scanning functionality

#### opendev-scanner

**Location:** `crates/opendev-scanner/`

**Purpose:** Core security scanning engine

**Features:**
- File traversal with gitignore support
- Secret detection (regex + entropy)
- Vulnerability detection (Phase 2)
- Configurable patterns (YAML)
- Multiple output formats (Text, JSON, SARIF, YAML)

**Key Files:**
- `src/lib.rs` - Library entry point, public API
- `src/scanner.rs` - File traversal, parallel scanning
- `src/secrets.rs` - Secret detection patterns
- `src/entropy.rs` - Shannon entropy analysis
- `src/config.rs` - YAML configuration loading
- `src/output.rs` - Report generation
- `src/error.rs` - Error types

**Dependencies:**
- `regex` - Pattern matching
- `ignore` - Gitignore-aware file traversal
- `serde` / `serde_yaml` - Configuration parsing
- `thiserror` - Error handling

#### opendev-parser

**Location:** `crates/opendev-parser/`

**Purpose:** Code parsing for AST/CFG generation (Phase 2)

**Features:**
- Tree-sitter integration
- Multi-language support (Rust, Python, JavaScript, Go)
- AST extraction
- CFG generation
- Code slicing for LLM context

**Status:** Phase 2 (in development)

#### opendev-threat-model

**Location:** `crates/opendev-threat-model/`

**Purpose:** Automated threat modeling (Phase 2)

**Features:**
- STRIDE threat categorization
- PLOT4 attack tree generation
- YAML/JSON export
- Integration with threat modeling tools (Threagile, OWASP Threat Dragon)

**Status:** Phase 2 (in development)

#### coax-tui

**Location:** `crates/coax-tui/`

**Purpose:** Interactive terminal UI dashboard

**Features:**
- Real-time finding visualization
- Interactive filtering by severity
- Search functionality
- False positive marking
- Baseline file creation
- Code context navigation

**Key Files:**
- `src/lib.rs` - TUI library entry point
- `src/app.rs` - Application state and navigation
- `src/ui.rs` - UI rendering components
- `src/events.rs` - Event handling

**Dependencies:**
- `ratatui` - Terminal UI framework
- `crossterm` - Terminal manipulation
- `coax-scanner` - Core scanning functionality

**Status:** Active development

#### opendev-binary

**Location:** `crates/opendev-binary/`

**Purpose:** Binary analysis and verification (Phase 3)

**Features:**
- Ghidra integration
- DWARF/debug symbol parsing
- Source↔binary correlation
- Malware signature detection (YARA)
- Agentic deobfuscation

**Status:** Phase 3 (planned)

### Where to Find Things

| Task | Location |
|------|----------|
| Add secret detection pattern | `config/patterns/secrets.yml` |
| Modify CLI arguments | `crates/opendev-cli/src/main.rs` |
| Change file traversal logic | `crates/opendev-scanner/src/scanner.rs` |
| Update entropy threshold | `crates/opendev-scanner/src/entropy.rs` |
| Add output format | `crates/opendev-scanner/src/output.rs` |
| Modify error types | `crates/opendev-scanner/src/error.rs` |
| Add language support | `crates/opendev-parser/src/languages.rs` |
| Change threat model template | `crates/opendev-threat-model/src/templates.rs` |
| Update CI workflow | `.github/workflows/ci.yml` |
| Modify just commands | `justfile` |
| Change code formatting | `rustfmt.toml` |
| Add clippy lint | `clippy.toml` |

---

## Additional Resources

### Documentation

- [README.md](README.md) - Project overview and quick start
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - System architecture
- [ROADMAP.md](docs/ROADMAP.md) - Development roadmap

### External Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Clap Book](https://docs.rs/clap/latest/clap/) - CLI framework
- [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) - Parsing library
- [SARIF Specification](https://docs.oasis-open.org/sarif/sarif/v2.1.0/) - Static analysis results format

### Community

- [GitHub Issues](https://github.com/gl33mer/coax/issues) - Bug reports and feature requests
- [GitHub Discussions](https://github.com/gl33mer/coax/discussions) - Questions and discussions
- [Discord](https://discord.gg/coax) - Real-time chat (coming soon)

---

**Happy coding! 🛡️**
