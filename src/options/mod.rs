use std::io::{stdin,stdout,Write};
use std::process;
use crate::utils::*;
use ip_validation::{IPAddressValidation,is_valid_ip_address};
pub mod ping_all_devices;

pub fn run() {
    println!("\n=================");
    println!("{}", " Network Helper");
    println!("=================");
    
    loop {
        options();
    }
    
}

fn options() {
    
    println!("Please select one of the options");
    println!("1- Ping all devices");
    println!("2- Reset certain switch");
    println!("3- Reset all switches");
    println!("Any other key to exit");
    
    let s: String = prompt();
    
    match s.as_str() {
        "" => {
            println!("*** Please enter a key! ***\n");
        }
        
        "1" => {
            ping_all_devices::run();
        }
        
        "2" => {
            let ip: String = ip_prompt();
            
            // check if can ping the swith
            pinger::run(&vec![ip.as_str()]);
        }
        
        "3" => {
            println!("");
        }
        _ => {
            println!("Good bye!");
            process::exit(0x0100);
        }
    }
}

fn prompt<'a>() -> String {
    
    let mut s = String::new();
    
    let _= stdout().flush();
    
    stdin().read_line(&mut s).expect("Please make a selection");
    
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    } 
    
    s
}

fn ip_prompt<'a>() -> String {
    
    println!("{}", "Please enter an ip address");
    
    let ip: String = prompt();
    
    loop {
        let validate_ip: IPAddressValidation = is_valid_ip_address(&ip);
        if ! validate_ip.is_valid {
            println!("{}\n", validate_ip.message);
            ip_prompt();
        }  else {
            return ip;
        }
    }
}

