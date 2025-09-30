# First Run

This chapter walks you through your first Ghost_Monkey session, demonstrating basic client-implant communication in a safe, controlled environment.

## Safety First

⚠️ **Important Safety Guidelines**:

- Always start with localhost (127.0.0.1) connections
- Use isolated virtual machines or containers when possible
- Run as a non-privileged user
- Only use in authorized environments

## Basic Setup

For your first run, we'll demonstrate the call-in mode where the client connects to a listening implant.

### Terminal Setup

You'll need two terminal windows:

- **Terminal 1**: For the implant (server)
- **Terminal 2**: For the client

### Step 1: Start the Implant

In Terminal 1, start the implant in listen mode:

```bash
# Navigate to your Ghost_Monkey directory
cd ghost_monkey

# Start the implant listening on localhost port 8080
./target/release/ghost-implant --listen 127.0.0.1:8080
```

You should see output similar to:

```
[INFO] Ghost_Monkey Implant v0.1.0
[WARN] Educational Use Only - Authorized Testing Only
[INFO] Listening on 127.0.0.1:8080
[INFO] Waiting for client connection...
```

### Step 2: Connect the Client

In Terminal 2, connect the client to the implant:

```bash
# Connect to the listening implant
./target/release/ghost-client 127.0.0.1:8080
```

You should see the client establish a connection:

```
[INFO] Ghost_Monkey Client v0.1.0
[INFO] Connecting to 127.0.0.1:8080...
[INFO] Performing secure handshake...
[INFO] Handshake complete - secure channel established
[INFO] Connection ready
ghost>
```

### Step 3: Execute Your First Command

At the `ghost>` prompt, try the basic command:

```bash
ghost> ls
```

You should see the directory listing from the implant's working directory:

```
[INFO] Executing command: ls
total 48
drwxr-xr-x  8 user user 4096 Jan 15 10:30 .
drwxr-xr-x  3 user user 4096 Jan 15 10:25 ..
-rw-r--r--  1 user user  1234 Jan 15 10:30 Cargo.toml
drwxr-xr-x  2 user user 4096 Jan 15 10:30 src
drwxr-xr-x  2 user user 4096 Jan 15 10:30 target
...
```

## Understanding the Output

### Implant Terminal

The implant terminal will show:

```
[INFO] Client connected from 127.0.0.1:54321
[INFO] Performing secure handshake...
[INFO] Handshake complete - client authenticated
[INFO] Received command: ls
[INFO] Command executed successfully
[INFO] Response sent (1024 bytes)
```

### Client Terminal

The client terminal displays:

- Command prompt (`ghost>`)
- Command execution status
- Command output (stdout/stderr)
- Connection status information

## Testing Different Scenarios

### Valid Commands

Try these safe commands:

```bash
ghost> ls
ghost> pwd
ghost> whoami
```

### Invalid Commands

Try a command that's not allowed:

```bash
ghost> cat /etc/passwd
```

You should see an error:

```
[ERROR] Command not allowed: cat
[INFO] Only 'ls' command is permitted in this educational version
```

## Connection Modes

### Call-in Mode (Default)

What we just demonstrated:

1. Implant listens on a port
2. Client connects to the implant

### Callback Mode

For firewall evasion scenarios:

**Terminal 1 (Client listening):**

```bash
./target/release/ghost-client --listen 127.0.0.1:8080
```

**Terminal 2 (Implant connecting back):**

```bash
./target/release/ghost-implant --callback 127.0.0.1:8080
```

## Troubleshooting First Run

### Connection Refused

If you see "Connection refused":

- Ensure the implant is running and listening
- Check that you're using the correct IP and port
- Verify no firewall is blocking the connection

### Permission Denied

If you see permission errors:

- Ensure you're running as the correct user
- Check file permissions on the binaries
- Try using a different port (above 1024)

### Handshake Failures

If the secure handshake fails:

- Check that both binaries are the same version
- Ensure no network interference
- Try restarting both client and implant

## Clean Shutdown

To properly close the connection:

1. In the client terminal, type `exit` or press `Ctrl+C`
2. The implant will detect the disconnection and can accept new connections

## Next Steps

Now that you've successfully run Ghost_Monkey:

- Read the [Quick Start Guide](quick-start.md) for more usage examples
- Review [Safety Guidelines](safety-guidelines.md) for important security considerations
- Explore the [Architecture Overview](../ch02-architecture/overview.md) to understand how it works

## Educational Notes

This first run demonstrates several important concepts:

- **Secure Communication**: All traffic is encrypted using ChaCha20-Poly1305
- **Key Exchange**: ECDH key exchange establishes the secure channel
- **Command Validation**: Only allowed commands can be executed
- **Bidirectional Communication**: Both call-in and callback modes work identically

Understanding these fundamentals prepares you for more advanced topics covered in later chapters.
