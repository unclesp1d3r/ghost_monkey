# Network-Protocol Crate

Ghost Monkey is designed to leverage the `network-protocol` crate for its networking foundation. This chapter covers the planned integration and usage of this crate.

**Current Status**: The `network-protocol` dependency is configured in `Cargo.toml` but not yet implemented in the codebase.

## Overview

The `network-protocol` crate provides:

- **Transport Abstraction**: Cross-platform transport layer
- **Packet Codec**: Message serialization and deserialization
- **ECDH Handshake**: Secure key exchange protocol
- **Heartbeat Mechanism**: Connection health monitoring
- **Error Handling**: Comprehensive error types and handling

## Planned Integration

### Transport Layer

```rust
// Planned transport manager implementation
use network_protocol::{PacketCodec, Transport};

pub struct TransportManager {
    transport: Transport,
    codec: PacketCodec,
}

impl TransportManager {
    pub fn new_client(addr: &str) -> Result<Self, TransportError> {
        // Implementation planned
        todo!()
    }

    pub fn new_server(bind_addr: &str) -> Result<Self, TransportError> {
        // Implementation planned
        todo!()
    }
}
```

### Security Integration

```rust
// Planned security manager using network-protocol's ECDH
use network_protocol::{HandshakeState, SecureHandshake};

pub struct SecurityManager {
    handshake_state: HandshakeState,
    session_key: Option<[u8; 32]>,
}

impl SecurityManager {
    pub fn initiate_handshake(&mut self) -> Result<(), SecurityError> {
        // Implementation planned
        todo!()
    }
}
```

## Current Implementation

The project currently includes the `network-protocol` dependency in `Cargo.toml`:

```toml
[dependencies]
network-protocol = "1.0"
```

However, the actual implementation is still in the planning phase. The binary targets currently contain only placeholder code.

## Next Steps

1. Implement basic transport layer using `network-protocol`
2. Add secure handshake functionality
3. Create message types and serialization
4. Implement connection management
5. Add comprehensive error handling

## Development Progress

Track the implementation progress in the [tasks specification](.kiro/specs/core-networking-protocol/tasks.md).
