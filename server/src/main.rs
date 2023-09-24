pub mod utils;
use utils::cmd_parser::parse_commands;

mod client_handler;
mod http;
use std::net::TcpListener;

fn main() -> Result<(), std::io::Error> {
    let arguments = parse_commands();

    match arguments.port_flag.as_str() {
        "-p" => {
            eprintln!("Listening on port: {}", arguments.port);

            server_loop(arguments.port);
        }
        _ => println!("NOT VALID PORT FLAG!"),
    }

    Ok(())
}

/**
 * function start the server and begin listening
 */
fn server_loop(port_string: i32) {
    let listener = TcpListener::bind(format!("127.0.0.1:{port_string}")).unwrap();

    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next() {
        let s = stream.unwrap();
        eprintln!("Accepting from: {:?}", s.peer_addr());

        let _ = client_handler::handle_client(s);
    }
}
