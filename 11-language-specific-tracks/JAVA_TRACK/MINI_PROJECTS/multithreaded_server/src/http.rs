use std::collections::HashMap;
use std::fmt;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

/// HTTP request method
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
}

impl Method {
    fn from_str(s: &str) -> Option<Method> {
        match s {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "DELETE" => Some(Method::DELETE),
            "HEAD" => Some(Method::HEAD),
            "OPTIONS" => Some(Method::OPTIONS),
            "PATCH" => Some(Method::PATCH),
            _ => None,
        }
    }
}

/// HTTP request
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    /// Parse an HTTP request from a TCP stream
    pub fn parse(stream: &mut TcpStream) -> Result<Request, String> {
        let buf_reader = BufReader::new(stream);
        let mut lines = buf_reader.lines();

        // Parse request line: GET /path HTTP/1.1
        let request_line = lines
            .next()
            .ok_or("No request line")?
            .map_err(|e| format!("Failed to read request line: {}", e))?;

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(format!("Invalid request line: {}", request_line));
        }

        let method = Method::from_str(parts[0])
            .ok_or_else(|| format!("Invalid method: {}", parts[0]))?;
        let path = parts[1].to_string();
        let version = parts[2].to_string();

        // Parse headers
        let mut headers = HashMap::new();
        for line in lines.by_ref() {
            let line = line.map_err(|e| format!("Failed to read header: {}", e))?;

            // Empty line signals end of headers
            if line.is_empty() {
                break;
            }

            if let Some((key, value)) = line.split_once(':') {
                headers.insert(
                    key.trim().to_lowercase(),
                    value.trim().to_string(),
                );
            }
        }

        // Parse body (simplified - doesn't handle chunked encoding)
        let mut body = String::new();
        if let Some(content_length) = headers.get("content-length") {
            if let Ok(length) = content_length.parse::<usize>() {
                let buffer = vec![0; length];
                // Note: In production, you'd read from the stream properly
                body = String::from_utf8_lossy(&buffer).to_string();
            }
        }

        Ok(Request {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}

/// HTTP response status code
#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    OK,
    NotFound,
    InternalServerError,
    BadRequest,
}

impl StatusCode {
    fn code(&self) -> u16 {
        match self {
            StatusCode::OK => 200,
            StatusCode::NotFound => 404,
            StatusCode::InternalServerError => 500,
            StatusCode::BadRequest => 400,
        }
    }

    fn reason(&self) -> &str {
        match self {
            StatusCode::OK => "OK",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::BadRequest => "Bad Request",
        }
    }
}

/// HTTP response
pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    /// Create a new response with the given status code
    pub fn new(status: StatusCode) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Server".to_string(), "Rust-HTTP/0.1".to_string());
        headers.insert("Connection".to_string(), "close".to_string());

        Response {
            status,
            headers,
            body: String::new(),
        }
    }

    /// Get the response body
    pub fn get_body(&self) -> &str {
        &self.body
    }

    /// Set the response body
    pub fn body(mut self, body: String) -> Self {
        self.headers.insert(
            "Content-Length".to_string(),
            body.len().to_string(),
        );
        self.body = body;
        self
    }

    /// Set a header
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Set Content-Type header
    pub fn content_type(self, content_type: &str) -> Self {
        self.header("Content-Type".to_string(), content_type.to_string())
    }

    /// Send the response to the client
    pub fn send(self, stream: &mut TcpStream) -> std::io::Result<()> {
        let response = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status.code(),
            self.status.reason(),
            self.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\r\n"),
            self.body
        );

        stream.write_all(response.as_bytes())?;
        stream.flush()
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}",
            self.status.code(),
            self.status.reason()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_parsing() {
        assert_eq!(Method::from_str("GET"), Some(Method::GET));
        assert_eq!(Method::from_str("POST"), Some(Method::POST));
        assert_eq!(Method::from_str("INVALID"), None);
    }

    #[test]
    fn test_status_codes() {
        assert_eq!(StatusCode::OK.code(), 200);
        assert_eq!(StatusCode::NotFound.code(), 404);
        assert_eq!(StatusCode::OK.reason(), "OK");
    }

    #[test]
    fn test_response_builder() {
        let response = Response::new(StatusCode::OK)
            .body("Hello, World!".to_string())
            .content_type("text/plain");

        assert_eq!(response.body, "Hello, World!");
        assert_eq!(
            response.headers.get("Content-Type"),
            Some(&"text/plain".to_string())
        );
    }
}
