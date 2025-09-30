/// Transport Layer Module
///
/// This module provides transport layer abstraction using the network-protocol crate.
/// It handles TCP connections, packet serialization, and basic network operations.
///
/// The transport layer is responsible for:
/// - Managing network connections (TCP)
/// - Packet encoding/decoding using network-protocol's PacketCodec
/// - Connection establishment and teardown
/// - Basic error handling and timeouts
pub mod config;
pub mod manager;

pub use config::TransportConfig;
pub use manager::TransportManager;

/// Re-export commonly used types from network-protocol crate
/// TODO: Uncomment these imports once network-protocol dependency is properly configured
// pub use network_protocol::{Packet, PacketCodec, Transport};
/// Transport layer result type
pub type TransportResult<T> = Result<T, TransportError>;

/// Transport layer errors
#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("Network protocol error: {0}")]
    NetworkProtocol(String), // TODO: Use network_protocol::Error once dependency is added

    #[error("Connection timeout")]
    Timeout,

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Connection closed")]
    ConnectionClosed,
}
