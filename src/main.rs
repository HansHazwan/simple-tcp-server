use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let http_request = String::from_utf8_lossy(&buffer[..bytes_read]);

            match http_request.lines().next() {
                Some(line_request) => {
                    let request_uri = line_request.split(" ").nth(1).unwrap();
                    let message = request_uri.split("/").nth(1).unwrap();
                    let message = message.replace("%20", " ");

                    println!("Message: {}", message);

                    let content = fs::read_to_string("index.html").unwrap();
                    let content = content.replace("{}", &message);
                    let response = format!("HTTP/1.1 OK 200\r\nContent-Length: {}\r\n\r\n{}", content.len(), content);

                    stream.write_all(response.as_bytes()).unwrap();
                },
                None => {
                    println!("Error: The request are empty.");
                },
            }
        },
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening server on http://localhost:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            },
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

    Ok(())
}
