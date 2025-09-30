/// Protocol Layer Module
///
/// This module handles the application protocol layer built on top of the transport layer.
/// It manages secure handshakes, message serialization, and protocol state management.
///
/// The protocol layer is responsible for:
/// - ECDH handshake using network-protocol's secure handshake functions
/// - Message encryption and decryption (ChaCha20-Poly1305)
/// - Application message types and serialization
/// - Heartbeat and keepalive mechanisms
/// - Protocol state management and error handling
pub mod handler;
pub mod message;
pub mod security;
pub mod state;

pub use handler::ProtocolHandler;
pub use message::{AppMessage, CommandResponse};
pub use security::SecurityManager;
pub use state::{ConnectionState, ProtocolError};

/// Protocol layer result type
pub type ProtocolResult<T> = Result<T, ProtocolError>;

// Re-export network-protocol message types for handshake operations
// TODO: Uncomment these imports once network-protocol dependency is properly configured
// pub use network_protocol::protocol::message::Message;
// pub use network_protocol::protocol::message::Message::{
//     SecureHandshakeInit,
//     SecureHandshakeResponse,
//     SecureHandshakeConfirm,
//     Custom,
//     Ping,
//     Pong,
// };
