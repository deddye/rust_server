mod cmd_parser;
use std::{net::{TcpListener, TcpStream, Shutdown, SocketAddr}, io::{BufReader, BufRead, Write}};


fn main() -> Result<(), std::io::Error> {

    let arguments = cmd_parser::parse_commands();

    match arguments.port_flag.as_str() {
        "-p" => {
            eprintln!("Using port {}", arguments.port);

            server_loop(arguments.port);
        },
        _ => println!("NOT VALID PORT FLAG!"),
    }

   Ok(())        

}

/**
 * function to handle a client
 */
fn handle_client(stream: TcpStream, client_port: SocketAddr) {

    
    let reader = BufReader::new(&stream); // 2

    for (index, line) in reader.lines().enumerate() { //Loop through all the lines in the file
        eprintln!("{} - CURRENT LINE: {:?}", index, line);
    }

    repond_client(client_port);

    stream.shutdown(Shutdown::Both).expect("shutdown call failed");

}

fn repond_client(client_addr: SocketAddr) {
    let stream = TcpStream::connect(client_addr);


    let _ = stream.unwrap().write(b"we good");
}

/**
 * function start the server and begin listening
 */
fn server_loop(port_string: i32) {

    /*
    starting off single threaded for now
    that means synchronously in rust right??
     */

    let listener = TcpListener::bind(format!("127.0.0.1:{port_string}")).unwrap();

    let mut incoming = listener.incoming();
    
    while let Some(stream) = incoming.next() {
        let s = stream.unwrap();
        let client_addr = s.peer_addr().unwrap();
        eprintln!("Accepting from: {:?}", s.peer_addr());

        let _ = handle_client(s, client_addr);

    }

}

