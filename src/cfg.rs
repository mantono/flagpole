use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[arg(short, long, env = "PORT", default_value = "3000")]
    port: u16,
    #[arg(short = 'H', long, env = "HOST", default_value = "0.0.0.0")]
    host: String,
    #[cfg(feature = "logging")]
    #[arg(short, long, env = "LOG_LEVEL", default_value = "Info")]
    log_level: log::Level,
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
