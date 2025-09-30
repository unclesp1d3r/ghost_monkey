use crate::protocol::{
    AppMessage, CommandResponse, ConnectionState, ProtocolError, ProtocolResult, SecurityManager,
};
/// Protocol Handler
///
/// Coordinates protocol-level operations including handshake management,
/// message routing, and connection state management.
use crate::transport::TransportManager;

/// Handles protocol-level operations and state management
pub struct ProtocolHandler {
    /// Security manager for handshake and encryption
    security: SecurityManager,
    /// Current connection state
    state: ConnectionState,
}

impl ProtocolHandler {
    /// Create a new protocol handler
    pub fn new() -> Self {
        Self {
            security: SecurityManager::new(),
            state: ConnectionState::Disconnected,
        }
    }

    /// Perform ECDH handshake using network-protocol's handshake functions
    pub async fn perform_handshake(
        &mut self,
        transport: &mut TransportManager,
        is_initiator: bool,
    ) -> ProtocolResult<()> {
        if !self.state.can_handshake() {
            return Err(ProtocolError::InvalidState(format!(
                "Cannot perform handshake in state: {}",
                self.state
            )));
        }

        self.state = ConnectionState::Handshaking;

        let result = if is_initiator {
            self.security.initiate_handshake(transport).await
        } else {
            self.security.respond_handshake(transport).await
        };

        match result {
            Ok(()) => {
                self.state = ConnectionState::Authenticated;
                Ok(())
            }
            Err(e) => {
                self.state = ConnectionState::Error(format!("Handshake failed: {}", e));
                Err(e)
            }
        }
    }

    /// Send encrypted command message
    pub async fn send_command(
        &mut self,
        transport: &mut TransportManager,
        command: &str,
    ) -> ProtocolResult<()> {
        if !self.state.can_send_secure() {
            return Err(ProtocolError::InvalidState(format!(
                "Cannot send secure message in state: {}",
                self.state
            )));
        }

        let message = AppMessage::Command {
            command: command.to_string(),
        };

        // Validate message before sending
        message.validate().map_err(ProtocolError::InvalidMessage)?;

        self.security.send_secure_message(transport, &message).await
    }

    /// Receive and decrypt response message
    pub async fn recv_response(
        &mut self,
        transport: &mut TransportManager,
    ) -> ProtocolResult<CommandResponse> {
        if !self.state.can_send_secure() {
            return Err(ProtocolError::InvalidState(format!(
                "Cannot receive secure message in state: {}",
                self.state
            )));
        }

        let message = self.security.recv_secure_message(transport).await?;

        match message {
            AppMessage::Response {
                success,
                stdout,
                stderr,
                exit_code,
            } => Ok(CommandResponse {
                success,
                stdout,
                stderr,
                exit_code,
            }),
            AppMessage::Error { message } => Ok(CommandResponse::error(message)),
            _ => Err(ProtocolError::ProtocolViolation(
                "Expected response or error message".to_string(),
            )),
        }
    }

    /// Handle heartbeat messages using network-protocol's heartbeat mechanism
    pub async fn handle_heartbeat(
        &mut self,
        _transport: &mut TransportManager,
    ) -> ProtocolResult<()> {
        // TODO: Implement heartbeat handling using network-protocol's Ping/Pong messages
        // TODO: Send Ping messages at regular intervals
        // TODO: Handle Pong responses and connection validation
        // TODO: Detect heartbeat timeouts and update connection state

        todo!("Implement heartbeat mechanism")
    }

    /// Get current connection state
    pub fn state(&self) -> &ConnectionState {
        &self.state
    }

    /// Update connection state (for transport layer events)
    pub fn set_state(&mut self, state: ConnectionState) {
        self.state = state;
    }

    /// Check if the protocol handler is ready for secure communication
    pub fn is_authenticated(&self) -> bool {
        self.state.can_send_secure() && self.security.is_secure()
    }

    /// Reset protocol handler state (for reconnection)
    pub fn reset(&mut self) {
        self.security.reset();
        self.state = ConnectionState::Disconnected;
    }
}

impl Default for ProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}
