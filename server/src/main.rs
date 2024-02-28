use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use log::info;
use std::str;

mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = settings::parse();

    let port = settings.get("port").and_then(|v| v.first()).unwrap();
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).await.expect("Failed to bind to address");
    info!("Listening on {}", address);

    loop {
        let (mut socket, addr) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let s = match str::from_utf8(&buf) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

                info!("Received {} bytes from {}: {}", n, addr, s);
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("Failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
