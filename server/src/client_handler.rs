use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{Shutdown, SocketAddr, TcpStream},
    str,
};

/**
 * function to handle a client
 */
pub fn handle_client(stream: TcpStream, client_port: SocketAddr) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut name = String::new();
    loop {
        let r = reader.read_line(&mut name).unwrap();
        if r < 3 {
            //detect empty line
            break;
        }
    }
    let mut size = 0;
    let linesplit = name.split("\n");
    for l in linesplit {
        if l.starts_with("Content-Length") {
            let sizeplit = l.split(":");
            for s in sizeplit {
                if !(s.starts_with("Content-Length")) {
                    size = s.trim().parse::<usize>().unwrap(); //Get Content-Length
                }
            }
        }
    }
    let mut buffer = vec![0; size]; //New Vector with size of Content
    reader.read_exact(&mut buffer).unwrap(); //Get the Body Content.
    let body = str::from_utf8(&buffer).unwrap();
    println!("{body}");
}

fn repond_client(client_addr: SocketAddr) {
    let stream = TcpStream::connect(client_addr);

    let _ = stream.unwrap().write(b"we good");
}
