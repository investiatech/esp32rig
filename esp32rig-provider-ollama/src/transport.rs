use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    ConnectionFailed,
    Timeout,
    BufferTooSmall,
    InvalidResponse,
    UnsupportedUrl,
}

#[derive(Debug, Clone, Copy)]
pub struct TransportResponse {
    pub status: u16,
    pub body_len: usize,
}

pub trait HttpTransport {
    fn post(
        &mut self,
        url: &str,
        body: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<TransportResponse, TransportError>;
}

pub struct LinuxHttpTransport;

impl LinuxHttpTransport {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for LinuxHttpTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpTransport for LinuxHttpTransport {
    fn post(
        &mut self,
        url: &str,
        body: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<TransportResponse, TransportError> {
        let (host, port, path) = parse_url(url)?;

        let mut stream = TcpStream::connect((host, port))
            .map_err(|_| TransportError::ConnectionFailed)?;

        stream
            .set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|_| TransportError::ConnectionFailed)?;

        stream
            .set_write_timeout(Some(Duration::from_secs(30)))
            .map_err(|_| TransportError::ConnectionFailed)?;

        write!(
            stream,
            "POST {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n",
            path,
            host,
            body.len()
        )
            .map_err(|_| TransportError::ConnectionFailed)?;

        stream
            .write_all(body)
            .map_err(|_| TransportError::ConnectionFailed)?;

        let mut received = 0;

        loop {
            if received == response_buffer.len() {
                return Err(TransportError::BufferTooSmall);
            }

            match stream.read(&mut response_buffer[received..]) {
                Ok(0) => break,
                Ok(count) => received += count,
                Err(error)
                if error.kind() == std::io::ErrorKind::TimedOut
                    || error.kind() == std::io::ErrorKind::WouldBlock =>
                    {
                        return Err(TransportError::Timeout);
                    }
                Err(_) => return Err(TransportError::ConnectionFailed),
            }
        }

        let header_end = response_buffer[..received]
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .ok_or(TransportError::InvalidResponse)?;

        let status_line_end = response_buffer[..header_end]
            .windows(2)
            .position(|window| window == b"\r\n")
            .unwrap_or(header_end);

        let status_line = core::str::from_utf8(
            &response_buffer[..status_line_end],
        )
            .map_err(|_| TransportError::InvalidResponse)?;

        let status = status_line
            .split_whitespace()
            .nth(1)
            .and_then(|value| value.parse::<u16>().ok())
            .ok_or(TransportError::InvalidResponse)?;

        let body_start = header_end + 4;
        let body_len = received - body_start;

        response_buffer.copy_within(body_start..received, 0);

        Ok(TransportResponse {
            status,
            body_len,
        })
    }
}

fn parse_url(url: &str) -> Result<(&str, u16, &str), TransportError> {
    let url = url
        .strip_prefix("http://")
        .ok_or(TransportError::UnsupportedUrl)?;

    let path_start = url.find('/').unwrap_or(url.len());

    let authority = &url[..path_start];
    let path = if path_start == url.len() {
        "/"
    } else {
        &url[path_start..]
    };

    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => {
            let port = port
                .parse::<u16>()
                .map_err(|_| TransportError::UnsupportedUrl)?;

            (host, port)
        }
        None => (authority, 80),
    };

    if host.is_empty() {
        return Err(TransportError::UnsupportedUrl);
    }

    Ok((host, port, path))
}
