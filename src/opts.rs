use clap::Parser;

/// Simple Rust websocket proxy server using Actix to know what track the specified user is currently listening
#[derive(Parser, Debug)]
#[command(version, author = "Benoît J. <benoit@jeaurond.dev>")]
pub struct Opts {
    /// Maximum interval in seconds which the Spotify API will be called
    #[arg(short, long, env = "INTERVAL_QUERY_SECS", default_value = "2")]
    pub interval: u64,

    /// Authentication code from the Spotify user taken from the Authentication authentication flow
    #[arg(long, env = "SPOTIFY_AUTH_CODE")]
    pub auth_code: String,

    /// Spotify application client id
    #[arg(long, env = "SPOTIFY_CLIENT_ID")]
    pub client_id: String,

    /// Spotify application client secret
    #[arg(long, env = "SPOTIFY_CLIENT_SECRET")]
    pub client_secret: String,

    /// Websocket server port
    #[arg(short, long, env = "WEBSOCKET_PORT", default_value = "8080")]
    pub port: String,

    /// Websocket server address
    #[arg(short, long, env = "WEBSOCKET_ADDRESS", default_value = "0.0.0.0")]
    pub address: String,

    /// Set a single allow origin target, permissive if nothing is passed
    #[arg(long, env = "CORS_ORIGIN")]
    pub cors_origin: Option<String>,

    /// Compacts the JSON response (removes many fields from the Spotify response)
    #[arg(short, long, env = "COMPACT", default_value = "false")]
    pub compact: bool,
}
