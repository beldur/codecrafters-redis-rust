use anyhow::Result;
use resp::Value::{Error, SimpleString};
use tokio::net::{TcpListener, TcpStream};

mod resp;

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

async fn handle_connection(stream: TcpStream) -> Result<()> {
    let mut conn = resp::RespConnection::new(stream);

    loop {
        let value = conn.read_value().await?;

        if let Some(value) = value {
            let (command, args) = value.to_command()?;
            let response = match command.to_ascii_lowercase().as_ref() {
                "ping" => SimpleString("PONG".to_string()),
                "echo" => args.first().unwrap().clone(),
                _ => Error(format!("Command not implemented: {}", command)),
            };

            conn.write_value(response).await?;
        } else {
            break;
        }
    }

    Ok(())
}
