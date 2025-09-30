# Quick Start Guide

This guide provides a rapid overview of Ghost_Monkey's core functionality with practical examples for immediate hands-on learning.

## Prerequisites

- Ghost_Monkey built and installed (see [Installation](installation.md))
- Two terminal windows available
- Basic understanding of client-server architecture

## 5-Minute Quick Start

### 1. Basic Call-in Mode

**Terminal 1 - Start Implant:**

```bash
./target/release/ghost-implant --listen 127.0.0.1:8080
```

**Terminal 2 - Connect Client:**

```bash
./target/release/ghost-client 127.0.0.1:8080
```

**Execute Command:**

```bash
ghost> ls
```

### 2. Callback Mode (Firewall Evasion)

**Terminal 1 - Client Listens:**

```bash
./target/release/ghost-client --listen 127.0.0.1:8080
```

**Terminal 2 - Implant Connects Back:**

```bash
./target/release/ghost-implant --callback 127.0.0.1:8080
```

## Common Usage Patterns

### Interactive Session

```bash
ghost> ls
total 48
drwxr-xr-x  8 user user 4096 Jan 15 10:30 .
...

ghost> pwd
/home/user/ghost_monkey

ghost> whoami
user

ghost> exit
[INFO] Connection closed
```

### Connection Status

Monitor connection health:

```bash
ghost> status
[INFO] Connection: Active
[INFO] Encryption: ChaCha20-Poly1305
[INFO] Uptime: 00:02:34
[INFO] Commands executed: 3
```

## Command Reference

### Allowed Commands

- `ls` - Directory listing
- `pwd` - Current directory
- `whoami` - Current user
- `status` - Connection information
- `help` - Show available commands
- `exit` - Close connection

### Restricted Commands

All other commands are blocked for safety:

```bash
ghost> cat /etc/passwd
[ERROR] Command not allowed: cat
[INFO] Only basic commands permitted in educational version
```

## Configuration Options

### Client Options

```bash
# Connect mode (default)
./target/release/ghost-client <host>:<port>

# Listen mode (callback)
./target/release/ghost-client --listen <bind_addr>:<port>

# Custom timeout
./target/release/ghost-client --timeout 30 <host>:<port>

# Verbose logging
./target/release/ghost-client --verbose <host>:<port>
```

### Implant Options

```bash
# Listen mode (default)
./target/release/ghost-implant --listen <bind_addr>:<port>

# Callback mode
./target/release/ghost-implant --callback <host>:<port>

# Custom retry interval
./target/release/ghost-implant --retry-interval 5 --callback <host>:<port>
```

## Security Features in Action

### Encrypted Communication

All traffic is automatically encrypted:

```bash
[INFO] Performing ECDH key exchange...
[INFO] Secure channel established (ChaCha20-Poly1305)
[INFO] All communication now encrypted
```

### Authentication

Each session uses ephemeral keys:

```bash
[INFO] Generating ephemeral keypair...
[INFO] Exchanging public keys...
[INFO] Session authenticated
```

### Heartbeat Monitoring

Connection health is automatically monitored:

```bash
[DEBUG] Sending heartbeat...
[DEBUG] Heartbeat acknowledged
[INFO] Connection stable
```

## Troubleshooting Quick Fixes

### Connection Issues

```bash
# Check if port is available
netstat -ln | grep 8080

# Try different port
./target/release/ghost-implant --listen 127.0.0.1:9090
```

### Permission Issues

```bash
# Use unprivileged port
./target/release/ghost-implant --listen 127.0.0.1:8080

# Check binary permissions
ls -la target/release/ghost-*
```

### Firewall Issues

```bash
# Test local connectivity
telnet 127.0.0.1 8080

# Check firewall rules (Linux)
sudo iptables -L | grep 8080
```

## Next Steps

- **Detailed Walkthrough**: [First Run](first-run.md) for step-by-step instructions
- **Safety Guidelines**: [Safety Guidelines](safety-guidelines.md) for important security considerations
- **Architecture**: [System Overview](../ch02-architecture/overview.md) to understand how it works
- **Advanced Usage**: [Basic Usage](../ch08-exercises/basic-usage.md) for practical exercises

## Educational Value

This quick start demonstrates:

- **Modern Cryptography**: ChaCha20-Poly1305 AEAD encryption
- **Key Exchange**: X25519 ECDH for perfect forward secrecy
- **Network Programming**: Bidirectional TCP communication
- **Security Design**: Defense-in-depth with multiple safety layers
- **Cross-Platform**: Consistent behavior across Unix-like systems

Understanding these fundamentals through hands-on practice prepares you for advanced penetration testing concepts and OSCP scenarios covered in later chapters.
