use std::io::{BufRead, BufReader};
use std::{io, thread};
use std::net::{TcpListener, TcpStream};

fn main() {
    let handle = thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
    }});
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input);
        let trimmed = input.trim();
        match &trimmed[..] {
            "Hello" => println!("Hello!"),
            "exit" => break,
            _ => println!("{} is an invalid command!", input),

        }
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(stream.try_clone().unwrap());
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) =
        match  &request_line[..] {
            "" =>{
                ("HTTP/1.1 200 OK", "test")
            }
            _ =>("HTTP/1.1 404 NOT FOUND", "test")
        };


}