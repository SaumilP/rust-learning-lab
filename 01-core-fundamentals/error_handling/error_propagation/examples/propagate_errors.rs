use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct MissingHost;

impl fmt::Display for MissingHost {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "host is required")
    }
}

impl Error for MissingHost {}

fn parse_port(text: &str) -> Result<u16, ParseIntError> {
    text.trim().parse()
}

fn build_address(host: Option<&str>, port: &str) -> Result<String, Box<dyn Error>> {
    // ok_or converts an absent value into Result so `?` can propagate it.
    let host = host
        .filter(|value| !value.trim().is_empty())
        .ok_or(MissingHost)?;

    // ParseIntError is automatically converted into Box<dyn Error> by `?`.
    let port = parse_port(port)?;
    Ok(format!("{}:{port}", host.trim()))
}

fn describe_address(host: Option<&str>, port: &str) -> String {
    build_address(host, port)
        .map(|address| format!("Ready to connect to {address}"))
        .unwrap_or_else(|error| format!("Configuration error: {error}"))
}

fn main() {
    println!("{}", describe_address(Some("localhost"), "8080"));
    println!("{}", describe_address(None, "8080"));
    println!("{}", describe_address(Some("localhost"), "eight"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propagates_missing_host() {
        let error = build_address(None, "8080").unwrap_err();
        assert_eq!(error.to_string(), "host is required");
    }

    #[test]
    fn propagates_invalid_port() {
        let error = build_address(Some("localhost"), "invalid").unwrap_err();
        assert!(error.downcast_ref::<ParseIntError>().is_some());
    }
}
