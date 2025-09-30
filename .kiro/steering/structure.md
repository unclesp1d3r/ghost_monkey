# Project Organization & Structure

## Repository Layout

```
ghost_monkey/
├── .git/                       # Git version control
├── .kiro/                      # Kiro IDE configuration
│   └── steering/               # AI assistant steering rules
├── src/                        # Source code (Rust implementation)
│   ├── client/                 # Client binary implementation
│   │   ├── main.rs            # Client entry point with TUI initialization
│   │   ├── app.rs             # Main application state and event handling
│   │   ├── ui/                # TUI interface components
│   │   │   ├── mod.rs         # UI module
│   │   │   ├── layout.rs      # Multi-pane layout management
│   │   │   ├── shell.rs       # Interactive shell pane
│   │   │   ├── transfer.rs    # File transfer pane
│   │   │   ├── logs.rs        # Logging and status pane
│   │   │   ├── connection.rs  # Connection status and controls
│   │   │   └── help.rs        # Help and keyboard shortcuts
│   │   ├── events.rs          # Event handling and input processing
│   │   ├── connect.rs         # Call-in mode (connect to implant)
│   │   └── listen.rs          # Callback mode (listen for implant)
│   ├── implant/               # Implant/server implementation
│   │   ├── main.rs            # Implant entry point with protocol/mode selection
│   │   ├── server.rs          # Call-in mode (listen for client)
│   │   └── callback.rs        # Callback mode (connect to client)
│   ├── transport/             # Transport layer abstraction
│   │   ├── mod.rs             # Transport module using network-protocol crate
│   │   ├── tcp.rs             # TCP transport implementation
│   │   ├── quic.rs            # QUIC transport implementation
│   │   ├── tls.rs             # TLS transport (via network-protocol)
│   │   └── traits.rs          # Common transport traits
│   ├── protocol/              # Protocol definitions (leveraging network-protocol)
│   │   ├── mod.rs             # Protocol module with PacketCodec integration
│   │   ├── message.rs         # Message types using network-protocol::Packet
│   │   ├── handshake.rs       # ECDH handshake via network-protocol
│   │   └── heartbeat.rs       # Keepalive and heartbeat mechanisms
│   ├── crypto/                # Modern cryptography
│   │   ├── mod.rs             # Crypto module
│   │   ├── aead.rs            # ChaCha20-Poly1305 AEAD
│   │   └── keyexchange.rs     # X25519 ECDH key exchange
│   ├── shell/                 # Interactive shell support
│   │   ├── mod.rs             # Shell module
│   │   ├── pty.rs             # PTY handling
│   │   └── terminal.rs        # Terminal emulation
│   ├── transfer/              # File transfer
│   │   ├── mod.rs             # Transfer module
│   │   ├── integrity.rs       # File integrity verification
│   │   └── streams.rs         # QUIC multi-stream support
│   └── lib.rs                 # Shared library code
├── tests/                     # Test files
├── docs/                      # Documentation
├── target/                    # Build artifacts (gitignored)
├── Cargo.toml                 # Rust package configuration
└── Configuration files...
```

## Configuration Files

### Rust Toolchain & Build

- **`rust-toolchain.toml`**: Specifies stable Rust channel with rustfmt/clippy
- **`rustfmt.toml`**: Code formatting configuration (2024 edition)
- **`Cargo.toml`**: Package metadata, dependencies, binary targets
- **`Cargo.lock`**: Dependency version lock file

### Code Quality & Standards

- **`.editorconfig`**: Editor settings (4-space Rust, 2-space markdown/yaml)
- **Linting**: Enforced via clippy with `-D warnings`
- **Formatting**: Automatic via rustfmt with 2024 edition style

### Documentation

- **`mkdocs.yml`**: Documentation site configuration
- **Material theme**: Modern documentation with dark/light modes
- **Mermaid support**: For architecture diagrams

## Code Organization Patterns

### Binary Structure

Each binary (client/implant) follows this pattern:

```
src/{binary}/
├── main.rs          # CLI parsing, initialization, main loop
└── mod.rs           # Core implementation logic
```

### Shared Code

- **`src/lib.rs`**: Common utilities and protocol definitions
- **`src/transport/`**: Transport layer abstraction using `network-protocol` crate
- **`src/protocol/`**: Message handling with PacketCodec, ECDH handshake, heartbeat
- **`src/crypto/`**: Additional crypto beyond network-protocol (if needed)
- **`src/shell/`**: Interactive shell and PTY handling
- **`src/transfer/`**: Secure file transfer with integrity verification
- **`src/client/ui/`**: TUI components and interface management
- **Error types**: Centralized error handling with thiserror

### Testing Structure

- **Unit tests**: Alongside source files (`#[cfg(test)]`)
- **Integration tests**: In `tests/` directory
- **Safety focus**: All tests use localhost, non-privileged execution

## Development Conventions

### File Naming

- **Snake_case**: For file and directory names
- **Kebab-case**: For binary names (`ghost-client`, `ghost-implant`)
- **CamelCase**: For Rust types and structs
- **SCREAMING_SNAKE_CASE**: For constants

### Module Organization

```rust
// Standard import order
use crate::protocol::Message;
use std::collections::HashMap; // Standard library
use tokio::net::TcpStream; // External crates // Internal modules
```

### Documentation Standards

- **Module docs**: `//!` for module-level documentation
- **Function docs**: `///` for public APIs
- **Examples**: Include usage examples in doc comments
- **Safety notes**: Document security considerations

## Architecture Patterns

### Client-Server Model

- **Multi-Protocol Support**:
  - **TCP**: Traditional reliable transport
  - **QUIC**: Modern transport with built-in encryption, multiplexing, 0-RTT
- **Dual Connection Modes**:
  - **Call-in**: Client connects to listening implant (traditional)
  - **Callback**: Implant connects to listening client (firewall evasion)
- **Interactive Shell**: Full PTY support with terminal emulation
- **Protocol**: ChaCha20-Poly1305 AEAD with X25519 key exchange
- **Authentication**: Ephemeral key exchange with pre-shared key verification
- **QUIC Features**: Multiple concurrent streams, connection migration, improved NAT traversal

### Error Handling Strategy

- **Result types**: All fallible operations return `Result<T, E>`
- **Error propagation**: Use `?` operator for clean error handling
- **User-friendly errors**: Clear error messages for educational use

### Safety Boundaries

- **Network binding**: Hardcoded to localhost (127.0.0.1) only
- **Command validation**: Input sanitization and length limits
- **Resource limits**: Timeouts, output size limits, memory bounds
- **Privilege separation**: No root/admin functionality

## Build Artifacts

### Target Directory Structure

```
target/
├── debug/                  # Development builds
│   ├── ghost-client       # Debug client binary
│   └── ghost-implant      # Debug implant binary
└── release/               # Optimized builds
    ├── ghost-client       # Release client binary
    └── ghost-implant      # Release implant binary
```

### Deployment Considerations

- **Single binaries**: Self-contained executables
- **Cross-platform**: Unix-like systems (Linux, macOS, WSL)
- **Educational packaging**: Include warnings and documentation

## Development Workflow Integration

### Local Development

1. **Edit**: Use any editor with EditorConfig support
2. **Build**: `cargo build` for development
3. **Test**: `cargo test` with safety constraints
4. **Format**: `cargo fmt` before commits
5. **Lint**: `cargo clippy` for code quality

### Safety Checklist

- [ ] All network operations use localhost only
- [ ] No external dependencies for core functionality
- [ ] Clear educational warnings in binaries
- [ ] Non-privileged execution required
- [ ] No persistence or stealth features
