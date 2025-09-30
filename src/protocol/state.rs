/// Protocol State Management
///
/// Defines connection states and error types for the protocol layer.
/// Connection state machine for protocol operations
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    /// Initial state - no connection established
    Disconnected,

    /// TCP connection established but not authenticated
    Connected,

    /// ECDH handshake in progress
    Handshaking,

    /// Handshake complete, ready for secure communication
    Authenticated,

    /// Connection error occurred
    Error(String),
}

impl ConnectionState {
    /// Check if the connection is in a state that allows secure communication
    pub fn can_send_secure(&self) -> bool {
        matches!(self, ConnectionState::Authenticated)
    }

    /// Check if the connection allows handshake operations
    pub fn can_handshake(&self) -> bool {
        matches!(
            self,
            ConnectionState::Connected | ConnectionState::Handshaking
        )
    }

    /// Check if the connection is in an error state
    pub fn is_error(&self) -> bool {
        matches!(self, ConnectionState::Error(_))
    }
}

impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionState::Disconnected => write!(f, "Disconnected"),
            ConnectionState::Connected => write!(f, "Connected"),
            ConnectionState::Handshaking => write!(f, "Handshaking"),
            ConnectionState::Authenticated => write!(f, "Authenticated"),
            ConnectionState::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

/// Comprehensive error handling for protocol operations
#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    /// Network transport errors
    #[error("Transport error: {0}")]
    Transport(#[from] crate::transport::TransportError),

    /// Security/handshake operation errors
    #[error("Security error: {0}")]
    Security(String),

    /// Message serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Command execution errors
    #[error("Command execution error: {0}")]
    CommandExecution(String),

    /// Protocol violation errors
    #[error("Protocol violation: {0}")]
    ProtocolViolation(String),

    /// Timeout errors
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Authentication/handshake errors
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Invalid message format or content
    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    /// Connection state errors
    #[error("Invalid connection state: {0}")]
    InvalidState(String),
}
