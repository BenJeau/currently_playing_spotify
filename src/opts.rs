use clap::Parser;

/// Simple Rust websocket proxy server using Actix to know what track the specified user is currently listening
#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "Benoît J. <benoit@jeaurond.dev>")]
pub struct Opts {
    /// Maximum interval in seconds which the Spotify API will be called
    #[clap(short, long, env = "INTERVAL_QUERY_SECS", default_value = "2")]
    pub interval: u64,

    /// Authentication code from the Spotify user taken from the Authentication authentication flow
    #[clap(long, env = "SPOTIFY_AUTH_CODE")]
    pub auth_code: String,

    /// Spotify application client id
    #[clap(long, env = "SPOTIFY_CLIENT_ID")]
    pub client_id: String,

    /// Spotify application client secret
    #[clap(long, env = "SPOTIFY_CLIENT_SECRET")]
    pub client_secret: String,

    /// Websocket server port
    #[clap(short, long, env = "WEBSOCKET_PORT", default_value = "8080")]
    pub port: String,

    /// Websocket server address
    #[clap(short, long, env = "WEBSOCKET_ADDRESS", default_value = "0.0.0.0")]
    pub address: String,
}
