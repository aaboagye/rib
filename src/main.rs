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
    use std::fs::File;
    use std::io::Write;
    use irc::*;
    use bufstream::BufStream;
    use std::str;

    #[test]
    fn it_works() {
    }
    
    // Test to see if you can read line by line from the stream.
    #[test]
    fn test_line_by_line() {
        const TEST_FILE: &'static str = "test_line_by_line.txt";
        // Create a sample text file and populate it with test strings
        let test_strings = vec![b"line 1\r\n", b"line 2\r\n", b"line 3\r\n"];
        {
            let mut f = File::create(TEST_FILE).unwrap();
            for s in test_strings {
                f.write(s).unwrap();
            }
        }
        
        {
            // Open the test file
            let f = File::open(TEST_FILE).unwrap();
            // Create a test bot
            let bs = BufStream::new(f);
            let mut test_bot = irc::IrcCon { stream: bs, nick: "test_bot" };
            let mut buffer = String::new();
            // Verify the strings are equal
            let test_strings = vec![b"line 1\r\n", b"line 2\r\n", b"line 3\r\n"];
            for s in test_strings {
                let read_str = test_bot.read_socket(&mut buffer).unwrap().as_bytes();
                if s.len() != read_str.len() {
                    println!("Strings are NOT the same length.");
                    println!("Expected: `{}`\nGot: `{}`",str::from_utf8(s).unwrap(), str::from_utf8(read_str).unwrap());
                    assert!(false);
                }
            }
        }
    }
}
