use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LogRecord {
    timestamp: String,
    contents: String,
}

impl LogRecord {
    fn format_timestamp(timestamp: DateTime<Local>) -> String {
        const FORMAT_STRING: &str = "%Y-%m-%dT%H:%M%:z";
        timestamp.format(FORMAT_STRING).to_string()
    }

    pub fn new(contents: String) -> LogRecord {
        LogRecord {
            timestamp: Self::format_timestamp(Local::now()),
            contents: contents.trim().to_owned(),
        }
    }

    pub fn new_with_timestamp(contents: String, timestamp: DateTime<Local>) -> LogRecord {
        LogRecord {
            timestamp: Self::format_timestamp(timestamp),
            contents: contents.trim().to_owned(),
        }
    }

    pub fn as_csv_row(&self) -> String {
        let mut csv_buffer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(vec![]);
        csv_buffer.serialize(self).unwrap();
        csv_buffer.flush().unwrap();

        String::from_utf8(csv_buffer.into_inner().unwrap()).unwrap()
    }
}
