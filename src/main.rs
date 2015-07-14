// My attempt at creating an IRC bot in rust.
// While writing on an airplane, I think I will try to get the text parsing bit done.

// Obtain input from somewhere (maybe a text file)
// Parse the input and perform an action based upon the input
#![feature(ip_addr,lookup_host)]
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::net;
use std::net::IpAddr;


fn main() {
    // Obtain filename from the Args
    // if env::args().count() < 2 {
    //     panic!("Insufficient args.");
    // }
    let server_ip;
    print!("args: ");
    for a in env::args() {
        println!("  {}", a);
    }
/*
    let s = String::from(env::args().nth(1).unwrap());
    let file_path = Path::new(&s);
    let f = File::open(file_path).ok().unwrap();
    let chat_vec = get_input(f);
    for e in chat_vec {
        println!("{}", e);
    }
*/
    /* Obtain server IP address. */
    println!("Enter server to connect: ");
    let mut hostname = String::new();
    io::stdin().read_line(&mut hostname)
        .ok()
        .expect("Error reading line.");

    // Try DNS lookup first.
    hostname.trim();
    let mut results = match net::lookup_host(&hostname) {
        Ok(r) => r,
        Err(_) => panic!("net::lookup_host failed."),
    };
    // Only want the first result:
    let server = results.next();
    if(server.is_some()) {
        let server_ip = match server.unwrap() {
            Ok(s) => s.ip(),
            Err(_) => panic!("Missing SocketAddr! O:"),
        };
    } else {
        // If net::lookup_host doesn't work then just ask for the IP.
        println!("No results from net::lookup_host.");
        println!("Enter IP address instead: ");
        let mut ip_addr = String::new();
        io::stdin().read_line(&mut ip_addr)
            .ok()
            .expect("Error reading line.");

        let server_ip: IpAddr = match ip_addr.trim().parse() {
            Ok(addr) => addr,
            Err(_) => panic!("Invalid IP!"),
        };

    }
    conn = connect_to_server(server_ip).unwrap();

    println!("Connected.\n===BEGIN STREAM===");
}

// Return the strings as the commands
fn get_input(f: File) -> Vec<String> {
    let infile = io::BufReader::new(&f);
    let mut input: Vec<String> = Vec::<String>::new();
    for l in infile.lines() {
        // Append line to vector
        input.push(l.ok().unwrap());
    }
    input
    // Open the file
    // Read each line into a string
}

fn connect_to_server(addr: net::IpAddr) -> Result<net::TcpStream, io::Error> {
    let port: u16 = 6660;
    net::TcpStream::connect((addr, port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
