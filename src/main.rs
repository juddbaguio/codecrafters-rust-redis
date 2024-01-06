// Uncomment this block to pass the first stage
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use anyhow::Result;

mod command_parser;

async fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer).await {
            Ok(_size) => {
                let stream_payload = String::from_utf8(buffer.to_vec())?;
                let mut buf_res = command_parser::Command::parse(stream_payload)?;

                stream
                    .write_all(buf_res.build_response()?.as_bytes())
                    .await?;
                stream.flush().await?;
            }
            Err(err) => {
                println!("error: {}", err);
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move { handle_connection(&mut socket).await });
    }
}
