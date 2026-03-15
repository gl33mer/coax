# Building Coax from Source

This guide provides step-by-step instructions for building Coax from source.

## Prerequisites

### 1. Rust Toolchain

Coax requires Rust 1.75.0 or later.

**Install Rust:**
```bash
# Install rustup (Rust installer and version manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts, then reload your shell
source $HOME/.cargo/env

# Verify installation
rustc --version  # Should be 1.75.0 or later
cargo --version
```

**Update Rust (if already installed):**
```bash
rustup update
```

### 2. System Dependencies

#### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    cmake \
    git
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install dependencies via Homebrew
brew install pkg-config openssl cmake git
```

#### Windows (WSL2 Recommended)
```bash
# Install WSL2 (from PowerShell as Administrator)
wsl --install

# Restart, then open Ubuntu from Start menu
# Follow Ubuntu instructions above
```

#### Windows (Native - Not Recommended)
```powershell
# Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/
# Select "Desktop development with C++"

# Install vcpkg for dependencies
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg integrate install
```

## Build Instructions

### 1. Clone Repository

```bash
git clone https://github.com/gl33mer/coax.git
cd coax
```

### 2. Build Debug Version

```bash
# Build all crates (debug mode, faster compilation)
cargo build

# Binary location: target/debug/coax
./target/debug/coax --version
```

### 3. Build Release Version (Recommended)

```bash
# Build optimized release binary
cargo build --release

# Binary location: target/release/coax
./target/release/coax --version
```

**Release builds are:**
- 10-100x faster than debug builds
- Smaller binary size
- Recommended for production use

### 4. Run Tests

```bash
# Run all tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture

# Run specific crate tests
cargo test -p coax-scanner

# Run specific test
cargo test -p coax-scanner entropy_filter
```

### 5. Run Benchmarks (Optional)

```bash
# Install cargo-bench if not available
cargo install cargo-bench

# Run benchmarks
cargo bench
```

### 6. Generate Documentation (Optional)

```bash
# Generate HTML documentation
cargo doc --workspace --no-deps

# Open documentation in browser
cargo doc --workspace --no-deps --open
```

## Installation

### Option 1: Install via Cargo

```bash
# Install to ~/.cargo/bin
cargo install --path crates/coax-cli

# Verify installation
coax --version
```

### Option 2: Manual Installation

```bash
# Copy binary to system path
sudo cp target/release/coax /usr/local/bin/

# Verify installation
coax --version
```

### Option 3: Use Without Installing

```bash
# Run directly from target directory
./target/release/coax --help
```

## Verify Installation

```bash
# Check version
coax --version

# View help
coax --help

# Run a test scan
coax scan secrets -p .

# Launch TUI
coax tui
```

## Troubleshooting

### Common Issues

#### "command not found: coax"

**Solution:** Add cargo bin to PATH
```bash
export PATH="$HOME/.cargo/bin:$PATH"
# Add to ~/.bashrc or ~/.zshrc for persistence
```

#### "package `coax v0.4.0` cannot be built"

**Solution:** Update Rust toolchain
```bash
rustup update
```

#### "openssl not found"

**Ubuntu/Debian:**
```bash
sudo apt-get install libssl-dev
```

**macOS:**
```bash
brew install openssl
export OPENSSL_DIR=$(brew --prefix openssl)
```

#### "linker `cc` not found"

**Ubuntu/Debian:**
```bash
sudo apt-get install build-essential
```

**macOS:**
```bash
xcode-select --install
```

#### Build fails with "out of memory"

**Solution:** Build with fewer parallel jobs
```bash
CARGO_BUILD_JOBS=1 cargo build --release
```

### Clean Build

If you encounter strange errors:

```bash
# Clean all build artifacts
cargo clean

# Rebuild
cargo build --release
```

### Update from Git

```bash
# Pull latest changes
git pull origin main

# Rebuild
cargo build --release
```

## System Requirements

### Minimum
- **CPU:** 2 cores
- **RAM:** 2GB
- **Disk:** 500MB (for build artifacts)
- **OS:** Linux, macOS, or Windows (WSL2)

### Recommended
- **CPU:** 4+ cores
- **RAM:** 4GB+
- **Disk:** 1GB (for build artifacts)
- **OS:** Linux or macOS

### Build Times

| System | Debug Build | Release Build |
|--------|-------------|---------------|
| 2-core, 4GB RAM | ~5 min | ~15 min |
| 4-core, 8GB RAM | ~2 min | ~8 min |
| 8-core, 16GB RAM | ~1 min | ~4 min |

## Development Workflow

### Daily Development

```bash
# Make changes
# Run tests
cargo test --workspace

# Check formatting
cargo fmt --workspace -- --check

# Run linter
cargo clippy --workspace -- -D warnings

# Build debug version
cargo build
```

### Before Committing

```bash
# Format code
cargo fmt --workspace

# Run all checks
just ci

# Or manually:
cargo fmt --workspace -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

### Pre-commit Hook

```bash
# Install pre-commit hook
coax pre-commit --install
```

## Additional Resources

- [GitHub Repository](https://github.com/gl33mer/coax)
- [Documentation](https://github.com/gl33mer/coax/tree/main/docs)
- [Issue Tracker](https://github.com/gl33mer/coax/issues)
- [Discussions](https://github.com/gl33mer/coax/discussions)

## Getting Help

If you encounter issues not covered here:

1. Check existing issues: https://github.com/gl33mer/coax/issues
2. Search discussions: https://github.com/gl33mer/coax/discussions
3. Create a new issue with build details
