use std::{io, sync::Arc, sync::Mutex};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt as _, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(
    mut stream: TcpStream,
    log: &Mutex<Vec<usize>>,
) -> Result<(), io::Error> {
    let (reader, writer) = stream.split();
    let reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);
    let mut lines = reader.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        // the code block here forces the MutexGuard drop to unlock the mutex
        {
            let mut log = log.lock().unwrap();
            log.push(line.len());
        }
        writer.write_all(b"> ").await?;
        writer.write_all(line.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
    }
    Ok(())
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
}
