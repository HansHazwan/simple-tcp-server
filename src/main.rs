use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use html_escape::encode_text;
use threadpool::ThreadPool;
use std::sync::Arc;
use std::thread;
use std::env;
use std::fs;

const THREAD_POOL_SIZE: usize = 4;

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .take_while(|line| match line {
            Ok(line) => !line.is_empty(),
            Err(_) => false,
        })
        .collect::<Result<Vec<_>, _>>()?;

    if http_request.is_empty() {
        log::warn!("Received an empty HTTP request");
        return Ok(());
    }

    let request_line = http_request[0].split_whitespace().collect::<Vec<&str>>();
    if request_line.len() < 2 {
        log::warn!("Malformed request: {:?}", request_line);
        return Ok(())
    }

    let request_uri = request_line[1];
    log::info!("HTTP Request: {:?}", request_uri);

    let message = encode_text(request_uri.strip_prefix("/").unwrap_or("")).replace("%20", " ");
    let content = format!("<h1>Hello {} !!!</h1>", message);

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        content.len(),
        content
    );

    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let pool = Arc::new(ThreadPool::new(THREAD_POOL_SIZE));

    log::info!("Running server on http://localhost:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::info!("New connection established");

                let pool = Arc::new(&pool);
                pool.execute(move || {
                    if let Err(e) = handle_connection(stream) {
                        log::error!("Failed to handle connection: {:?}", e);
                    }
                });
            },
            Err(err) => {
                log::error!("Connection failed: {:#?}", err);
            },
        }
    }

    Ok(())
}
