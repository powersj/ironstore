use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_config_file<P: AsRef<Path>>(path: P) -> io::Result<HashMap<String, Vec<String>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut config = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let Some(keyword) = parts.next() {
            let arguments: Vec<String> = parts.map(String::from).collect();
            config.insert(keyword.to_string(), arguments);
        }
    }

    Ok(config)
}
