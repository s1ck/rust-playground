use std::{
    error,
    fmt::Display,
    fs::File,
    net::{AddrParseError, Ipv6Addr},
};

#[derive(Debug)]
enum UpstreamError {
    IO(std::io::Error),
    Parsing(AddrParseError),
}

impl Display for UpstreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for UpstreamError {
    fn from(io_error: std::io::Error) -> Self {
        UpstreamError::IO(io_error)
    }
}

impl From<AddrParseError> for UpstreamError {
    fn from(addr_error: AddrParseError) -> Self {
        UpstreamError::Parsing(addr_error)
    }
}

impl error::Error for UpstreamError {}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt")?;
    let _l = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
