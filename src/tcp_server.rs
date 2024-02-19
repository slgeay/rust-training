// https://github.com/ferrous-systems/rust-exercises/blob/main/exercise-book/src/tcp-server.md

use std::{io::{self, BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread};

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut buffer = String::new();
    let mut reader = BufReader::new(stream.try_clone()?); 
    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(_) => {
                write!(stream, "{}", buffer)?;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

pub fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    loop {
        let (stream, _) = listener.accept()?;
        thread::spawn(move || {
            handle_client(stream).unwrap();
        });
    }
}
