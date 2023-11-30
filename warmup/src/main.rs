use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
fn main() {
    #[derive(Debug, Deserialize)]
    struct Record {
        key: i32,
        value: String,
    }
    fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(File::open(file_path)?);
        for result in rdr.deserialize() {
            let record: Record = result?;
            println!("key: {}, value: {}", record.key, record.value);
        }
        Ok(())
    }
    fn main() {
        if let Err(err) = read_csv("testdata/data.csv") {
            eprintln!("Error reading CSV file: {}", err);
        }
    }
    main();
}
