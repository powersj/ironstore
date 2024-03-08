use std::collections::HashMap;
use std::error::Error;
use std::str;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncReadExt;

mod action;
mod listener;
mod settings;

#[tokio::main]
/// Main function.
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
                    Ok(num) => num,
                    Err(err) => {
                        eprintln!("Failed to read from socket; err = {err:?}");
                        return;
                    }
                };

                let msg = match str::from_utf8(&buf) {
                    Ok(val) => val,
                    Err(err) => panic!("Invalid UTF-8 sequence: {err}"),
                };

                let commands:Vec<String> = msg.trim_end_matches('\x00')
                    .split_whitespace()
                    .map(str::to_string).collect();
                let result = match_action(
                    data_clone.clone(),
                    commands
                );
                let _: Result<(), Box<dyn Error>> = listener::send_over_tcp(
                    &mut socket,
                    result
                ).await;
            }
        });
    }
}


/// Determine action to run and return result.
fn match_action(
    shared_data: Arc<Mutex<HashMap<String, String>>>,
    commands: Vec<String>
) -> Result<String, String> {
    if commands.is_empty() {
        return Err("No command provided".to_owned());
    }

    match commands.first().unwrap().as_str() {
        "del" => {
            if commands.len() < 2 {
                return Err("No key provided for delete".to_owned());
            }
            action::del(shared_data, &commands[1])
        },
        "flushall" => {
            action::flushall(shared_data)
        },
        "get" => {
            if commands.len() < 2 {
                return Err("No key provided for get".to_owned());
            }
            action::get(shared_data, &commands[1])
        },
        "info" => {
            action::info()
        },
        "keys" => {
            action::keys(shared_data)
        },
        "ping" => {
            action::ping()
        },
        "set" => {
            if commands.len() < 3 {
                return Err("Not enough arguments for set".to_owned());
            }
            action::set(shared_data, &commands[1], &commands[2])
        },
        _ => Err("Unsupported command".to_owned()),
    }
}
