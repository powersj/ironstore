use clap::{Arg, Command, ArgAction};
use std::net::TcpStream;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};

fn main() {
    let matches = Command::new("Client")
        .version("1.0")
        .author("Joshua Powers <powersj@fastmail.com>")
        .about("ironstore CLI client for sending messages to the server")
        .arg(Arg::new("file")
            .long("file")
            .help("Read commands rom a file")
            .required(false)
        )
        .arg(Arg::new("port")
            .long("port")
            .help("The port to connect to")
            .default_value("8080")
            .required(false)
        )
        .arg(Arg::new("COMMAND")
            .action(ArgAction::Append)
        )
        .get_matches();

    let port: &String = matches.get_one("port").unwrap();
    let address = format!("127.0.0.1:{}", port);
    let mut stream = TcpStream::connect(address).expect("Failed to connect to server");

    if let Some(file_path) = matches.get_one::<String>("file") {
        send_commands_from_file(&mut stream, file_path);
    } else if let Some(commands) = matches.get_many::<String>("COMMAND") {
        let command: Vec<&String> = commands.collect::<Vec<_>>();
        let message = command.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
        send_command(&mut stream, &message);
    } else {
        interactive_mode(&mut stream);
    }
}

fn send_commands_from_file(stream: &mut TcpStream, file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        send_command(stream, &line);
    }
}

fn send_command(stream: &mut TcpStream, message: &str) {
    stream.write_all(message.as_bytes()).unwrap();

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).unwrap();
    if n > 0 {
        let response = std::str::from_utf8(&buffer[..n]).expect("Failed to parse response as UTF-8");
        println!("{}", response);
    }
}

fn interactive_mode(stream: &mut TcpStream) {
    println!("Enter commands to send to the server. Type 'exit' to quit.");
    print!("> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let lines = BufReader::new(stdin);
    for line in lines.lines() {
        let l = line.unwrap();
        if l.to_lowercase() == "exit" {
            break;
        }

        send_command(stream, &l);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
