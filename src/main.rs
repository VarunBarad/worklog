mod file_handler;
mod log_record;

use crate::file_handler::ensure_file_exists;
use crate::log_record::LogRecord;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    const FILE_PATH: &str = "/Users/varunb/worklog.csv";

    ensure_file_exists(FILE_PATH);

    println!("Enter the new worklog entry:");
    std::io::stdout().flush().unwrap();
    let mut contents: String = String::new();
    std::io::stdin().read_line(&mut contents).unwrap();

    let record = LogRecord::new(contents);

    {
        let mut file = OpenOptions::new().append(true).open(FILE_PATH).unwrap();
        write!(file, "{}", record.as_csv_row()).unwrap();
    }
}
