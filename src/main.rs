use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
struct LogRecord {
    timestamp: String,
    contents: String,
}

fn main() {
    let mut records: Vec<LogRecord> = vec![];
    {
        let mut reader = csv::Reader::from_path("/Users/varunb/worklog.csv").unwrap();
        for record in reader.deserialize() {
            let record: LogRecord = record.unwrap();
            records.push(record);
        }
    }

    print!("Log entry: ");
    std::io::stdout().flush().unwrap();
    let mut contents: String = String::new();
    std::io::stdin().read_line(&mut contents).unwrap();

    let now = chrono::offset::Local::now();
    records.push(LogRecord {
        timestamp: now.format("%Y-%m-%dT%H:%M%:z").to_string(),
        contents: contents.trim().to_owned(),
    });

    {
        let mut writer = csv::Writer::from_path("/Users/varunb/worklog.csv").unwrap();
        for record in records {
            writer.serialize(record).unwrap();
        }
        writer.flush().unwrap();
    }
}
