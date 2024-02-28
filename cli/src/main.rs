use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
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
             .required(true))
        .arg(Arg::new("COMMAND").action(ArgAction::Append))
        .get_matches();

    let port: &String = matches.get_one("port").unwrap();
    let address = format!("127.0.0.1:{}", port);
    let mut stream = TcpStream::connect(&address).await.expect("Failed to connect to server");

    let command: Vec<&String> = matches.get_many::<String>("COMMAND").unwrap().collect::<Vec<_>>();
    let message = command.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
    stream.write_all(message.as_bytes()).await.expect("Failed to write to stream");

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await.expect("Failed to read from stream");
    if n > 0 {
        let response = std::str::from_utf8(&buffer[..n]).expect("Failed to parse response as UTF-8");
        println!("{}", response);
    } else {
        println!("No response received");
    }

    Ok(())
}
