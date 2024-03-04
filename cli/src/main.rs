use clap::{Arg, Command, ArgAction};
use std::net::TcpStream;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::Result;
use std::io;
use std::io::BufReader;
use std::io::BufRead;

fn main() -> io::Result<()> {
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
    let mut stream = TcpStream::connect(&address).expect("Failed to connect to server");

    if let Some(file_path) = matches.get_one::<String>("file") {
        send_commands_from_file(&mut stream, file_path);
    } else if let Some(commands) = matches.get_many::<String>("COMMAND") {
        let command: Vec<&String> = commands.collect::<Vec<_>>();
        let message = command.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
        send_command(&mut stream, &message);
    } else {
        interactive_mode(&mut stream);
    }

    Ok(())
}

fn send_commands_from_file(stream: &mut TcpStream, file_path: &str) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        send_command(stream, &line);
    }

    Ok(())
}

fn send_command(stream: &mut TcpStream, message: &str) -> Result<()> {
    stream.write_all(message.as_bytes());

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    if n > 0 {
        let response = std::str::from_utf8(&buffer[..n]).expect("Failed to parse response as UTF-8");
        println!("{}", response);
    }

    Ok(())
}

fn interactive_mode(stream: &mut TcpStream) -> Result<()> {
    println!("Enter commands to send to the server. Type 'exit' to quit.");
    print!("> ");
    io::stdout().flush();

    let stdin = io::stdin();
    let mut lines = BufReader::new(stdin).lines();
    while let Some(line) = lines.next() {
        let l = line.unwrap();
        if l.to_lowercase() == "exit" {
            break;
        }

        send_command(stream, &l);
        print!("> ");
        io::stdout().flush();
    }

    Ok(())
}
