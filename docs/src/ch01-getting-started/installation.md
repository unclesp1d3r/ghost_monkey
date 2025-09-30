# Installation

This chapter covers the installation and setup of Ghost_Monkey on various platforms.

## Prerequisites

Before installing Ghost_Monkey, ensure you have the following prerequisites:

### Rust Toolchain

Ghost_Monkey requires Rust 1.70 or later with the 2024 edition:

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Ensure you have the latest stable toolchain
rustup update stable
rustup default stable

# Add required components
rustup component add rustfmt clippy
```

### System Dependencies

#### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

#### Linux (CentOS/RHEL/Fedora)

```bash
# CentOS/RHEL
sudo yum groupinstall "Development Tools"
sudo yum install openssl-devel

# Fedora
sudo dnf groupinstall "Development Tools"
sudo dnf install openssl-devel
```

#### macOS

```bash
# Install Xcode command line tools
xcode-select --install

# Or install via Homebrew
brew install openssl pkg-config
```

## Building from Source

### Clone the Repository

```bash
git clone https://github.com/your-org/ghost_monkey.git
cd ghost_monkey
```

### Build the Project

```bash
# Development build
cargo build

# Optimized release build
cargo build --release
```

### Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

## Cross-Platform Builds

Ghost_Monkey supports cross-compilation for multiple platforms using `cargo-zigbuild`:

### Install cargo-zigbuild

```bash
# Install cargo-zigbuild (includes Zig)
cargo install cargo-zigbuild

# Or install via pip (if you have Python)
pip install cargo-zigbuild
```

### Add Target Platforms

```bash
# Add common targets
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### Cross-Compile

```bash
# Linux x86_64
cargo zigbuild --target x86_64-unknown-linux-gnu --release

# Linux ARM64
cargo zigbuild --target aarch64-unknown-linux-gnu --release

# macOS Intel
cargo zigbuild --target x86_64-apple-darwin --release

# macOS Apple Silicon
cargo zigbuild --target aarch64-apple-darwin --release

# macOS Universal Binary
cargo zigbuild --target universal2-apple-darwin --release
```

## Binary Locations

After building, the binaries will be located in:

```
target/
├── debug/                  # Development builds
│   ├── ghost-client       # Debug client binary
│   └── ghost-implant      # Debug implant binary
└── release/               # Optimized builds
    ├── ghost-client       # Release client binary
    └── ghost-implant      # Release implant binary
```

## Verification

Verify your installation by running:

```bash
# Check client version
./target/release/ghost-client --version

# Check implant version
./target/release/ghost-implant --version

# Run basic connectivity test
./target/release/ghost-client --help
./target/release/ghost-implant --help
```

## Development Tools

For development and testing, install additional tools:

```bash
# Fast test runner
cargo install cargo-nextest

# Documentation tools
cargo install mdbook
cargo install mdbook-mermaid
cargo install mdbook-admonish

# Code coverage
cargo install cargo-tarpaulin
```

## Next Steps

Once installation is complete, proceed to:

- [First Run](first-run.md) - Your first Ghost_Monkey session
- [Quick Start Guide](quick-start.md) - Basic usage examples
- [Safety Guidelines](safety-guidelines.md) - Important safety considerations
