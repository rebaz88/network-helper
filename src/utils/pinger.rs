use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;

pub fn run(ips: &Vec<&str>) {
    let (pinger, results) = match Pinger::new(None, Some(56)) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e),
    };
    
    for ip in ips {
        pinger.add_ipaddr(ip);
    }
    
    pinger.run_pinger();
    
    let mut remaining_ips = ips.clone();
    
    loop {
        match results.recv() {
            Ok(result) => match result {
                Idle { addr } => {
                    println!("Idle address {}", addr);
                }
                Receive { addr, rtt } => {
                    let ip_str = format!("{}", addr);
                    remaining_ips.retain(|x| *x != ip_str);
                    pinger.remove_ipaddr(ip_str.as_str());
                    println!("Receive from address {} in {:?}.", addr, rtt);
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
        
        if remaining_ips.len() == 0 {
            pinger.stop_pinger();
            println!("\nPinging succeed!\n");
            break;
        }
    }
}