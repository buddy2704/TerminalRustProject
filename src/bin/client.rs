use std::{io,};
use std::io::{BufRead, BufReader, Write};
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
                let text_to_send = format!("{}\n", trimmed);

                let mut stream = match TcpStream::connect("127.0.0.1:7878"){
                   Ok(stream) => stream,
                    Err(error) => {
                        eprintln!("Failed to connect to server: {error}");
                        continue;
                    }
                };
                stream.write_all(text_to_send.as_bytes()).unwrap();
                handle_connection(stream);
            }
        }}
    Ok(())
}
fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());
    let mut response = String::new();
    if buf_reader.read_line(&mut response).is_ok() {
        println!("Server responded: {}", response.trim());
    }
}


