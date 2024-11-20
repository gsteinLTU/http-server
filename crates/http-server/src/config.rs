use std::net::IpAddr;

pub struct Config {
    /// The IP address to bind to.
    pub host: IpAddr,
    /// The port to bind to.
    pub port: u16,
    /// Enable CORS with a permissive policy.
    pub cors: bool,
}