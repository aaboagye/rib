// My attempt at creating an IRC bot in rust.
// While writing on an airplane, I think I will try to get the text parsing bit done.

// Obtain input from somewhere (maybe a text file)
// Parse the input and perform an action based upon the input
#![feature(ip_addr,lookup_host,convert)]
use std::fs::File;
use std::io;
use std::io::{Read,BufRead};
use std::vec;
use std::net;

pub mod irc;

fn main() {
    let conn = match connect_to_server() {
        Ok(result) => result,
        Err(_) => panic!("Could not connect to server."),
    };
    println!("Connected.\n===BEGIN STREAM===");
    let mut bot = irc::IrcCon { stream : conn, nick : "aaboagye_bot" };
    bot.send_message(irc::IrcMessage::User, "foo").ok();
    bot.send_message(irc::IrcMessage::Nick, "aaboagye_bot").ok();

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
}

fn connect_to_server() -> Result<net::TcpStream, io::Error> {
    // Obtain server IP address from hostname.
    println!("Enter server to connect: ");
    let mut hostname = String::new();
    io::stdin().read_line(&mut hostname)
        .ok()
        .expect("Error reading line.");

    hostname.trim();
    let mut results = match net::lookup_host(&hostname) {
        Ok(r) => r,
        Err(_) => panic!("net::lookup_host failed."),
    };
    // Only want the first result.
    let server = results.next();
    let server_ip = {
        if server.is_some() {
            match server.unwrap() {
                Ok(s) => s.ip(),
                Err(_) => panic!("Missing SocketAddr! O:"),
            }
        } else {
            // If net::lookup_host doesn't work then just ask for the IP.
            println!("No results from net::lookup_host.");
            println!("Enter IP address instead: ");
            let mut ip_addr = String::new();
            io::stdin().read_line(&mut ip_addr)
                .ok()
                .expect("Error reading line.");

            match ip_addr.trim().parse() {
                Ok(addr) => addr,
                Err(_) => panic!("Invalid IP!"),
            }
        }
    };
    let port: u16 = 6660;
    net::TcpStream::connect((server_ip, port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
