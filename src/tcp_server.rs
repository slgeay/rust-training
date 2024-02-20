// https://github.com/ferrous-systems/rust-exercises/blob/main/exercise-book/src/tcp-server.md

use std::{io::{self, BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, sync::mpsc::{self, Sender}, thread};

fn handle_client(mut stream: TcpStream, log: Sender<usize>) -> Result<(), io::Error> {
    let mut buffer = String::new();
    let mut reader = BufReader::new(stream.try_clone()?); 
    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(_) => {
                {
                    log.send(buffer.len()).unwrap();
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

    let mut log = vec![];

    let (sender, receiver) = mpsc::channel();

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    thread::scope(|s| {
        s.spawn(move || {
            while let Ok(size) = receiver.recv() {
                log.push(size);
                println!("{:?}", log);
            }
        });
        loop {
            let (stream, _) = listener.accept().unwrap();
            let sender = sender.clone();
            s.spawn({|| {
                    if let Err(e) = handle_client(stream, sender) {
                        eprintln!("Error: {}", e);
                    }
                }
            });
        }
    });
    Ok(())
}
