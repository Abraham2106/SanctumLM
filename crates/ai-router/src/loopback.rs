use std::net::Ipv4Addr;

use core_domain::AdapterError;
use url::{Host, Url};

/// Solo `http://127.0.0.1:<puerto>` (ADR-0008; rechaza `localhost`, LAN y esquemas no HTTP).
pub fn parse_loopback_http_base(base: &str) -> Result<Url, AdapterError> {
    let url = Url::parse(base.trim()).map_err(|_| AdapterError::InvalidResponse)?;
    if url.scheme() != "http" {
        return Err(AdapterError::PolicyDenied);
    }
    let host = url.host().ok_or(AdapterError::PolicyDenied)?;
    match host {
        Host::Ipv4(ip) if ip == Ipv4Addr::LOCALHOST => Ok(url),
        _ => Err(AdapterError::PolicyDenied),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_ipv4_loopback() {
        assert!(parse_loopback_http_base("http://127.0.0.1:11434").is_ok());
    }

    #[test]
    fn rejects_localhost_hostname() {
        assert_eq!(
            parse_loopback_http_base("http://localhost:11434").unwrap_err(),
            AdapterError::PolicyDenied
        );
    }

    #[test]
    fn rejects_https() {
        assert_eq!(
            parse_loopback_http_base("https://127.0.0.1:11434").unwrap_err(),
            AdapterError::PolicyDenied
        );
    }
}
