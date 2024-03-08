use log::info;
use std::collections::HashMap;
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

/// Create TCP listener with provided host and port.
pub async fn create_listener(settings:HashMap<String, Vec<String>>) -> TcpListener {
    let host: &String = settings.get("host").and_then(|val| val.first()).unwrap();
    let port: &String = settings.get("port").and_then(|val| val.first()).unwrap();
    let address = format!("{}:{}", host, port);
    info!("Listening on {}", address);
    TcpListener::bind(&address).await.expect("Failed to bind to address")
}

/// Send a string over TCP stream.
pub async fn send_over_tcp(socket: &mut TcpStream, message: Result<String, String>) -> Result<(), Box<dyn Error>> {
    // Prepare the message to send based on the result of match_action
    let message_to_send = match message {
        Ok(msg) => msg,
        Err(err_msg) => err_msg,
    };

    // Send the message (either Ok or Error) over TCP
    socket.write_all(message_to_send.as_bytes()).await?;
    Ok(())
}
