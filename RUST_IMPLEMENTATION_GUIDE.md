# Ghost_Monkey Rust Implementation Guide

## Quick Start: Returning to Rust Roots

Since Ghost_Monkey is based on your original `ghost_shell` Rust project, this guide provides practical steps to implement the Rust version by leveraging the existing `ghost_shell` codebase as a foundation.

## Phase 1: Project Setup and Migration

### 1. Initialize New Rust Project Structure

```bash
# Create new Rust workspace (alongside existing Nim version)
mkdir -p ghost_monkey_rust
cd ghost_monkey_rust

# Initialize Cargo workspace
cargo init --name ghost_monkey --bin
```

### 2. Set Up Cargo.toml Based on Original ghost_shell

```toml
[package]
name = "ghost_monkey"
version = "0.2.0"
edition = "2021"
authors = ["UncleSp1d3r"]
description = "Educational backdoor for authorized penetration testing (back to Rust roots)"
license = "MIT"
repository = "https://github.com/unclesp1d3r/ghost_monkey"
keywords = ["security", "educational", "backdoor", "penetration-testing"]
categories = ["command-line-utilities"]

[[bin]]
name = "ghost-client"
path = "src/client/main.rs"

[[bin]]
name = "ghost-implant"  
path = "src/implant/main.rs"

[dependencies]
# Core networking and async
tokio = { version = "1.0", features = ["full"] }
tokio-util = { version = "0.7", features = ["codec"] }

# CLI and argument parsing
clap = { version = "4.4", features = ["derive"] }

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Serialization (if needed for protocol)
serde = { version = "1.0", features = ["derive"] }

[dev-dependencies]
tokio-test = "0.4"
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.0"

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### 3. Create Project Structure

```bash
mkdir -p src/{client,implant,protocol}
mkdir -p {tests,examples,docs,scripts}

# Create main entry points
touch src/lib.rs
touch src/client/{main.rs,mod.rs}
touch src/implant/{main.rs,mod.rs}
touch src/protocol/{mod.rs,message.rs}
```

## Phase 2: Reference Original ghost_shell Implementation

### 1. Analyze Original ghost_shell Code

```bash
# Clone or reference the original ghost_shell repository
# (This provides the architectural foundation)
git clone https://github.com/unclesp1d3r/ghost_shell.git /tmp/ghost_shell_reference

# Review the key components:
# - Client implementation patterns
# - Server/implant architecture  
# - Protocol definition
# - Error handling strategies
```

### 2. Core Protocol Module (src/protocol/mod.rs)

Based on the Nim analysis and ghost_shell patterns:

```rust
//! Protocol definitions for Ghost_Monkey
//! 
//! Maintains compatibility with existing Nim implementation while
//! improving type safety and error handling.

pub mod message;

use std::io;
use thiserror::Error;

pub const PROTOCOL_VERSION: u8 = 1;
pub const MESSAGE_DELIMITER: &str = "\r\n";
pub const END_OF_RESPONSE: &str = "\r\n";
pub const MAX_COMMAND_LENGTH: usize = 8192;
pub const MAX_RESPONSE_LENGTH: usize = 1048576; // 1MB

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Invalid message format")]
    InvalidFormat,
    #[error("Message too large: {0} bytes")]
    MessageTooLarge(usize),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Connection closed")]
    ConnectionClosed,
}

pub type ProtocolResult<T> = Result<T, ProtocolError>;
```

### 3. Message Types (src/protocol/message.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    /// Command to execute
    Command(String),
    /// Response from command execution
    Response(String),
    /// Error message
    Error(String),
    /// Client quit signal
    Quit,
}

impl Message {
    /// Serialize message for wire protocol
    pub fn to_wire_format(&self) -> Vec<u8> {
        match self {
            Message::Command(cmd) => format!("{}\r\n", cmd).into_bytes(),
            Message::Response(resp) => format!("{}\r\n", resp).into_bytes(),
            Message::Error(err) => format!("ERROR: {}\r\n", err).into_bytes(),
            Message::Quit => b"quit\r\n".to_vec(),
        }
    }
    
    /// Parse message from wire format
    pub fn from_wire_format(data: &str) -> crate::protocol::ProtocolResult<Self> {
        let trimmed = data.trim_end_matches("\r\n");
        
        if trimmed == "quit" {
            Ok(Message::Quit)
        } else if trimmed.starts_with("ERROR: ") {
            Ok(Message::Error(trimmed[7..].to_string()))
        } else if data.ends_with("\r\n") {
            // Assume it's a command or response
            Ok(Message::Command(trimmed.to_string()))
        } else {
            Err(crate::protocol::ProtocolError::InvalidFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization_roundtrip() {
        let messages = vec![
            Message::Command("echo hello".to_string()),
            Message::Response("hello\n".to_string()),
            Message::Error("command failed".to_string()),
            Message::Quit,
        ];

        for msg in messages {
            let wire = msg.to_wire_format();
            let wire_str = String::from_utf8(wire).unwrap();
            let parsed = Message::from_wire_format(&wire_str).unwrap();
            
            match (&msg, &parsed) {
                (Message::Command(a), Message::Command(b)) => assert_eq!(a, b),
                (Message::Response(a), Message::Command(b)) => assert_eq!(a, b), // Response parsed as Command
                (Message::Error(a), Message::Error(b)) => assert_eq!(a, b),
                (Message::Quit, Message::Quit) => (),
                _ => panic!("Message roundtrip failed: {:?} -> {:?}", msg, parsed),
            }
        }
    }
}
```

### 4. Client Implementation (src/client/main.rs)

```rust
//! Ghost_Monkey Client
//! 
//! Interactive client for connecting to ghost_monkey implant.
//! Based on original ghost_shell architecture with improvements.

use clap::Parser;
use std::time::Duration;
use tracing::{info, error, debug};

mod client_impl;
use client_impl::GhostClient;

#[derive(Parser)]
#[command(name = "ghost-client")]
#[command(about = "Educational backdoor client (authorized testing only)")]
#[command(version)]
struct Args {
    /// Target host
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: String,
    
    /// Target port
    #[arg(short, long, default_value = "5555")]
    port: u16,
    
    /// Connection timeout in seconds
    #[arg(short, long, default_value = "10")]
    timeout: u64,
    
    /// Custom prompt
    #[arg(long, default_value = "> ")]
    prompt: String,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    // Initialize logging
    let log_level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(log_level))
        .init();
        
    // Educational use warning
    println!("Ghost_Monkey Client v0.2.0");
    println!("⚠️  FOR EDUCATIONAL AND AUTHORIZED TESTING ONLY ⚠️");
    println!("   User is responsible for lawful use with explicit consent");
    println!();
    
    info!("Connecting to {}:{}", args.host, args.port);
    
    // Validate connection target (security check)
    if args.host != "127.0.0.1" && args.host != "localhost" {
        error!("Security restriction: Only localhost connections allowed in educational mode");
        error!("Use --host 127.0.0.1 or --host localhost");
        std::process::exit(1);
    }
    
    let timeout = Duration::from_secs(args.timeout);
    let client = GhostClient::connect(&args.host, args.port, timeout).await?;
    
    info!("Connected successfully");
    client.run_interactive(&args.prompt).await?;
    
    Ok(())
}
```

### 5. Client Implementation (src/client/mod.rs)

```rust
//! Client implementation module

use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use std::time::Duration;
use anyhow::{Context, Result};
use tracing::{debug, info, warn};

pub struct GhostClient {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl GhostClient {
    pub async fn connect(host: &str, port: u16, timeout: Duration) -> Result<Self> {
        let stream = tokio::time::timeout(
            timeout,
            TcpStream::connect(format!("{}:{}", host, port))
        ).await
        .context("Connection timeout")?
        .context("Failed to connect")?;
        
        debug!("TCP connection established");
        
        // Split stream for reading and writing
        let (read_half, write_half) = stream.into_split();
        let reader = BufReader::new(TcpStream::from_split(read_half, write_half.clone()));
        let writer = BufWriter::new(TcpStream::from_split(read_half.clone(), write_half));
        
        Ok(GhostClient { reader, writer })
    }
    
    pub async fn run_interactive(&mut self, prompt: &str) -> Result<()> {
        info!("Starting interactive session. Type 'quit' to exit.");
        
        loop {
            // Display prompt
            print!("{}", prompt);
            use std::io::{self, Write};
            io::stdout().flush()?;
            
            // Read user input
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let command = input.trim();
            
            // Handle quit command
            if command == "quit" || command == "exit" {
                info!("Quit command received");
                break;
            }
            
            // Skip empty commands
            if command.is_empty() {
                continue;
            }
            
            // Send command and get response
            match self.execute_command(command).await {
                Ok(response) => {
                    if !response.trim().is_empty() {
                        println!("{}", response);
                    }
                },
                Err(e) => {
                    warn!("Command execution failed: {}", e);
                    println!("Error: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn execute_command(&mut self, command: &str) -> Result<String> {
        debug!("Sending command: {}", command);
        
        // Send command
        self.writer.write_all(command.as_bytes()).await?;
        self.writer.write_all(b"\r\n").await?;
        self.writer.flush().await?;
        
        // Read response (possibly multi-line)
        let mut response = String::new();
        loop {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line).await?;
            
            if bytes_read == 0 {
                return Err(anyhow::anyhow!("Connection closed"));
            }
            
            // Check for end of response marker
            if line == "\r\n" {
                break;
            }
            
            response.push_str(&line);
        }
        
        Ok(response)
    }
}
```

## Phase 3: Integration with Existing Project

### 1. Update Existing justfile for Rust Support

Add to the existing justfile:

```make
# Rust-specific recipes (add to existing justfile)

# Setup Rust development environment  
setup-rust:
    rustup update stable
    rustup default stable
    rustup component add clippy rustfmt

# Build Rust version
build-rust:
    cd ghost_monkey_rust && cargo build

# Build Rust release version
build-rust-release:
    cd ghost_monkey_rust && cargo build --release

# Test Rust implementation
test-rust:
    cd ghost_monkey_rust && cargo test

# Lint Rust code
lint-rust:
    cd ghost_monkey_rust && cargo clippy -- -D warnings
    cd ghost_monkey_rust && cargo fmt --check

# Run Rust integration test
test-rust-local:
    @just build-rust
    cd ghost_monkey_rust && ./scripts/test_local.sh

# Compare Nim vs Rust implementations
compare-implementations:
    echo "Building both versions for comparison..."
    @just build      # Nim version
    @just build-rust # Rust version
    echo "Both versions ready for testing"
```

### 2. Migration Testing Strategy

Create `scripts/migration_test.sh`:

```bash
#!/usr/bin/env bash
# Test compatibility between Nim and Rust implementations

set -euo pipefail

echo "Testing Ghost_Monkey Nim->Rust migration compatibility..."

# Build both versions
echo "Building Nim version..."
just build

echo "Building Rust version..."
just build-rust

# Test 1: Nim client -> Rust implant
echo "Test 1: Nim client -> Rust implant compatibility"
cd ghost_monkey_rust
./target/debug/ghost-implant &
RUST_IMPLANT_PID=$!
sleep 1

cd ..
echo "echo 'Hello from Nim client'" | timeout 5 ./client || true
kill $RUST_IMPLANT_PID 2>/dev/null || true

# Test 2: Rust client -> Nim implant
echo "Test 2: Rust client -> Nim implant compatibility"
./implant &
NIM_IMPLANT_PID=$!
sleep 1

cd ghost_monkey_rust
echo "echo 'Hello from Rust client'" | timeout 5 ./target/debug/ghost-client || true
kill $NIM_IMPLANT_PID 2>/dev/null || true

echo "Migration compatibility tests completed"
```

### 3. Documentation Updates

Update the main README.md to reflect the dual nature:

```markdown
# Ghost_Monkey: Educational Backdoor

## Multi-Language Implementation

Ghost_Monkey is available in two implementations:

### Nim Implementation (Current/Legacy)
- Original port for Nim language learning
- Located in: `src/`
- Build: `just build`

### Rust Implementation (Returning to Roots)
- Based on original `ghost_shell` Rust project
- Enhanced with lessons learned from Nim version
- Located in: `ghost_monkey_rust/`
- Build: `just build-rust`

Both implementations maintain protocol compatibility for educational comparison.
```

## Phase 4: Implementation Priorities

### Week 1: Foundation
- [x] Set up Rust project structure
- [ ] Implement basic protocol module
- [ ] Create minimal client with REPL
- [ ] Create minimal implant with command execution

### Week 2: Feature Parity
- [ ] Add CLI argument parsing
- [ ] Implement error handling
- [ ] Add logging and debugging
- [ ] Protocol compatibility testing

### Week 3: Enhancement
- [ ] Security hardening
- [ ] Performance optimization
- [ ] Comprehensive test suite
- [ ] Documentation completion

### Week 4: Integration
- [ ] Migration testing between implementations
- [ ] Cross-platform builds
- [ ] Release preparation
- [ ] Educational materials

## Reference Resources

### Original ghost_shell Project
- **Crates.io**: https://crates.io/crates/ghost_shell
- **Repository**: https://github.com/unclesp1d3r/ghost_shell
- **Description**: "A lightweight and fast remote shell that provides secure communication between a client and a server"

### Key Differences for Educational Focus
1. **Explicit Educational Context**: Clear warnings and ethical boundaries
2. **Localhost Restriction**: Hardcoded safety for educational environments
3. **Enhanced Documentation**: Learning-focused explanations
4. **Comprehensive Testing**: Educational examples and test cases

This implementation guide provides a practical roadmap for returning Ghost_Monkey to its Rust roots while preserving the educational value gained from the Nim exploration.