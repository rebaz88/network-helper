use std::error::Error;

use csv;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Device {
    ip_address: String,
    device_type: String,
}

/// Reads data from a file into a reader and deserializes each record
///
/// # Error
///
/// If an error occurs, the error is returned to `main`.
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;

    // Retrieve and print header record
    // let headers = reader.headers()?;
    // println!("{:?}", headers);
    
    println!("\nThe application will check these devices\n");

    // `.deserialize` returns an iterator of the internal
    // record structure deserialized
    for result in reader.deserialize() {
        let device: Device = result?;

        println!("{:?}", device);
    }

    Ok(())
}

pub fn run() {
    
    if data_file_exist() {
        if let Err(e) = read_from_file(file_path()) {
            eprintln!("{}", e);
        }
    }
    
}


fn file_path<'a>() -> &'a str {
    "./data/data.csv"
}

fn data_file_exist() -> bool {
    let exists = Path::new(file_path()).exists();
    if !exists  {
        println!("The data file does not exist");
    }
    exists
}