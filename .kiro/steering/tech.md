# Technology Stack & Build System

## Primary Technology Stack

- **Language**: Rust (stable channel)
- **Edition**: 2024 (as specified in rustfmt.toml)
- **Minimum Rust Version**: ≥ 1.70
- **Target Platforms**: Maximum Rust platform support
  - **Tier 1**: Linux (x86_64, aarch64), macOS (x86_64, Apple Silicon), Windows (x86_64)
  - **Tier 2**: FreeBSD, OpenBSD, NetBSD, Android, iOS, WebAssembly
  - **Embedded**: ARM Cortex-M, RISC-V (where applicable)
  - **Cross-compilation**: Support for all major architectures

## Core Dependencies

### Runtime Dependencies

- **Primary Networking**: `network-protocol` v1.0.0 for all transport and protocol needs
  - Cross-platform transport layer abstraction
  - Built-in packet codec for message serialization
  - ECDH handshake protocol with secure key exchange
  - Heartbeat and keepalive mechanisms
  - Platform-agnostic error handling and logging
- **TUI Interface**: `ratatui` for rich terminal user interface
  - **Backend**: `crossterm` for cross-platform terminal handling
  - **Layout**: Multi-pane interface with real-time updates
  - **Input**: Keyboard shortcuts and mouse support
  - **Widgets**: Progress bars, tables, syntax highlighting, scrollable text
- **Terminal**: `portable-pty` for cross-platform interactive shell sessions
- **Process Execution**: Platform-specific process spawning
  - Unix: `tokio::process::Command`
  - Windows: `tokio::process::Command` with Windows-specific handling
  - Embedded: Conditional compilation for limited environments
- **Async Runtime**: `tokio` with platform-appropriate feature flags

### Development Dependencies

- **CLI**: `clap` with derive macros for cross-platform argument parsing
- **Error Handling**: `anyhow` and `thiserror` for platform-agnostic error management
- **Logging**: `tracing` and `tracing-subscriber` for cross-platform structured debugging
- **Testing**: Standard Rust testing with `tokio-test` for async tests
- **TUI Development**:
  - `ratatui` for terminal user interface framework
  - `crossterm` for cross-platform terminal control
  - `tui-textarea` for text input widgets (if needed)
  - `syntect` for syntax highlighting in code displays
- **Cross-compilation**: `cargo-zigbuild` for superior cross-compilation via Zig
- **Platform Detection**: `cfg-if` for conditional compilation

## Build System

### Rust Toolchain Configuration

- **Channel**: stable
- **Components**: rustfmt, clippy
- **Configuration**: `rust-toolchain.toml` specifies toolchain requirements

### Code Formatting

- **Tool**: rustfmt with 2024 edition style
- **Configuration**: `rustfmt.toml` and `.editorconfig`
- **Standards**: 4-space indentation for Rust, consistent line endings (LF)

### Common Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Code formatting
cargo fmt

# Linting
cargo clippy -- -D warnings

# Documentation generation
cargo doc --open
```

### Cross-Platform Build Support

Ghost_Monkey supports multiple cross-compilation approaches for maximum platform coverage:

#### Preferred: cargo-zigbuild (Recommended)

```bash
# Install Zig and cargo-zigbuild
cargo install cargo-zigbuild
# Alternative: pip install cargo-zigbuild (includes zig)

# Add Rust targets
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for Linux targets (cargo-zigbuild's strength)
cargo zigbuild --target x86_64-unknown-linux-gnu
cargo zigbuild --target aarch64-unknown-linux-gnu
cargo zigbuild --target armv7-unknown-linux-gnueabihf

# Build for macOS (cross-platform from Linux/Windows)
cargo zigbuild --target x86_64-apple-darwin
cargo zigbuild --target aarch64-apple-darwin

# macOS universal2 binaries (special cargo-zigbuild feature)
cargo zigbuild --target universal2-apple-darwin

# Specify glibc version for Linux targets
cargo zigbuild --target x86_64-unknown-linux-gnu.2.17
```

#### Alternative: cross (Docker-based)

```bash
# Install cross for Docker-based cross-compilation
cargo install cross

# Build for various targets using Docker containers
cross build --target x86_64-unknown-linux-gnu
cross build --target aarch64-unknown-linux-gnu
cross build --target x86_64-pc-windows-gnu
cross build --target x86_64-unknown-freebsd
```

#### Alternative: cargo-xwin (Windows MSVC)

```bash
# Install cargo-xwin for Windows MSVC targets
cargo install cargo-xwin

# Build for Windows MSVC targets
cargo xwin build --target x86_64-pc-windows-msvc
cargo xwin build --target aarch64-pc-windows-msvc
```

#### Standard cargo (Native and simple targets)

```bash
# Native compilation
cargo build

# Simple cross-compilation for supported targets
cargo build --target wasm32-unknown-unknown
cargo build --target x86_64-unknown-linux-musl
```

### Cross-Compilation Tool Comparison

| Tool               | Best For      | Pros                           | Cons                      |
| ------------------ | ------------- | ------------------------------ | ------------------------- |
| **cargo-zigbuild** | Linux, macOS  | Fast, no Docker, glibc control | Limited Windows support   |
| **cross**          | All platforms | Comprehensive target support   | Docker overhead, slower   |
| **cargo-xwin**     | Windows MSVC  | Native Windows toolchain       | Windows targets only      |
| **cargo**          | Native/simple | Built-in, fast                 | Limited cross-compilation |

**Recommendation**: Use `cargo-zigbuild` as the primary cross-compilation tool, with fallbacks to other tools for specific targets as needed.

## Documentation System

- **Tool**: MkDocs with Material theme
- **Configuration**: `mkdocs.yml`
- **Features**: Code highlighting, Mermaid diagrams, search, dark/light themes
- **Deployment**: GitHub Pages integration

## Code Quality Tools

### Linting & Formatting

- **rustfmt**: Code formatting (2024 edition style)
- **clippy**: Rust linter for catching common mistakes
- **EditorConfig**: Consistent editor settings across team

### Testing Strategy

- Unit tests with `cargo test`
- Integration tests for client-server communication
- Safety-focused testing (localhost only, non-privileged)

## Development Workflow

### Local Development

1. Use `cargo build` for development builds
2. Test with `cargo test` in isolated environments
3. Format code with `cargo fmt`
4. Lint with `cargo clippy`

### Safety Requirements

- All testing must use localhost (127.0.0.1) only
- Run as non-privileged user
- Use benign commands only (`whoami`, `pwd`, `ls`)
- No external network connections during development

## Project Structure Conventions

```
ghost_monkey/
├── src/                    # Rust source code
│   ├── client/            # Client implementation
│   ├── implant/           # Implant/server implementation
│   └── lib.rs             # Shared library code
├── tests/                 # Test files
├── docs/                  # Documentation
├── target/                # Build artifacts (gitignored)
└── Cargo.{toml,lock}      # Rust package configuration
```

## Binary Targets

- **ghost-client**: Interactive client leveraging `network-protocol` transport abstraction
- **ghost-implant**: Implant using `network-protocol` for secure communications
- **Transport Options** (via network-protocol):
  - Local transport for same-machine testing
  - Remote transport for network communication
  - TLS transport for encrypted connections
- **Connection Modes**:
  - Call-in: `ghost-client <host>` → `ghost-implant --listen`
  - Callback: `ghost-client --listen` ← `ghost-implant --callback <host>`
- **Security**: Built-in ECDH handshake and packet-level encryption via network-protocol

Both binaries should be built with safety constraints and educational warnings built-in.
