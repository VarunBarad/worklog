use std::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct LogRecord {
    timestamp: String,
    contents: String,
}

fn main() {
    println!("Start!");

    let mut reader = csv::Reader::from_path("/Users/varunb/worklog.csv").unwrap();
    let mut records: Vec<LogRecord> = vec![];
    for record in reader.deserialize() {
        let record: LogRecord = record.unwrap();
        records.push(record);
    }

    print!("Log entry: ");
    std::io::stdout().flush().unwrap();
    let mut contents: String = String::new();
    std::io::stdin().read_line(&mut contents).unwrap();

    println!("You entered: {}", contents.trim());

    // let now = chrono::offset::Local::now();
    // records.push(LogRecord {
    //     timestamp: now.format("%Y-%m-%dT%H:%M%:z").to_string(),
    //     contents: "Hello, World!".to_string(),
    // });
    //
    // let mut writer = csv::Writer::from_path("/Users/varunb/worklog.csv").unwrap();
    // for record in records {
    //     writer.serialize(record).unwrap();
    // }
    // writer.flush().unwrap();
    //
    // println!("Total number of records: {}", records.len());
}
