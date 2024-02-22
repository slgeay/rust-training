use std::sync::{Arc, Mutex};

// 1. Add Tokio dependency X
// 2. Convert all functions to async
// 3. Remove std imports for IO, fix with Tokio versions
// 4. Replace for in with while let await
// use std::{
//     // io::{self, BufRead, BufReader, BufWriter, Write},
//     net::{TcpListener, TcpStream},
//     sync::{Arc, Mutex},
//     thread,
// };
use tokio::{io::{self, Interest}, net::{TcpListener, TcpStream}};

async fn handle_client(
    stream: TcpStream,
    log: &Mutex<Vec<usize>>,
) -> Result<(), io::Error> {

    loop {
        let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await?;
        let mut data: Vec<u8> = vec![0; 1024];

        if ready.is_readable() {
            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match stream.try_read(&mut data) {
                Ok(n) => {
                    if n > 0 {
                        println!("read {} bytes", n);
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }

        }
        if data.len() == 0 {
            continue;
        }

        if ready.is_writable() {
            // Try to write data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match stream.try_write(&data) {
                Ok(n) => {
                    println!("write {} bytes", n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), io::Error> {
    let log = Arc::new(Mutex::new(vec![]));

    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        let stream = listener.accept().await;
        let Ok((stream, _)) = stream else {
            eprintln!("Bad connection");
            continue;
        };

        let log = Arc::clone(&log);
        tokio::spawn(async move {
            handle_client(stream, &log).await.unwrap();
        });
    }
    // Ok(())
}
