mod client;

use std::io::{BufRead, BufReader, Write};
use std::{thread};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let _handle = thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        listener.incoming().for_each(|stream| {
            let mut stream = stream.unwrap();
            let request = handle_connection(&stream);
            println!("request: {:?}", request);
            let response = process_request(request);
            stream.write_all(response.as_bytes()).unwrap();
        });
    });
    Ok(())
}
fn handle_connection(stream: &TcpStream) -> String {
    let buf_reader = BufReader::new(stream.try_clone().unwrap());
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let output = request_line.trim().to_owned();
    output
}
fn process_request(request: String) ->String {
    loop {
        return match &request[..] {
            "Hello" => "Hello".to_string(),
            _ => ("{request} is an invalid command!").to_string(),
        }
    }
}