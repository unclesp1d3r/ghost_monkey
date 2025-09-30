# Safety Guidelines

⚠️ **Critical**: Ghost Monkey is an educational tool designed for authorized penetration testing and learning purposes only. This chapter outlines essential safety guidelines that must be followed when using this tool.

**Current Status**: The project is in early development with placeholder implementations. These guidelines apply to both current development and future usage.

## Legal and Ethical Requirements

### Authorization is Mandatory

- **Never use Ghost Monkey without explicit written authorization**
- Only use on systems you own or have explicit permission to test
- Ensure you have proper legal documentation before any testing
- Understand that unauthorized use may violate local, state, and federal laws

### Educational Context Only

- Use Ghost Monkey only for learning network security concepts
- Employ it in controlled lab environments or personal systems
- Focus on understanding the underlying technologies and protocols
- Do not use for any malicious or unauthorized activities

## Technical Safety Measures

### Network Isolation

```bash
# Always start with localhost testing
./target/release/ghost-implant --listen --bind 127.0.0.1:8080
./target/release/ghost-client --connect 127.0.0.1:8080

# Use isolated networks for advanced testing
# Set up a dedicated lab network: 192.168.100.0/24
```

### Virtual Machine Isolation

Recommended VM setup for safe testing:

```bash
# Create isolated VMs with no external network access
# Example using VirtualBox:
VBoxManage createvm --name "ghost-monkey-lab" --register
VBoxManage modifyvm "ghost-monkey-lab" --memory 2048 --cpus 2
VBoxManage modifyvm "ghost-monkey-lab" --nic1 intnet --intnet1 "ghost-lab"
```

### User Privileges

```bash
# Always run as non-privileged user
# Create dedicated test user
sudo useradd -m -s /bin/bash ghosttest
sudo su - ghosttest

# Verify you're not running as root
whoami  # Should NOT return 'root'
id      # Should show non-zero UID
```

## Built-in Safety Features

### Educational Warnings

Ghost Monkey displays warnings on startup:

```
[EDUCATIONAL WARNING] This tool is for authorized testing only
[SAFETY NOTICE] Ensure you have proper authorization before use
[LEGAL REMINDER] Unauthorized access to computer systems is illegal
```

### Command Restrictions

The implant has built-in command restrictions:

- **Initial Implementation**: Only `ls` command is allowed
- **Input Validation**: Commands are sanitized and length-limited
- **No Privilege Escalation**: Runs with current user privileges only
- **Resource Limits**: Timeouts prevent resource exhaustion

### Network Binding Restrictions

```rust
// Example of safe binding configuration
let config = TransportConfig {
    bind_addr: "127.0.0.1:8080".parse().unwrap(), // Localhost only
    connect_timeout: Duration::from_secs(30),      // Reasonable timeout
    io_timeout: Duration::from_secs(60),           // Prevent hanging
    max_packet_size: 1024 * 1024,                 // 1MB limit
    ..Default::default()
};
```

## Recommended Testing Environments

### Home Lab Setup

1. **Isolated Network**: Use a dedicated subnet with no internet access
2. **Virtual Machines**: Run both client and implant in separate VMs
3. **Snapshot Management**: Take VM snapshots before testing
4. **Network Monitoring**: Use Wireshark to analyze traffic

### Cloud Lab Environment

```bash
# Example using AWS with proper isolation
# Create VPC with no internet gateway
aws ec2 create-vpc --cidr-block 10.0.0.0/16

# Create isolated subnets
aws ec2 create-subnet --vpc-id vpc-xxx --cidr-block 10.0.1.0/24
aws ec2 create-subnet --vpc-id vpc-xxx --cidr-block 10.0.2.0/24

# Launch instances in isolated subnets
# No NAT gateway or internet gateway attached
```

### Container Isolation

```dockerfile
# Example Docker setup for isolated testing
FROM rust:1.85-slim

# Create non-root user
RUN useradd -m -s /bin/bash ghosttest
USER ghosttest

# Copy and build Ghost Monkey
COPY --chown=ghosttest:ghosttest . /app
WORKDIR /app
RUN cargo build --release

# Run with network isolation
# docker run --network none ghost-monkey-test
```

## Monitoring and Logging

### Enable Comprehensive Logging

```bash
# Set environment variables for detailed logging (when implemented)
export RUST_LOG=debug
export GHOST_MONKEY_LOG_LEVEL=trace

# Current usage (placeholder implementations)
./target/release/ghost-client 2>&1 | tee client.log
./target/release/ghost-implant 2>&1 | tee implant.log

# Planned usage (future implementation)
# ./target/release/ghost-client --connect 127.0.0.1:8080 2>&1 | tee client.log
# ./target/release/ghost-implant --listen --port 8080 2>&1 | tee implant.log
```

### Network Traffic Analysis

```bash
# Monitor network traffic during testing
sudo tcpdump -i lo -w ghost-monkey-traffic.pcap port 8080

# Analyze with Wireshark
wireshark ghost-monkey-traffic.pcap
```

## Incident Response

### If Something Goes Wrong

1. **Immediately Stop All Processes**:

   ```bash
   # Kill all Ghost Monkey processes
   pkill -f ghost-client
   pkill -f ghost-implant
   ```

2. **Document the Incident**:

   - Save all log files
   - Record exact commands used
   - Note any unexpected behavior

3. **Isolate Affected Systems**:

   - Disconnect from network if necessary
   - Take system snapshots for analysis

4. **Report if Required**:

   - Follow your organization's incident response procedures
   - Contact appropriate authorities if unauthorized access occurred

## Best Practices Checklist

Before using Ghost Monkey, verify:

- [ ] You have explicit written authorization for all target systems
- [ ] You understand the legal implications in your jurisdiction
- [ ] You're using isolated test environments
- [ ] You're running as a non-privileged user
- [ ] You have proper logging and monitoring in place
- [ ] You have an incident response plan
- [ ] You understand the tool's capabilities and limitations
- [ ] You've read and understood all documentation

## Educational Objectives

Remember that Ghost Monkey is designed to teach:

- **Network Security Concepts**: Understanding how secure communication works
- **Cryptographic Protocols**: Learning about modern encryption and key exchange
- **System Security**: Understanding privilege separation and access controls
- **Ethical Hacking**: Learning responsible disclosure and authorized testing practices

## Prohibited Uses

**Never use Ghost Monkey for**:

- Unauthorized access to any system
- Malicious activities or causing harm
- Violating privacy or confidentiality
- Circumventing security measures without authorization
- Any illegal activities under applicable laws

## Conclusion

Safety is paramount when working with security tools. By following these guidelines, you can learn effectively while maintaining ethical and legal compliance. Remember: with great power comes great responsibility.

Next: [Quick Start Tutorial](./quick-start.md) - Learn how to safely run Ghost Monkey for the first time.
