// Uncomment this block to pass the first stage
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use anyhow::Result;

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    stream.write_all(b"+PONG\r\n")?;

    Ok(())
}

fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    loop {
        match listener.accept() {
            Ok((mut socket, _)) => {
                println!("accepted new connection");

                handle_connection(&mut socket)?
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
