use crate::utils::read_csv;
use crate::utils::read_csv::Device;
use crate::utils::pinger;
use crate::utils::ip_validation::is_valid_ip_address;
use std::error::Error;
use std::fmt;

pub struct PingAll<'a> {
    pub valid_ips : Vec<&'a Device>,
    invalid_ips : Vec<&'a Device>,
    passed_ips:  Vec<&'a Device>,
    failed_ips: Vec<&'a Device>, 
}

impl<'a> PingAll<'a> {
    
    fn new() -> PingAll<'a> {
        PingAll {
            valid_ips: Vec::new(),
            invalid_ips: Vec::new(),
            passed_ips: Vec::new(),
            failed_ips: Vec::new(),
        }
    }

    pub fn validate_all(&mut self) {
    
        let ips: Result<Vec<Device>, Box<dyn Error>> = read_csv::run();
        
        match ips {
            Ok(data) => {
                for ip in data.iter() {
                    let d: &Device = ip;
                    let validate = is_valid_ip_address(&ip.ip_address);
                    if validate.is_valid {
                        &self.valid_ips.push(d);
                    } else {
                        self.invalid_ips.push(d);
                    }
                }
            },
            Err(_) => panic!("Could not perform the operation!"),
        }
    }
}

impl fmt::Debug for PingAll<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PingAll")
         .field("valid_ips", &self.valid_ips)
         .field("invalid_ips", &self.invalid_ips)
         .field("passed_ips", &self.passed_ips)
         .field("failed_ips", &self.failed_ips)
         .finish()
    }
}

pub fn run() {
    let ping_all = PingAll::new().validate_all();
    // println!("{:?}", ping_all.valid_ips);  
}

