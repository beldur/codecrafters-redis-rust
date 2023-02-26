use anyhow::Result;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let incoming = listener.accept().await;

        match incoming {
            Ok((stream, _)) => {
                println!("Accepted new connection");

                tokio::spawn(async move {
                    handle_connection(stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = BytesMut::with_capacity(512);

    loop {
        let bytes_read = stream.read(&mut buffer).await?;

        if bytes_read == 0 {
            println!("Client closed the connection");
            break;
        }

        stream.write("+PONG\r\n".as_bytes()).await?;
    }
    Ok(())
}
