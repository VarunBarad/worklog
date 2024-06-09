mod file_handler;

use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use std::io::Write;
use crate::file_handler::ensure_file_exists;

#[derive(Debug, Deserialize, Serialize)]
struct LogRecord {
    timestamp: String,
    contents: String,
}

fn main() {
    const FILE_PATH: &str = "/Users/varunb/worklog.csv";

    ensure_file_exists(FILE_PATH);

    println!("Enter the new worklog entry:");
    std::io::stdout().flush().unwrap();
    let mut contents: String = String::new();
    std::io::stdin().read_line(&mut contents).unwrap();

    let now = chrono::offset::Local::now();
    let record = LogRecord {
        timestamp: now.format("%Y-%m-%dT%H:%M%:z").to_string(),
        contents: contents.trim().to_owned(),
    };

    let mut csv_buffer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(vec![]);
    csv_buffer.serialize(record).unwrap();
    csv_buffer.flush().unwrap();
    let record_string = String::from_utf8(csv_buffer.into_inner().unwrap()).unwrap();

    {
        let mut file = OpenOptions::new().append(true).open(FILE_PATH).unwrap();
        write!(file, "{}", record_string).unwrap();
    }
}
