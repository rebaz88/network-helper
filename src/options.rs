use std::io::{stdin,stdout,Write};
use std::process;
use colored::*;
pub mod ip_validation;
pub mod pinger;

use ip_validation::{IPAddressValidation,is_valid_ip_address};

pub fn run() {
    println!("\n=================");
    println!("{}", " Network Helper".green());
    println!("=================");
    
    loop {
        options();
    }
    
}

fn options() {
    
    println!("Please select one of the options");
    println!("1- Reset certain switch");
    println!("2- Reset all switches");
    println!("Any other key to exit");
    
    let s: String = prompt();
    
    match s.as_str() {
        "" => {
            println!("*** Please enter a key! ***\n");
        }
        "1" => {
            let ip: String = ip_prompt();
            
            // check if can ping the swith
            pinger::run(vec![ip.as_str()]);
        }
        "2" => {
            println!("You want to reset all switches");
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
    
    println!("{}", "Please enter an ip address".magenta());
    
    let ip: String = prompt();
    
    loop {
        let validate_ip: IPAddressValidation = is_valid_ip_address(&ip);
        if ! validate_ip.is_valid {
            println!("{}\n", validate_ip.message.red());
            ip_prompt();
        }  else {
            return ip;
        }
    }
}

