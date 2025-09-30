/// Transport Configuration
///
/// Configuration struct for transport layer operations including
/// bind addresses, timeouts, and connection parameters.
use std::net::SocketAddr;
use std::time::Duration;

/// Configuration for network transport operations
#[derive(Debug, Clone)]
pub struct TransportConfig {
    /// Bind address for server mode
    pub bind_addr: SocketAddr,

    /// Connection timeout duration
    pub connect_timeout: Duration,

    /// Read/write operation timeout
    pub io_timeout: Duration,

    /// Maximum packet size in bytes
    pub max_packet_size: usize,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:8080".parse().unwrap(),
            connect_timeout: Duration::from_secs(30),
            io_timeout: Duration::from_secs(60),
            max_packet_size: 1024 * 1024, // 1MB
        }
    }
}

impl TransportConfig {
    /// Create a new transport configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set bind address
    pub fn with_bind_addr(mut self, addr: SocketAddr) -> Self {
        self.bind_addr = addr;
        self
    }

    /// Set connection timeout
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set I/O timeout
    pub fn with_io_timeout(mut self, timeout: Duration) -> Self {
        self.io_timeout = timeout;
        self
    }

    /// Set maximum packet size
    pub fn with_max_packet_size(mut self, size: usize) -> Self {
        self.max_packet_size = size;
        self
    }

    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.connect_timeout.is_zero() {
            return Err("Connect timeout cannot be zero".to_string());
        }

        if self.io_timeout.is_zero() {
            return Err("I/O timeout cannot be zero".to_string());
        }

        if self.max_packet_size == 0 {
            return Err("Maximum packet size cannot be zero".to_string());
        }

        Ok(())
    }
}
