use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut dic: HashMap<String, String> = HashMap::new();

            // In a loop, read data from the socket and write the data back.
            loop {
                let _n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let mut res = String::from("+PONG\r\n");
                let req_array: Vec<&str> =
                    std::str::from_utf8(&buf).unwrap().split("\r\n").collect();
                let cmd_name = req_array.get(2).unwrap_or(&"").to_string();

                println!("req: {}", req_array.join(", "));

                if cmd_name == "echo" && req_array.len() > 4 {
                    res = format!("+{}\r\n", req_array.get(4).unwrap());
                } else if cmd_name == "set" && req_array.len() > 6 {
                    res = String::from("+OK\r\n");
                    dic.insert(req_array[4].to_string(), req_array[6].to_string());
                } else if cmd_name == "get" {
                    let str = dic.get(&req_array[4].to_string()).unwrap().to_string();
                    res = format!("+{}\r\n", str);
                }

                // Write the data back
                if let Err(e) = socket.write_all(res.as_bytes()).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

/*
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
    loop {
        let mut buf: [u8; 255] = [0; 255];
        let size = stream.read(&mut buf).unwrap();

        if size == 0 {
            break;
        }

        println!("res: {}", String::from_utf8_lossy(&buf));

        let response = "+PONG\r\n";
        // if buf.len() > 0 {
        // stream.write(&buf).unwrap();
        // } else {
        stream.write(&response.as_bytes()).unwrap();
        // }
    }

    stream.flush().unwrap();
}
*/
