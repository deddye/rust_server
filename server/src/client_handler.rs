use std::{io::Write, net::TcpStream};

use crate::http;

/**
 * function to handle a client
 */
pub fn handle_client(mut stream: TcpStream) {
    let buffer = http::parse_request(&mut stream);
    let _ = stream.write(b"HTTP/1.1 200 OK\r\n\r\nRequest Body: ");
    let _ = stream.write(&buffer);

    stream.flush().unwrap();
}
