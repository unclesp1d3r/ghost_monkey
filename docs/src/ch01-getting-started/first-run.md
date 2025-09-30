# First Run

This chapter guides you through running Ghost Monkey for the first time and understanding its current development state.

## Current Implementation Status

Ghost Monkey is currently in early development with placeholder implementations. This guide shows you how to build and run the current version.

## Building the Project

First, ensure you have Rust installed and clone the repository:

```bash
# Clone the repository
git clone https://github.com/unclesp1d3r/ghost_monkey.git
cd ghost_monkey

# Build the project
cargo build --release
```

## Running the Binaries

### Ghost Client

```bash
./target/release/ghost-client
```

**Expected Output:**

```text
Ghost Monkey Client - Educational Tool
This is a placeholder implementation.
```

### Ghost Implant

```bash
./target/release/ghost-implant
```

**Expected Output:**

```text
Ghost Monkey Implant - Educational Tool
This is a placeholder implementation.
```

## Understanding the Current State

### What's Implemented

- ✅ Basic project structure with Cargo.toml
- ✅ Binary targets for client and implant
- ✅ Development dependencies configured
- ✅ Cross-compilation tooling setup
- ✅ Comprehensive documentation structure
- ✅ Educational safety guidelines

### What's Planned

- 🔄 Network protocol implementation using `network-protocol` crate
- 🔄 Secure handshake with ECDH key exchange
- 🔄 ChaCha20-Poly1305 encryption
- 🔄 Rich TUI interface with ratatui
- 🔄 Interactive shell support
- 🔄 File transfer capabilities
- 🔄 Cross-platform compatibility

## Development Environment

### Checking Dependencies

Verify your development environment:

```bash
# Check Rust version (should be 1.85+)
rustc --version

# Check cargo version
cargo --version

# Run tests (currently minimal)
cargo test

# Generate documentation
cargo doc --open
```

### Development Tools

Install recommended development tools:

```bash
# Enhanced test runner
cargo install cargo-nextest

# Cross-compilation support
cargo install cargo-zigbuild

# Documentation tools
cargo install mdbook
```

## Next Steps

1. **Explore the Code**: Review the placeholder implementations in `src/client/main.rs` and `src/implant/main.rs`

2. **Read the Specifications**: Check out the detailed planning in `.kiro/specs/core-networking-protocol/`

3. **Contribute**: The project is open for contributions. See the task list for implementation opportunities

4. **Follow Development**: Watch the repository for updates as features are implemented

## Safety Reminders

Even in development:

- Always run in isolated environments
- Use localhost (127.0.0.1) for testing
- Run as non-privileged user
- Follow the [Safety Guidelines](./safety-guidelines.md)

## Getting Help

If you encounter issues:

1. Check that Rust 1.85+ is installed
2. Ensure all dependencies are available
3. Review the [Installation](./installation.md) guide
4. Check the project repository for known issues

---

Ready to dive deeper? Continue with the [Architecture Overview](../ch02-architecture/overview.md) to understand the planned system design.
