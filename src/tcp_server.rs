// https://github.com/ferrous-systems/rust-exercises/blob/main/exercise-book/src/tcp-server.md

use std::{io::{self, BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, sync::Mutex, thread};

fn handle_client(mut stream: TcpStream, log: &Mutex<Vec<usize>>) -> Result<(), io::Error> {
    let mut buffer = String::new();
    let mut reader = BufReader::new(stream.try_clone()?); 
    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(_) => {
                {
                    let mut log = log.lock().expect("Could not lock log");
                    log.push(buffer.len());
                }
                write!(stream, "{}", buffer)?;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

pub fn main() -> Result<(), io::Error> {
    println!("<<< TCP Server >>>");

    let log = Mutex::new(vec![]);
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    thread::scope(|s| {
        loop {
            let (stream, _) = listener.accept().unwrap();
            s.spawn({|| {
                    if let Err(e) = handle_client(stream, &log) {
                        eprintln!("Error: {}", e);
                    }
                }
            });
        }
    });
    Ok(())
}
