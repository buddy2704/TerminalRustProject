use std::{io, thread};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

fn main () -> std::io::Result<()> {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input).expect("TODO: panic message");
        let trimmed = input.trim();
        match trimmed {
            "exit" => break,
            _=> {
                let text_to_send = trimmed.to_string();

                let handle = thread::spawn(move || {

                let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
                stream.write_all(text_to_send.as_bytes()).unwrap();
                    handle_connection(stream);
            });
        }};
        }
    Ok(())
    }
fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());
    let response = buf_reader.read_to_string(&mut String::new()).unwrap();
    println!("{}", response);
}


