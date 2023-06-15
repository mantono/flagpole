use clap::Parser;

#[derive(Parser)]
#[command(name = "flagpole")]
pub struct Config {
    /// Address to bind to
    #[arg(short = 'H', long, env = "HOST", default_value = "0.0.0.0")]
    host: String,

    /// Port number
    ///
    /// Port number to listen on for incoming requests
    #[arg(short, long, env = "PORT", default_value = "3000")]
    port: u16,
    /// Log level
    ///
    /// Set the minimum log level
    #[cfg(feature = "logging")]
    #[arg(short, long, env = "LOG_LEVEL", default_value = "Info")]
    log_level: log::Level,

    /// API key
    ///
    /// Set an optional API key. If set, it must be present in any requests that alters state.
    /// If no API key is set, all requests can be done without any form of authentication.
    #[arg(short = 'K', long, env = "API_KEY")]
    api_key: Option<String>,
}

impl Config {
    pub fn address(&self) -> Result<std::net::SocketAddr, std::net::AddrParseError> {
        format!("{}:{}", self.host, self.port).parse()
    }

    #[cfg(feature = "logging")]
    pub fn log_level(&self) -> log::Level {
        self.log_level
    }

    pub fn api_key(&self) -> Option<String> {
        self.api_key.clone()
    }
}
