mod client;

use rusqlite::Result;
use std::io::{BufRead, BufReader, Write};
use std::{thread};
use std::net::{TcpListener, TcpStream};
use rusqlite::Connection;

fn create_var(name: &str, val: &str) -> rusqlite::Result<(String)> {
    let conn = Connection::open("thedata.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (id INTEGER PRIMARY KEY, name TEXT UNIQUE, val TEXT)",
        [],
    )?;

    conn.execute(
        "INSERT INTO items (name, val) VALUES (?1, ?2)",
        [name, val],
    )?;
    return Ok("ok".to_string());
}

fn get_value(name: &str) -> Result<Option<String>> {
    let conn = Connection::open("thedata.db")?;
    let mut stmt = conn.prepare("SELECT val FROM items WHERE name = ?1")?;
    let result = stmt.query_row([name], |row| {
        let value: String = row.get(0)?;
        Ok(value)
    });
    match result {
        Ok(val) => Ok(Some(val)),                              // Found it!
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),  // No matching name found
        Err(e) => Err(e),                                      // Database error
    }
}


fn main() -> std::io::Result<()> {
    let handle = thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let request = handle_connection(&stream);
            let request_trimmed = request.trim();
            let vals: Vec<&str> = request_trimmed.split_whitespace().collect();
            if vals.is_empty() {
                stream.write_all(b"Wrong number of arguments\n").unwrap();
                continue;
            }
            println!("request: {:?}", request);
            let command = vals[0].to_string().to_lowercase();
            if command == "shutdown" {
                stream.write_all(b"Server is shutting down...\n").unwrap();
                println!("Shutdown command received. Stopping server...");break;
            }
            let response = process_request(command, vals);

            let format_response = format!("{}\n", response);
            println!("response: {:?}", response);
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
fn process_request(request: String, rest: Vec<&str>) ->String {
    match &request[..] {
        "hello" => "Hello".to_string(),
        "set" => {
            if rest.len() != 3 {
                return "Wrong number of arguments".to_string();
            }
            let arg1: &str = &rest[1];
            let arg2: &str = &rest[2];
            match create_var(arg1, arg2) {
                Ok(_) => "Variable saved Successfully!".to_string(),
                Err(e) => format!("Database Error: Could not save variable: {}", e),
            }
        }
        "get" =>{
            if rest.len() != 2 {
                return "Wrong number of arguments".to_string();
            }
            let arg1: &str = &rest[1];
            let value = get_value(arg1).unwrap();
            if value.is_some(){
                return value.unwrap().to_string();
            }
            else{
                return "Value Null or Name not found".to_string();
            }
        }
        _ => format!("{} is an invalid command!", request),
        }
}