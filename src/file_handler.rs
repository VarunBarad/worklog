use std::fs::File;
use std::io::Write;
use std::ops::Not;
use std::path::Path;

pub fn ensure_file_exists(file_path: &str) {
    let file_path = Path::new(file_path);
    if file_path.exists().not() {
        let mut csv_buffer = csv::WriterBuilder::new().from_writer(vec![]);
        csv_buffer.write_record(&["timestamp", "contents"]).unwrap();
        csv_buffer.flush().unwrap();
        let header_string = String::from_utf8(csv_buffer.into_inner().unwrap()).unwrap();

        let mut file = File::create(file_path).unwrap();
        write!(file, "{}", header_string).unwrap();
    }
}
