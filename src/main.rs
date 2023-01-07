// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf: [u8; 255] = [0; 255];
    stream.read(&mut buf).unwrap();

    // println!("{}", String::from_utf8_lossy(&buf));

    let response = "+PONG\r\n";
    // if buf.len() > 0 {
    // stream.write(&buf).unwrap();
    // } else {
    stream.write(&response.as_bytes()).unwrap();
    // }

    // stream.flush().unwrap();
}
