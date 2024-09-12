use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::env;
use std::fs;

const HELLO_PAGE_FILENAME: &str = "hello.html";

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let request_line = http_request.get(0).unwrap();
    let request_uri = request_line.split(" ").nth(1).unwrap();

    log::info!("HTTP Request: {:#?}\nRequest URI: {}", http_request, request_uri);

    let message = request_uri.strip_prefix("/").unwrap().replace("%20", " ");
    let content = format!("<h1>Hello {} !!!</h1>", message);
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", content.len(), content);

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    log::info!("Running server on http://localhost:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::info!("New connection !!!");

                thread::spawn(move || {
                    handle_connection(stream);
                });
            },
            Err(err) => {
                log::error!("Connection failed: {:#?}", err);
            }
        }
    }

    Ok(())
}
