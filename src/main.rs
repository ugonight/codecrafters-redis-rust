use chrono::{Date, DateTime, Duration, Local, TimeZone};
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

struct RedisData {
    value: String,
    expired_datetime: DateTime<Local>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut dic: HashMap<String, RedisData> = HashMap::new();

        tokio::spawn(async move {
            let mut buf = [0; 1024];

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

                // echo コマンド
                if cmd_name == "echo" && req_array.len() > 4 {
                    res = format!("+{}\r\n", req_array.get(4).unwrap());
                }
                // set コマンド
                else if cmd_name == "set" && req_array.len() > 6 {
                    res = String::from("+OK\r\n");

                    let mut expired_datetime = Local::now();
                    let px_index = req_array.iter().position(|&e| e == "px").unwrap_or(0);
                    if px_index > 0 && px_index + 2 <= req_array.len() {
                        let px_str = req_array.get(px_index + 2).unwrap();
                        expired_datetime += Duration::milliseconds(px_str.parse().unwrap());
                    }

                    let data = RedisData {
                        value: req_array[6].to_string(),
                        expired_datetime,
                    };
                    dic.insert(req_array[4].to_string(), data);
                }
                // get コマンド
                else if cmd_name == "get" && req_array.len() > 6 {
                    let data = dic.get(&req_array[4].to_string()).unwrap();
                    let str = data.value.to_string();
                    let expired_datetime = data.expired_datetime;

                    if Local::now() > expired_datetime {
                        res = "$-1\r\n".to_string();
                    } else {
                        res = format!("+{}\r\n", str);
                    }
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
