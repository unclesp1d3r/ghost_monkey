# Installation

This chapter covers installing Ghost Monkey and its dependencies on various platforms.

## System Requirements

### Minimum Requirements

- **Rust**: Version 1.85 or later (2024 edition)
- **Operating System**: Unix-like systems (Linux, macOS, WSL on Windows)
- **Memory**: 512 MB RAM (for compilation)
- **Disk Space**: 100 MB for source code and dependencies

### Recommended Requirements

- **Rust**: Latest stable version
- **Memory**: 2 GB RAM (for faster compilation)
- **Disk Space**: 1 GB for cross-compilation targets
- **Network**: Internet connection for dependency downloads

## Installing Rust

If you don't have Rust installed, install it using rustup:

```bash
# Install Rust using the official installer
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the on-screen instructions, then reload your shell
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Updating Rust

Keep Rust up to date for the latest features and security fixes:

```bash
# Update Rust toolchain
rustup update

# Update cargo and other tools
cargo install-update -a
```

## Installing Ghost Monkey

### From Source (Recommended)

Clone and build from the official repository:

```bash
# Clone the repository
git clone https://github.com/unclesp1d3r/ghost_monkey.git
cd ghost_monkey

# Build in release mode for optimal performance
cargo build --release

# Verify the build (currently shows placeholder messages)
./target/release/ghost-client
./target/release/ghost-implant
```

### Development Build

For development and testing:

```bash
# Build in debug mode (faster compilation, slower execution)
cargo build

# Run tests to verify everything works
cargo test

# Generate documentation
cargo doc --open
```

## Optional Dependencies

### Cross-Compilation Tools

For building binaries for different platforms:

#### cargo-zigbuild (Recommended)

```bash
# Install cargo-zigbuild for superior cross-compilation
cargo install cargo-zigbuild

# Verify installation
cargo zigbuild --version
```

#### Alternative: cross (Docker-based)

```bash
# Install cross for Docker-based cross-compilation
cargo install cross

# Verify installation
cross --version
```

#### Alternative: cargo-xwin (Windows MSVC)

```bash
# Install cargo-xwin for Windows MSVC targets
cargo install cargo-xwin

# Verify installation
cargo xwin --version
```

### Testing and Development Tools

```bash
# Install nextest for faster test execution
cargo install cargo-nextest

# Install additional development tools
cargo install cargo-watch      # Auto-rebuild on file changes
cargo install cargo-audit      # Security vulnerability scanning
cargo install cargo-outdated   # Check for outdated dependencies
```

### Documentation Tools

```bash
# Install mdbook for building the user guide
cargo install mdbook

# Install mdbook plugins
cargo install mdbook-mermaid    # Mermaid diagram support
cargo install mdbook-toc        # Table of contents generation
```

## Platform-Specific Instructions

### Linux

#### Ubuntu/Debian

```bash
# Install system dependencies
sudo apt update
sudo apt install build-essential pkg-config libssl-dev

# Install Rust and Ghost Monkey as described above
```

#### CentOS/RHEL/Fedora

```bash
# Install system dependencies
sudo dnf install gcc pkg-config openssl-devel

# Or on older systems:
# sudo yum install gcc pkg-config openssl-devel

# Install Rust and Ghost Monkey as described above
```

#### Arch Linux

```bash
# Install system dependencies
sudo pacman -S base-devel pkg-config openssl

# Install Rust and Ghost Monkey as described above
```

### macOS

#### Using Homebrew

```bash
# Install Xcode command line tools
xcode-select --install

# Install Rust and Ghost Monkey as described above
```

#### Using MacPorts

```bash
# Install required tools
sudo port install pkgconfig openssl

# Install Rust and Ghost Monkey as described above
```

### Windows (WSL)

Ghost Monkey is designed for Unix-like systems. On Windows, use WSL:

```bash
# Install WSL2 (run in PowerShell as Administrator)
wsl --install

# Once in WSL, follow the Linux instructions above
```

## Verification

After installation, verify everything works:

```bash
# Check Rust version
rustc --version

# Check Ghost Monkey binaries (currently show placeholder messages)
./target/release/ghost-client
./target/release/ghost-implant

# Run tests (currently minimal)
cargo test

# Generate and view documentation
cargo doc --open
```

## Troubleshooting

### Common Issues

#### Compilation Errors

```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

#### Missing System Dependencies

```bash
# On Ubuntu/Debian
sudo apt install build-essential pkg-config libssl-dev

# On CentOS/RHEL/Fedora
sudo dnf install gcc pkg-config openssl-devel
```

#### Network Issues

```bash
# Configure cargo to use a different registry mirror
echo '[source.crates-io]
replace-with = "mirror"

[source.mirror]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"' >> ~/.cargo/config.toml
```

### Getting Help

If you encounter issues:

1. Check the [Troubleshooting](../appendices/troubleshooting.md) section
2. Search existing issues on the GitHub repository
3. Create a new issue with detailed error information

## Next Steps

Once installation is complete, proceed to [First Run](./first-run.md) to test your installation and run Ghost Monkey for the first time.
