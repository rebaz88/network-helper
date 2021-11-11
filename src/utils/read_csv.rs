use std::error::Error;

use csv;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Device {
    pub ip_address: String,
    pub device_type: String,
}

/// Reads data from a file into a reader and deserializes each record
///
/// # Error
///
/// If an error occurs, the error is returned to `main`.
fn read_from_file(path: &str) -> Result<Vec<Device>, Box<dyn Error>> {
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;

    // Retrieve and print header record
    // let headers = reader.headers()?;
    // println!("{:?}", headers);
    
    // `.deserialize` returns an iterator of the internal
    // record structure deserialized
    let mut ips: Vec<Device> = Vec::new();
    for result in reader.deserialize() {
        let record: Device = result?;
        ips.push(record);
    }
    
    Ok(ips)
}

pub fn run() -> Result<Vec<Device>, Box<dyn Error>> {
    
    if ! data_file_exist() {
        return Err("The file does not exist".into());
    }
        
    read_from_file(file_path())
}


fn file_path<'a>() -> &'a str {
    "./data/data.csv"
}

fn data_file_exist() -> bool {
    Path::new(file_path()).exists()
}