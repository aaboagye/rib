// My attempt at creating an IRC bot in rust.
// While writing on an airplane, I think I will try to get the text parsing bit done.

// Obtain input from somewhere (maybe a text file)
// Parse the input and perform an action based upon the input
#![feature(ip_addr,lookup_host,convert,duration,thread_sleep)]
use std::thread;
use std::time::Duration;

extern crate bufstream;

pub mod irc;

fn main() {
    let conn = match irc::connect_to_server() {
        Ok(result) => bufstream::BufStream::new(result),
        Err(_) => panic!("Could not connect to server."),
    };
    println!("Connected.\n===BEGIN STREAM===");
    let mut bot = irc::IrcCon { stream : conn, nick : "aaboagye_bot" };
    bot.send_cmd(irc::IrcMessage::User).ok();
    bot.send_cmd(irc::IrcMessage::Nick).ok();
    let mut buffer = String::new();
    loop {
        println!("{}", bot.read_socket(&mut buffer).unwrap());
        thread::sleep(Duration::new(5, 0));
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead,Write};
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
        let list = test_strings.as_slice();
        {
            let mut f = File::create(TEST_FILE).unwrap();
            for i in 0..list.len() {
                f.write(list[i]).unwrap();
            }
        }
        
        {
            // Open the test file
            let f = File::open(TEST_FILE).unwrap();
            // Create a test bot
            let bs = BufStream::new(f);
            let mut test_bot = irc::IrcCon { stream: bs, nick: "test_bot" };
            let mut buffer = String::new();
            // Grab all available data from the stream and obtain a line iterator.
            let mut read_str = test_bot.read_socket(&mut buffer).unwrap().lines();

            // Verify the lines are what we expect.
            for i in 0..list.len() {
                let line = read_str.next().unwrap(); // Does not return the newline
                let expected = str::from_utf8(list[i]).unwrap().split("\n").nth(0).unwrap();
                if expected != line {
                    println!("{}-Expected: `{}`\nGot: `{}`", i, expected, line);
                    panic!();
                }
            }
            // TODO: Need to let the BufStream know we consumed all those bytes.
        }
    }
}
