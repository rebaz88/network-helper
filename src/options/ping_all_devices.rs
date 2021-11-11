use crate::utils::read_csv;
use crate::utils::read_csv::Device;
use crate::utils::pinger;
use crate::utils::ip_validation::is_valid_ip_address;
use std::error::Error;

pub fn run() {
    let ips: Result<Vec<Device>, Box<dyn Error>> = read_csv::run();
    
    match ips {
        Ok(data) => {
            let mut valid_devices: Vec<&Device> = Vec::new();
            for (i,ip) in data.iter().enumerate() {
                let d: &Device = ip;
                let validate = is_valid_ip_address(&ip.ip_address);
                if validate.is_valid {
                    valid_devices.push(d);
                } else {
                    println!("Invalid ip address found at line {}", i + 1);
                }
            }
            
            let valid_device_ips: Vec<&str> = valid_devices.iter()
                .map(|d| d.ip_address.as_str())
                .collect();
            
            pinger::run(&valid_device_ips);
        },
        Err(_) => panic!("Could not perform the operation!"),
    }
    
    
}