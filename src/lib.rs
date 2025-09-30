pub mod protocol;
/// Ghost Monkey - Educational Backdoor for Authorized Penetration Testing
///
/// This library provides the core functionality for Ghost Monkey, an educational
/// backdoor designed for authorized penetration testing and OSCP preparation.
///
/// # Safety and Legal Notice
///
/// This tool is designed for educational purposes and authorized penetration testing only.
/// Users must ensure they have explicit permission before using this tool on any system.
///
/// # Architecture
///
/// The library is organized into several layers:
/// - **Transport Layer**: Network communication using network-protocol crate
/// - **Protocol Layer**: Secure handshake, message handling, and state management
/// - **Application Layer**: Command execution and response handling (in binaries)
///
/// # Example Usage
///
/// ```rust,no_run
/// use ghost_monkey::transport::{TransportManager, TransportConfig};
/// use ghost_monkey::protocol::ProtocolHandler;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create transport manager for client mode
/// let mut transport = TransportManager::new_client("127.0.0.1:8080".parse()?).await?;
///
/// // Create protocol handler and perform handshake
/// let mut protocol = ProtocolHandler::new();
/// protocol.perform_handshake(&mut transport, true).await?;
///
/// // Send command and receive response
/// protocol.send_command(&mut transport, "ls").await?;
/// let response = protocol.recv_response(&mut transport).await?;
/// println!("Command output: {}", response);
/// # Ok(())
/// # }
/// ```
pub mod transport;

// Re-export commonly used types for convenience
pub use protocol::{AppMessage, CommandResponse, ConnectionState, ProtocolHandler};
pub use transport::{TransportConfig, TransportManager};
