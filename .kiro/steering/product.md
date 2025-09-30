# Product Overview

Ghost_Monkey is an educational UNIX backdoor written in Rust, designed specifically for authorized penetration testing and OSCP preparation. The project serves as a learning resource for understanding backdoor mechanisms in controlled, authorized environments.

## Core Purpose

- **Educational Tool**: Demonstrates fundamental network programming and remote access techniques
- **OSCP Preparation**: Helps students understand backdoor concepts for certification
- **Authorized Testing**: Designed for legitimate penetration testing scenarios only
- **Comprehensive Documentation**: Everything must be clearly documented with extensive code comments and an associated mdbook guide

## Key Characteristics

- **Modern TUI Interface**: Rich terminal user interface built with ratatui
  - **Multi-pane Layout**: Separate panes for shell, file transfer, logs, and connection status
  - **Real-time Updates**: Live connection status, command output, and system metrics
  - **Interactive Navigation**: Keyboard shortcuts and mouse support for efficient operation
  - **Visual Feedback**: Progress bars, status indicators, and syntax highlighting
- **Multi-Protocol Support**: TCP and QUIC transport protocols
- **Call-in Mode**: Traditional client connects to listening implant
- **Callback Mode**: Implant connects back to listening client (firewall evasion)
- **QUIC Benefits**: Built-in encryption, multiplexing, 0-RTT connections, NAT traversal
- Modern cryptographic protocols (ChaCha20-Poly1305, X25519 key exchange)
- Interactive shell with full terminal emulation and PTY support
- Secure file transfer with integrity verification and parallel streams
- Cross-platform compatibility for Unix-like systems

## Safety Guidelines

**Educational Purpose**: This tool is designed for learning and authorized penetration testing.

**Recommended Practices**:

- Start with localhost (127.0.0.1) connections for initial learning and testing
- Use isolated virtual machines or lab environments for experimentation
- Run as non-privileged user when possible for safety
- Understand the ethical and legal implications before use
- Follow responsible disclosure practices if vulnerabilities are discovered
- Focus on educational features rather than evasion or persistence mechanisms

## Documentation Standards

**Top Priority**: Clear, comprehensive documentation is a primary goal of Ghost_Monkey.

### Code Documentation

- **Universal Rust Doc Comments**: All functions (public and private) should use `///` doc comments
  - **Public APIs**: Must include examples, error conditions, and usage patterns
  - **Private Functions**: Should explain purpose, parameters, and implementation approach
  - **Internal Logic**: Complex algorithms and cryptographic operations need detailed explanations
- **Module Documentation**: Every module documented with `//!` explaining its role and concepts
- **Architecture Explanations**: Complex networking and cryptographic code must explain the "why" not just the "what"
- **Educational Focus**: Comments should teach concepts, algorithms, and design decisions
- **Code Examples**: Include `# Examples` sections in doc comments wherever helpful for learning

### mdbook Guide

- **Comprehensive Tutorial**: Step-by-step guide covering all aspects of the project
- **Conceptual Explanations**: Deep dives into networking protocols, cryptography, and security concepts
- **Practical Examples**: Real-world usage scenarios and educational exercises
- **Cross-Platform Instructions**: Building and running on different operating systems
- **Security Best Practices**: Ethical guidelines and responsible use documentation

### Documentation Structure

```
docs/
├── book/                   # mdbook source
│   ├── src/
│   │   ├── SUMMARY.md      # Table of contents
│   │   ├── introduction.md
│   │   ├── ch01-getting-started/
│   │   │   ├── README.md
│   │   │   ├── installation.md
│   │   │   ├── first-run.md
│   │   │   └── safety-guidelines.md
│   │   ├── ch02-architecture/
│   │   │   ├── README.md
│   │   │   ├── overview.md
│   │   │   ├── client-implant.md
│   │   │   ├── connection-modes.md
│   │   │   └── transport-layer.md
│   │   ├── ch03-networking/
│   │   │   ├── README.md
│   │   │   ├── tcp-fundamentals.md
│   │   │   ├── quic-protocol.md
│   │   │   ├── network-protocol-crate.md
│   │   │   └── firewall-evasion.md
│   │   ├── ch04-cryptography/
│   │   │   ├── README.md
│   │   │   ├── encryption-basics.md
│   │   │   ├── chacha20-poly1305.md
│   │   │   ├── key-exchange.md
│   │   │   └── authentication.md
│   │   ├── ch05-implementation/
│   │   │   ├── README.md
│   │   │   ├── code-walkthrough.md
│   │   │   ├── client-implementation.md
│   │   │   ├── implant-implementation.md
│   │   │   └── protocol-handling.md
│   │   ├── ch06-cross-platform/
│   │   │   ├── README.md
│   │   │   ├── cargo-zigbuild.md
│   │   │   ├── alternative-tools.md
│   │   │   ├── platform-specific.md
│   │   │   └── deployment.md
│   │   ├── ch07-security/
│   │   │   ├── README.md
│   │   │   ├── threat-model.md
│   │   │   ├── defensive-measures.md
│   │   │   ├── detection-evasion.md
│   │   │   └── ethical-considerations.md
│   │   ├── ch08-exercises/
│   │   │   ├── README.md
│   │   │   ├── basic-usage.md
│   │   │   ├── protocol-analysis.md
│   │   │   ├── custom-modifications.md
│   │   │   └── oscp-scenarios.md
│   │   ├── ch09-advanced/
│   │   │   ├── README.md
│   │   │   ├── extending-protocols.md
│   │   │   ├── performance-tuning.md
│   │   │   ├── debugging-techniques.md
│   │   │   └── contributing.md
│   │   └── appendices/
│   │       ├── glossary.md
│   │       ├── references.md
│   │       ├── troubleshooting.md
│   │       └── legal-disclaimer.md
│   └── book.toml
└── api/                    # Generated API docs (cargo doc)
```

## Project Evolution

Originally based on `ghost_shell` Rust project, later ported to Nim for learning, now returning to Rust with improvements and lessons learned from the Nim implementation.
