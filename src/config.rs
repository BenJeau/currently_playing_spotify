use std::time::Duration;

/// How often heartbeat pings are sent
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
/// Websocket address and port
pub const WEBSOCKET_ADDR: &str = "0.0.0.0:8080";
