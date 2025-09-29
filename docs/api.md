# API Reference

This document provides detailed information about the Ghost_Monkey codebase structure and key functions.

## Project Structure

```
src/
├── client.nim          # Socket client implementation
└── implant.nim         # Socket server implementation

tests/
├── test_client.nim     # Client unit tests
├── test_implant.nim    # Implant unit tests
└── test_integration.nim # Integration tests
```

## Client API (src/client.nim)

The client component provides an interactive interface for command input and output display.

### Key Functions

#### Connection Management

- **Socket Creation**: Creates TCP socket for communication
- **Connection Establishment**: Connects to implant server
- **Connection Cleanup**: Properly closes socket connections

#### Command Handling

- **Input Processing**: Reads user commands from stdin
- **Command Transmission**: Sends commands to implant via socket
- **Output Display**: Receives and displays command output

#### Error Handling

- **Connection Errors**: Handles connection failures gracefully
- **Socket Errors**: Manages socket-related exceptions
- **User Input**: Validates and processes user commands

### Dependencies

```nim
import std/[net, os, strutils]
import strenc
```

### Key Variables

- **Socket**: TCP socket for communication
- **Host**: Server address (hardcoded to 127.0.0.1)
- **Port**: Server port (hardcoded to 5555)

## Implant API (src/implant.nim)

The implant component acts as a socket server that executes received commands.

### Key Functions

#### Server Management

- **Socket Server**: Creates and configures TCP server socket
- **Port Binding**: Binds to specified port (default 5555)
- **Connection Listening**: Listens for incoming client connections

#### Command Execution

- **Command Processing**: Receives commands from client
- **Process Execution**: Executes commands via `execProcess()`
- **Output Capture**: Captures stdout/stderr from executed commands

#### Connection Handling

- **Client Connection**: Accepts incoming client connections
- **Message Processing**: Handles command transmission
- **Response Sending**: Returns command output to client

### Dependencies

```nim
import std/[net, os, strutils, osproc]
import strenc
```

### Key Variables

- **Server Socket**: TCP server socket for listening
- **Client Socket**: Individual client connection socket
- **Port**: Listening port (default 5555)

## Protocol Specification

### Communication Flow

1. **Connection Establishment**
   - Client connects to implant on port 5555
   - Implant accepts connection and creates client socket

2. **Command Transmission**
   - Client sends command string via socket
   - Implant receives command string

3. **Command Execution**
   - Implant executes command using `execProcess()`
   - Captures stdout and stderr output

4. **Response Transmission**
   - Implant sends output back to client
   - Client receives and displays output

5. **Connection Cleanup**
   - Client sends "quit" command or closes connection
   - Implant closes client socket and waits for next connection

### Message Format

- **Commands**: Plain text strings
- **Responses**: Plain text output from command execution
- **Line Endings**: Uses `\r\n` for network protocol
- **Encoding**: No special encoding, plain text only

## Error Handling

### Client Error Handling

```nim
try:
  # Socket operations
except OSError as e:
  stderr.writeLine("Connection error: " & e.msg)
  # Handle connection failure
```

### Implant Error Handling

```nim
try:
  result = execProcess(command)
except OSError as e:
  stderr.writeLine("Command execution error: " & e.msg)
  result = ""
```

## Security Considerations

### Design Constraints

- **No Authentication**: Protocol is unauthenticated by design
- **No Encryption**: Communications are plain text
- **Local Testing Only**: Designed for loopback testing
- **No Persistence**: No startup scripts or service installation

### Safety Measures

- **Loopback Binding**: Always binds to 127.0.0.1
- **Non-Privileged Execution**: Runs as non-root user
- **Input Validation**: Basic input sanitization
- **Connection Cleanup**: Proper socket cleanup on disconnect

## Testing API

### Unit Tests

#### Client Tests (test_client.nim)

- **Binary Existence**: Verifies client.nim exists
- **Compilation**: Tests successful compilation
- **Basic Functionality**: Tests core client functions

#### Implant Tests (test_implant.nim)

- **Binary Existence**: Verifies implant.nim exists
- **Compilation**: Tests successful compilation
- **Basic Functionality**: Tests core implant functions

#### Integration Tests (test_integration.nim)

- **Component Compilation**: Tests both components compile
- **Environment Setup**: Verifies test environment
- **End-to-End Testing**: Tests client-implant communication

### Test Execution

```bash
# Run all tests
nimble test

# Run specific test
nim c -r tests/test_client.nim
```

## Build Configuration

### Nimble Package (ghost_monkey.nimble)

```nim
# Package metadata
version = "0.1.0"
author = "UncleSp1d3r"
description = "Educational UNIX backdoor written in Nim"
license = "MIT"

# Build configuration
srcDir = "src"
bin = @["client", "implant"]

# Dependencies
requires "nim >= 2.0.0"
requires "strenc"
```

### Build Commands

```bash
# Debug build
nimble build

# Release build
nimble build -d:release

# Static analysis
nim check src/client.nim
nim check src/implant.nim
```

## Development Guidelines

### Code Style

- **Indentation**: 2 spaces
- **Naming**: camelCase for variables, PascalCase for types
- **Comments**: Use `#` for single-line, `##` for documentation
- **Error Handling**: Use try/except blocks for error management

### Best Practices

- **Single Responsibility**: Each component has a clear purpose
- **Minimal Dependencies**: Use only necessary external packages
- **Clean Interfaces**: Simple, well-defined component interfaces
- **Error Handling**: Graceful error handling throughout

### Security Guidelines

- **No External Network Access**: All development must use loopback
- **No Privilege Escalation**: Avoid root/sudo functionality
- **No Persistence**: Don't add startup scripts
- **No Stealth Features**: Keep tool visible and detectable

This API reference provides comprehensive information about the Ghost_Monkey codebase structure, functions, and development guidelines.
