use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                let mut buffer = [0; 512];

                loop {
                    let bytes_read = stream.read(&mut buffer).unwrap();

                    if bytes_read == 0 {
                        println!("Client closed the connection");
                        break;
                    }

                    stream.write("+PONG\r\n".as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
