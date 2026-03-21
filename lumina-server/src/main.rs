use std::net::{TcpListener};
use std::io::{Write, BufReader, BufRead};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000")
        .expect("Failed to bind address and port!");

    println!("Listening for connections on localhost 3000!");

    for request in listener.incoming() {
        let mut stream = request.expect("Request could not be unwrapped.");

        BufReader::new(&stream)
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .for_each(|unwraped_line| println!("{}", unwraped_line));

        stream
            .write(b"HTTP/1.1 200 OK\nContent-Type: application/json\r\n\r\n{ \"message\": \"Hello World\"}")
            .expect("Failed to send response!");
    }
}
