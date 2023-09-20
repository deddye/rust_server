
use std::{net::{ TcpStream, Shutdown, SocketAddr}, io::{BufReader, BufRead, Write}};


/**
 * function to handle a client
 */
pub fn handle_client(stream: TcpStream, client_port: SocketAddr) {

    
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