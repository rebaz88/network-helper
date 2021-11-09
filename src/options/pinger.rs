use colored::*;
use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;

pub fn run(ips: Vec<&str>) {
    let (pinger, results) = match Pinger::new(None, Some(56)) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e),
    };
    
    for ip in ips {
        pinger.add_ipaddr(ip);
    }
    
    pinger.run_pinger();

    loop {
        match results.recv() {
            Ok(result) => match result {
                Idle { addr } => {
                    let msg = format!("Idle Address {}.", addr);
                    println!("{}", msg.red());
                }
                Receive { addr, rtt } => {
                    pinger.remove_ipaddr(format!("{}", addr).as_str());
                    let msg = format!("Receive from address {} in {:?}.", addr, rtt);
                    println!("{}", msg.green());
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
    }
}