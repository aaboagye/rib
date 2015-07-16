// My attempt at creating an IRC bot in rust.
// While writing on an airplane, I think I will try to get the text parsing bit done.

// Obtain input from somewhere (maybe a text file)
// Parse the input and perform an action based upon the input
#![feature(ip_addr,lookup_host,convert,duration,thread_sleep)]
use std::fs::File;
use std::io;
use std::io::{Read,BufRead};
use std::vec;
use std::net;
use std::error::Error;

extern crate bufstream;
use bufstream::BufStream;

use std::thread;
use std::time::Duration;

pub mod irc;

fn main() {
    let conn = match irc::connect_to_server() {
        Ok(result) => bufstream::BufStream::new(result),
        Err(_) => panic!("Could not connect to server."),
    };
    println!("Connected.\n===BEGIN STREAM===");
    let mut bot = irc::IrcCon { stream : conn, nick : "aaboagye_bot" };
    bot.send_message(irc::IrcMessage::User, "foo").ok();
    bot.send_message(irc::IrcMessage::Nick, "aaboagye_bot").ok();
    let mut buffer = String::new();
    while true {
        println!("{}", bot.read_socket(&mut buffer).unwrap());
        thread::sleep(Duration::new(5, 0));
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
