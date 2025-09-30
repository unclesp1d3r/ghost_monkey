# Contributing to Ghost_Monkey

Thank you for your interest in contributing to Ghost_Monkey! This document outlines the development process, coding standards, and safety guidelines for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Security Considerations](#security-considerations)
- [Pull Request Process](#pull-request-process)
- [Development Workflow](#development-workflow)

## Code of Conduct

This project is committed to providing a welcoming and educational environment. All contributors are expected to:

- Be respectful and constructive in all interactions
- Focus on educational value and code quality
- Follow responsible disclosure practices
- Maintain the project's educational mission

## Getting Started

### Prerequisites

- Nim ≥ 2.0
- Git
- Unix-like operating system (Linux, macOS, WSL)
- Basic understanding of network programming concepts

### Setting Up Development Environment

1. **Fork and clone the repository**:

   ```bash
   git clone https://github.com/your-username/ghost_monkey.git
   cd ghost_monkey
   ```

2. **Install dependencies**:

   ```bash
   nimble install strenc
   ```

3. **Build the project**:

   ```bash
   nimble build
   ```

4. **Verify installation**:

   ```bash
   ./implant &
   ./client
   # Test with benign commands like 'whoami', 'pwd'
   ```

## Development Environment

### Recommended Tools

- **Editor**: VS Code with Nim extension
- **Terminal**: Any POSIX-compliant shell
- **Version Control**: Git with conventional commits

### Project Structure

```text
ghost_monkey/
├── src/
│   ├── client.nim      # Socket client implementation
│   └── implant.nim     # Socket server implementation
├── tests/
│   ├── test_client.nim     # Client unit tests
│   ├── test_implant.nim    # Implant unit tests
│   └── test_integration.nim # Integration tests
├── docs/
│   └── ARCHITECTURE.md # Detailed architecture documentation
├── ghost_monkey.nimble # Nim package configuration
├── AGENTS.md           # Detailed development guidelines
├── CONTRIBUTING.md     # This file
├── README.md           # Project overview
└── LICENSE             # Project license
```

## Coding Standards

### Nim Style Guidelines

- **Indentation**: 2 spaces (no tabs)
- **Naming**: camelCase for variables, PascalCase for types
- **Comments**: Use `#` for single-line, `##` for documentation
- **Line Length**: Keep under 100 characters when possible

### Code Organization

```nim
# Module-level documentation
## This module handles socket communication
import std/[net, os, strutils]

# Type definitions
type
  Client* = object
    socket*: Socket
    connected*: bool

# Function documentation
proc connect*(client: var Client, host: string, port: int): bool =
  ## Connects to the specified host and port
  # Implementation here
```

### Error Handling

Always handle errors gracefully:

```nim
try:
  result = execProcess(command)
except OSError as e:
  stderr.writeLine("Error executing command: " & e.msg)
  result = ""
```

## Testing Guidelines

### Safety First

- **Never test on production systems**
- **Always use loopback (127.0.0.1) for testing**
- **Run as non-privileged user**
- **Use benign commands only** (`whoami`, `pwd`, `ls`)

### Test Environment Setup

```bash
# Create isolated test environment
mkdir -p test_env
cd test_env

# Copy binaries to test directory
cp ../client ../implant .

# Run tests
./implant &
IMPLANT_PID=$!
./client
# Test with: whoami, pwd, ls
kill $IMPLANT_PID
```

### Test Cases

1. **Connection Tests**:

   - Successful connection to implant
   - Connection failure handling
   - Socket cleanup on disconnect

2. **Command Execution Tests**:

   - Basic command execution (`whoami`, `pwd`)
   - Error handling for invalid commands
   - Output capture and display

3. **Protocol Tests**:

   - Message format validation
   - Connection state management
   - Graceful shutdown

## Security Considerations

### Development Safety

- **No External Network Access**: All development must use loopback interfaces
- **No Privilege Escalation**: Avoid adding root/sudo functionality
- **No Persistence**: Don't add startup scripts or service installation
- **No Stealth Features**: Keep the tool visible and detectable

### Code Review Security Checklist

- [ ] No hardcoded credentials or sensitive data
- [ ] Proper input validation and sanitization
- [ ] No external network connections
- [ ] No file system persistence
- [ ] No privilege escalation attempts
- [ ] Clear error messages without information disclosure

## Pull Request Process

### Before Submitting

1. **Test your changes thoroughly**:

   ```bash
   nimble build
   nimble test
   # Run integration tests
   ```

2. **Check code quality**:

   ```bash
   nim check src/client.nim
   nim check src/implant.nim
   ```

3. **Clean up build artifacts**:

   ```bash
   rm -rf nimcache/
   ```

### Pull Request Guidelines

1. **Use descriptive titles**: `feat: add connection timeout handling`
2. **Provide clear descriptions**: Explain what changed and why
3. **Include test results**: Show that your changes work correctly
4. **Keep PRs focused**: One feature or fix per PR
5. **Update documentation**: If adding new features

### Commit Message Format

Use conventional commits:

```text
feat: add connection timeout handling
fix: resolve socket cleanup issue
docs: update installation instructions
test: add integration test for client connection
```

## Development Workflow

### Feature Development

1. **Create feature branch**:

   ```bash
   git checkout -b feature/connection-timeout
   ```

2. **Make changes**:

   - Write code following style guidelines
   - Add appropriate tests
   - Update documentation if needed

3. **Test thoroughly**:

   - Run all tests in isolated environment
   - Verify no external network access
   - Check for proper error handling

4. **Submit PR**:

   - Create pull request with clear description
   - Include test results and screenshots if relevant
   - Request review from maintainers

### Bug Fixes

1. **Identify the issue**: Document the problem clearly
2. **Create fix branch**: `git checkout -b fix/socket-cleanup`
3. **Implement fix**: Follow coding standards
4. **Test fix**: Verify the issue is resolved
5. **Submit PR**: Include issue reference and test results

### Documentation Updates

- Update README.md for user-facing changes
- Update AGENTS.md for development process changes
- Add inline code documentation for complex logic
- Include usage examples for new features

## Review Process

### What Reviewers Look For

- **Code Quality**: Clean, readable, well-documented code
- **Security**: No unsafe practices or external dependencies
- **Testing**: Adequate test coverage and proper test environment
- **Documentation**: Clear explanations and updated docs
- **Educational Value**: Maintains the project's learning focus

### Review Timeline

- **Initial Review**: Within 48 hours
- **Follow-up**: Within 24 hours of requested changes
- **Final Approval**: Maintainer approval required

## Getting Help

- **Documentation**: Check AGENTS.md for detailed guidelines
- **Issues**: Use GitHub issues for questions and bug reports
- **Discussions**: Use GitHub discussions for general questions
- **Code Review**: All PRs receive thorough review and feedback

## Recognition

Contributors will be recognized in:

- README.md contributors section
- Release notes for significant contributions
- Project documentation where appropriate

Thank you for contributing to Ghost_Monkey and helping make it a valuable educational resource!
