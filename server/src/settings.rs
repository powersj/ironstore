use keyword_parser::parse_config_file;
use clap::{Arg, Command};
use simplelog::*;
use std::collections::HashMap;
use std::fs::File;
use log::LevelFilter;

pub fn parse() -> HashMap<String, Vec<String>> {
    let matches = Command::new("My Server")
        .version("1.0")
        .author("Author Name <email@example.com>")
        .about("An example server with configurable options")
        .arg(Arg::new("config")
             .short('c')
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
        )
        .arg(Arg::new("port")
             .long("port")
             .help("Sets the port to listen on")
             .default_value("8080")
        )
        .arg(Arg::new("loglevel")
             .long("loglevel")
             .help("Sets the logging level (error, warn, info, debug, trace)")
             .default_value("debug")
        )
        .arg(Arg::new("logfile")
             .long("logfile")
             .help("Sets the file to log to")
        )
        .get_matches();

    let mut settings = if let Some(config_path) = matches.get_one::<String>("config") {
        parse_config_file(config_path).expect("Failed to parse config file")
    } else {
        HashMap::new()
    };

    // Override settings with CLI args if provided
    if let Some(port) = matches.get_one::<String>("port") {
        settings.insert("port".to_string(), vec![port.to_string()]);
    }
    if let Some(loglevel) = matches.get_one::<String>("loglevel") {
        settings.insert("loglevel".to_string(), vec![loglevel.to_string()]);
    }
    if let Some(logfile) = matches.get_one::<String>("logfile") {
        settings.insert("logfile".to_string(), vec![logfile.to_string()]);
    }

    let log_level = settings.get("loglevel").and_then(|v| v.first()).map_or(LevelFilter::Info, |level| match level.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    });
    let default_log_file = "".to_string();
    let log_file = settings.get("logfile").and_then(|v| v.first()).unwrap_or(&default_log_file);
    if log_file.is_empty() {
        println!("Logging to stderr");
        let _ = WriteLogger::init(log_level, Config::default(), std::io::stderr());
    } else {
        println!("Logging to {}", log_file);
        let _ = WriteLogger::init(log_level, Config::default(), File::create(log_file).unwrap());
    }

    settings
}
