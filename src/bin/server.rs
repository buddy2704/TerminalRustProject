mod client;

use std::io::{BufRead, BufReader, Write};
use std::{thread};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let handle = thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let request = handle_connection(&stream).to_lowercase();
            println!("request: {:?}", request);
            if request == "shutdown" {
                stream.write_all(b"Server is shutting down...\n").unwrap();
                println!("Shutdown command received. Stopping server...");
                break;
            }
            let response = process_request(request);

            let format_response = format!("{}\n", response);
            println!("response: {:?}", format_response);
            stream.write_all(format_response.as_bytes()).unwrap();
        }
    });
    handle.join().unwrap();
    Ok(())
}
fn handle_connection(stream: &TcpStream) -> String {
    let buf_reader = BufReader::new(stream.try_clone().unwrap());
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let output = request_line.trim().to_owned();
    output
}
fn process_request(request: String) ->String {
    match &request[..] {
        "hello" => "Hello".to_string(),
        _ => format!("{} is an invalid command!", request),
        }
}