# Quick Start Tutorial

This tutorial gets you up and running with Ghost Monkey in its current development state.

## Prerequisites

- Rust 1.85 or later
- Git
- Unix-like system (Linux, macOS, WSL)

## 5-Minute Setup

### 1. Install Rust (if needed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Clone and Build

```bash
git clone https://github.com/unclesp1d3r/ghost_monkey.git
cd ghost_monkey
cargo build --release
```

### 3. Run the Binaries

```bash
# Run the client
./target/release/ghost-client

# Run the implant
./target/release/ghost-implant
```

You should see educational messages indicating these are placeholder implementations.

## Understanding the Output

Both binaries currently display:

- Educational tool identification
- Placeholder implementation notice

This is expected behavior for the current development phase.

## Exploring the Project

### Project Structure

```bash
# View the source structure
tree src/
```

### Documentation

```bash
# Build and serve the documentation
cargo install mdbook
mdbook serve docs/
```

Then open http://localhost:3000 in your browser.

### Development Planning

```bash
# View the implementation tasks
cat .kiro/specs/core-networking-protocol/tasks.md
```

## Development Workflow

### Making Changes

```bash
# Make changes to source files
# Build and test
cargo build
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy
```

### Cross-Platform Building

```bash
# Install cross-compilation tools
cargo install cargo-zigbuild

# Build for different targets (when implemented)
cargo zigbuild --target x86_64-unknown-linux-gnu
```

## What's Next?

1. **Read the Architecture**: Understand the planned system design
2. **Review Tasks**: Check the implementation roadmap
3. **Contribute**: Pick up development tasks
4. **Stay Updated**: Follow the repository for progress

## Current Limitations

Remember that Ghost Monkey is in early development:

- No network functionality yet
- No encryption implementation
- No TUI interface
- Placeholder command-line interfaces

These features are planned and documented in the architecture guides.

## Safety First

Even during development:

- Use isolated environments
- Test only on localhost
- Run as non-privileged user
- Follow ethical guidelines

## Getting Help

- Check the [Installation Guide](./installation.md) for setup issues
- Review [Safety Guidelines](./safety-guidelines.md) for best practices
- Read the [Architecture Overview](../ch02-architecture/overview.md) for design details
- Visit the GitHub repository for the latest updates

---

You're now ready to explore Ghost Monkey! Continue with the detailed [Architecture](../ch02-architecture/overview.md) documentation to understand the planned system design.
