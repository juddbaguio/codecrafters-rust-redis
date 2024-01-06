use std::sync::Arc;

// Uncomment this block to pass the first stage
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use anyhow::Result;

use crate::storage::KVStore;

mod command_parser;
mod storage;

async fn handle_connection(stream: &mut TcpStream, kv_store: Arc<KVStore>) -> Result<()> {
    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer).await {
            Ok(_size) => {
                let stream_payload = String::from_utf8(buffer.to_vec())?;
                let payload = command_parser::Command::parse(stream_payload)?;
                println!("{:#?}", payload.payload);
                let resp = kv_store.build_response(payload)?;

                stream.write_all(resp.as_bytes()).await?;
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
    let kv_store = Arc::new(KVStore::new());

    loop {
        let (mut socket, _) = listener.accept().await?;
        let cloned_store = Arc::clone(&kv_store);
        tokio::spawn(async move { handle_connection(&mut socket, cloned_store).await });
    }
}
