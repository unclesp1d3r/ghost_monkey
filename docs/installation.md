# Installation

This guide will help you install Ghost_Monkey and its dependencies.

## Prerequisites

- **Nim ≥ 2.0**: Programming language runtime
- **Unix-like OS**: Linux, macOS, or WSL
- **Git**: For cloning the repository

## Installation Steps

### 1. Install Nim

**Cross-Platform (Recommended):**

```bash
curl https://nim-lang.org/choosenim/init.sh -sSf | sh
# Restart shell, then verify:
nim --version
nimble --version
```

**Windows (PowerShell):**

```powershell
# Option 1: Check winget first (preferred)
winget search nim
# If official Nim package available: winget install <PackageId>

# Option 2: Fallback to choosenim
iwr https://nim-lang.org/choosenim/init.ps1 -UseBasicParsing | iex
# Restart terminal, then verify:
nim --version
nimble --version
```

### 2. Clone the Repository

```bash
git clone https://github.com/unclesp1d3r/ghost_monkey.git
cd ghost_monkey
```

### 3. Install Dependencies

```bash
nimble refresh
nimble install -y strenc
```

### 4. Build the Project

```bash
nimble build
```

### 5. Verify Installation

```bash
# Start implant in background
./implant &
IMPLANT_PID=$!

# Connect with client
./client
# Test with benign commands like 'whoami', 'pwd'

# Clean up
kill $IMPLANT_PID
```

## Build Options

### Debug Build

```bash
nimble build
```

### Release Build

```bash
nimble build -d:release
```

### Static Analysis

```bash
nim check src/client.nim
nim check src/implant.nim
```

## Troubleshooting

### Common Issues

**Nim not found:**

```bash
# Add Nim to PATH
export PATH=$HOME/.nimble/bin:$PATH
# Or restart your shell after installing choosenim
```

**strenc package not found:**

```bash
nimble refresh
nimble install strenc
```

**Build failures:**

```bash
# Clean build artifacts
rm -rf nimcache/
nimble build
```

### Verification

Run the test suite to verify everything is working:

```bash
nimble test
```

Expected output:

```text
[Suite] Client Tests
  [OK] client binary exists
  [OK] client compiles
[Suite] Implant Tests
  [OK] implant binary exists
  [OK] implant compiles
[Suite] Integration Tests
  [OK] client and implant compile
  [OK] test environment setup
Success: All tests passed
```

## Security Notes

!!! warning "Security Reminders"
    - Always rebuild from source on trusted machines
    - Never use precompiled binaries without verification
    - Test only in isolated, loopback environments
    - Run as non-privileged user
