# Usage

This guide covers how to use Ghost_Monkey for educational and authorized testing purposes.

## Basic Usage

### Starting the Implant

The implant acts as a socket server that listens for incoming connections:

```bash
./implant [port]  # Default port: 5555
```

**Example:**

```bash
$ ./implant
Server: started. Listening to new connections on port 5555...
```

### Connecting with Client

The client provides an interactive interface for command execution:

```bash
./client  # Connects to 127.0.0.1:5555
```

**Example Session:**

```bash
$ ./client
Client: connected to server on address 127.0.0.1:5555
> whoami
user
> pwd
/home/user
> ls -la
total 8
drwxr-xr-x  2 user user 4096 Jan 15 10:30 .
drwxr-xr-x  3 user user 4096 Jan 15 10:30 ..
> exit
```

## Command Examples

### Safe Testing Commands

Use only benign commands for testing:

```bash
# System information
whoami
pwd
id
uname -a

# File system exploration
ls -la
ls -la /tmp
cat /etc/os-release

# Network information (local only)
hostname
ip addr show
netstat -tuln
```

### Command Execution Flow

1. **Client sends command** to implant via TCP socket
2. **Implant executes command** using `execProcess()`
3. **Output returned** to client via socket
4. **Client displays** output to user

## Advanced Usage

### Custom Port Configuration

```bash
# Start implant on custom port
./implant 8080

# Connect client to custom port
./client  # Still connects to 127.0.0.1:5555 by default
```

!!! note "Port Configuration"
    The client currently hardcodes port 5555. For custom ports, you would need to modify the source code.

### Background Execution

```bash
# Start implant in background
./implant &
IMPLANT_PID=$!

# Run client
./client

# Clean up when done
kill $IMPLANT_PID
```

### Multiple Sessions

The implant can handle multiple client connections sequentially:

```bash
# Terminal 1: Start implant
./implant

# Terminal 2: First client
./client
# Run some commands, then exit

# Terminal 3: Second client
./client
# Run more commands
```

## Safety Guidelines

### Testing Environment

!!! warning "Safety Requirements"
    - **Always use loopback (127.0.0.1)** for testing
    - **Run as non-privileged user** for safety
    - **Use isolated environments** only
    - **Test with benign commands** only

### Recommended Test Commands

```bash
# System information (safe)
whoami
pwd
id
uname -a
hostname

# File operations (safe)
ls -la
cat /etc/os-release
head -5 /etc/passwd

# Network (local only)
ip addr show
netstat -tuln
```

### Commands to Avoid

```bash
# Dangerous commands (DO NOT USE)
rm -rf /
dd if=/dev/zero of=/dev/sda
mkfs.ext4 /dev/sda
shutdown -h now
```

## Troubleshooting

### Connection Issues

**"Connection refused":**

```bash
# Check if implant is running
ps aux | grep implant
# Restart implant if needed
./implant
```

**"Address already in use":**

```bash
# Find process using port 5555
lsof -i :5555
# Kill the process or use different port
./implant 8080
```

### Performance Issues

**Slow command execution:**

- Check system resources
- Use simpler commands for testing
- Ensure proper network connectivity (loopback)

**Memory usage:**

- Restart implant periodically
- Monitor system resources
- Use `htop` or `top` to check usage

## Best Practices

### Development Workflow

1. **Start implant** in background
2. **Connect client** for testing
3. **Use benign commands** for verification
4. **Clean up** processes when done
5. **Monitor logs** for any issues

### Security Considerations

- **No external network access** during testing
- **Use non-privileged accounts** for all operations
- **Clean up test artifacts** after testing
- **Monitor system resources** during testing

## Example Workflow

```bash
# 1. Start implant
./implant &
IMPLANT_PID=$!

# 2. Test connection
./client
> whoami
user
> pwd
/home/user/ghost_monkey
> exit

# 3. Clean up
kill $IMPLANT_PID
echo "Testing complete"
```

This workflow ensures safe, controlled testing of the Ghost_Monkey backdoor in an educational environment.
