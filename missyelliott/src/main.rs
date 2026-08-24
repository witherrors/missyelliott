//
// This is a learning project so were gonna be overly verbose and list steps out
//
// Step 1 - collect args and validate
// Step 2 - connect
//
//

use std::net::{Ipv4Addr, TcpStream};
use std::env;
use std::process;

//takes ipv4 string, checks if valid format
fn validate_ip(ip: &str) -> bool {
    match ip.parse::<Ipv4Addr>() {
        Ok(_ip) => {
            println!("[+] valid ipv4 address");
            true
        }
        Err(_err) => {
            eprintln!("[-] not valid ipv4 address");
            false
        }
    }
}

fn main() {

    //collect command line args && validate
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("[-] Usage: {} <ipv4_address> <port>", args[0]);
        process::exit(1);
    }

    let ip = &args[1];
    let _port = &args[2];

    if !validate_ip(ip) {
        eprintln!("[-] exiting program due to invalid IP address.");
        process::exit(1);
    }

}