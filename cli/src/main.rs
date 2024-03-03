use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt, BufReader, AsyncBufReadExt};
use clap::{Arg, Command, ArgAction};

#[tokio::main]
async fn main() -> io::Result<()> {
    let matches = Command::new("Client")
        .version("1.0")
        .author("Joshua Powers <powersj@fastmail.com>")
        .about("ironstore CLI client for sending messages to the server")
        .arg(Arg::new("port")
            .long("port")
            .help("The port to connect to")
            .default_value("8080")
            .required(false)
        )
        .arg(Arg::new("file")
            .long("file")
            .help("Read commands rom a file")
            .required(false)
        )
        .arg(Arg::new("COMMAND").action(ArgAction::Append))
        .get_matches();

    let port: &String = matches.get_one("port").unwrap();
    let address = format!("127.0.0.1:{}", port);
    let mut stream = TcpStream::connect(&address).await.expect("Failed to connect to server");

    if let Some(file_path) = matches.get_one::<String>("file") {
        send_commands_from_file(&mut stream, file_path).await?;
    } else if let Some(commands) = matches.get_many::<String>("COMMAND") {
        let command: Vec<&String> = commands.collect::<Vec<_>>();
        let message = command.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
        send_command(&mut stream, &message).await?;
    } else {
        interactive_mode(&mut stream).await?;
    }

    Ok(())
}

async fn send_commands_from_file(stream: &mut TcpStream, file_path: &str) -> io::Result<()> {
    for line in std::fs::read_to_string(file_path).unwrap().lines() {
        send_command(stream, line).await?;
    }

    Ok(())
}

async fn send_command(stream: &mut TcpStream, message: &str) -> io::Result<()> {
    stream.write_all(message.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    if n > 0 {
        let response = std::str::from_utf8(&buffer[..n]).expect("Failed to parse response as UTF-8");
        println!("> {}", response);
    }
    Ok(())
}

async fn interactive_mode(stream: &mut TcpStream) -> io::Result<()> {
    println!("Enter commands to send to the server. Type 'exit' to quit.");

    let stdin = io::stdin();
    let mut lines = BufReader::new(stdin).lines();
    while let Some(line) = lines.next_line().await? {
        if line.to_lowercase() == "exit" {
            break;
        }

        send_command(stream, &line).await?;
    }

    Ok(())
}
