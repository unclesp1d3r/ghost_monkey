![GitHub](https://img.shields.io/github/license/unclesp1d3r/ghost_monkey) ![GitHub commit activity](https://img.shields.io/github/commit-activity/m/unclesp1d3r/ghost_monkey)

# Ghost_Monkey

Ghost_Monkey is an educational UNIX backdoor written in Rust, designed for authorized penetration testing and OSCP preparation. This tool implements a simple client-server architecture using TCP sockets for remote command execution.

## Overview

Ghost_Monkey provides a lightweight backdoor implementation that demonstrates fundamental concepts in network programming and remote access techniques. The project serves as an educational resource for understanding backdoor mechanisms in controlled, authorized environments.

### Key Features

- **Simple TCP Protocol**: Unauthenticated socket-based communication
- **Cross-Platform**: Built with Rust for Unix-like systems
- **Educational Focus**: Clean, readable code for learning purposes
- **Minimal Dependencies**: Uses only standard Rust libraries

## Architecture

The system consists of two main components:

```mermaid
sequenceDiagram
    participant Operator as Operator
    participant Client as client
    participant Implant as implant
    participant OS as Target System

    Operator->>Client: Enter command
    Client->>Implant: TCP connect (127.0.0.1:5555)
    Client->>Implant: Send command string
    Implant->>OS: Execute via std::process::Command
    OS-->>Implant: Return stdout/stderr
    Implant-->>Client: Send response
    Client-->>Operator: Display output
```

- **client**: Interactive socket client for command input
- **implant**: Socket server that executes commands via `std::process::Command`

## Requirements

- Rust ≥ 1.70
- Unix-like or Windows

## Installation

1. **Install Rust** (if not already installed):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Build the project**:

   ```bash
   cargo build
   ```

## Usage

### Starting the Implant

```bash
./target/debug/implant [port]  # Default port: 5555
```

### Connecting with Client

```bash
./target/debug/client  # Connects to 127.0.0.1:5555
```

### Example Session

```bash
$ ./target/debug/implant
Listening on 127.0.0.1:5555

$ ./target/debug/client
Connected to 127.0.0.1:5555
> whoami
user
> pwd
/home/user
> exit
```

## Security Considerations

- **No Authentication**: The protocol is unauthenticated by design
- **Plain Text**: Communications are not encrypted
- **Local Testing**: Always bind to 127.0.0.1 for testing
- **Non-Privileged**: Run as non-root user for safety

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing

Run tests in isolated environments only:

```bash
# Run unit tests
cargo test

# Manual integration testing
./target/debug/implant &
IMPLANT_PID=$!
./target/debug/client
# Test with benign commands
kill $IMPLANT_PID
```

## Project Structure

```text
ghost_monkey/
├── src/
│   ├── client.rs       # Socket client implementation
│   ├── implant.rs      # Socket server implementation
│   └── lib.rs          # Library root
├── tests/
│   ├── integration_tests.rs # Integration tests
│   └── unit_tests.rs   # Unit tests
├── docs/
│   └── index.md        # Documentation
├── Cargo.toml          # Rust package configuration
├── CONTRIBUTING.md     # Contribution guidelines
└── README.md           # This file
```

## Contributing

This project follows specific development patterns and safety guidelines. See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.

## License

This project is licensed under the terms specified in [LICENSE](LICENSE).

## Disclaimer

This tool is designed for educational purposes, authorized penetration testing, and OSCP preparation. Users are responsible for ensuring they have proper authorization before using this tool in any environment. Use responsibly and in accordance with applicable laws and regulations.

## Authors

- [@UncleSp1d3r](https://www.github.com/unclesp1d3r)
