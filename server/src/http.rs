use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

pub fn parse_request(stream: &mut TcpStream) -> Vec<u8> {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut name = String::new();
    loop {
        let r = reader.read_line(&mut name).unwrap();
        if r < 3 {
            //detect empty line
            break;
        }
    }

    println!("THIS IS THE HEADER: {}", name);

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

    return buffer;
}

// FUNCTION TO PARSE HEADERS

// GET THE END POINT FROM HEADER AND RESPOND DEPENDING ON
// HTTP REQUEST TYPE AND SPECIFIED ENDPOINT

// FUNCTION TO PARSE BODY
