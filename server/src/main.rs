use log::info;
use std::str;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncReadExt;

mod action;
mod listener;
mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = settings::parse();
    let listener = listener::create_listener(settings).await;
    let shared_data = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    loop {
        let (mut socket, _) = listener.accept().await?;
        let data_clone = shared_data.clone();

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let _n = match socket.read(&mut buf).await {
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

                let commands:Vec<String> = s.trim_end_matches('\x00').split_whitespace().map(str::to_string).collect();
                let result = match_action(data_clone.clone(), commands).await; // Call match_action and get its result
                let _ = listener::send_over_tcp(&mut socket, result).await; // Send the result over TCP
            }
        });
    }
}


// Updated match_action to return Result<String, String>
async fn match_action(shared_data: Arc<Mutex<HashMap<String, String>>>, commands: Vec<String>) -> Result<String, String> {
    if commands.is_empty() {
        return Err("No command provided".to_string());
    }

    match commands[0].as_str() {
        "del" => {
            if commands.len() < 2 {
                return Err("No key provided for delete".to_string());
            }
            let key = commands[1].clone();
            action::del(shared_data, key).await
        },
        "get" => {
            if commands.len() < 2 {
                return Err("No key provided for get".to_string());
            }
            let key = commands[1].clone();
            action::get(shared_data, key).await
        },
        "set" => {
            if commands.len() < 3 {
                return Err("Not enough arguments for set".to_string());
            }
            let key = commands[1].clone();
            let value = commands[2].clone();
            action::set(shared_data.clone(), key, value).await
        },
        _ => Err("Unsupported command".to_string()),
    }
}
