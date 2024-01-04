// Uncomment this block to pass the first stage
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_connection(stream: &mut TcpStream) {
    let _ = stream.write_all(b"+PONG\r\n");
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                handle_connection(&mut stream)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
