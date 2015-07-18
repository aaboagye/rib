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
    loop {
        thread::sleep(Duration::new(5, 0));
    }

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
        let test_strings = vec![b"line 1", b"line 2", b"line 3"];
        let list = test_strings.as_slice();
        {
            let mut f = File::create(TEST_FILE).unwrap();
            for i in 0..list.len() {
                f.write(list[i]).unwrap();
                f.write(b"\r\n").unwrap();
            }
        }
        
        {
            // Open the test file
            let f = File::open(TEST_FILE).unwrap();
            // Create a test bot
            let bs = BufStream::new(f);
            let mut test_bot = irc::IrcCon { stream: bs, nick: "test_bot" };

            // Grab all available data from the stream.
            let read_buf = test_bot.read_socket().unwrap();

            // Verify the lines are what we expect.
            for i in 0..list.len() {
                let expected = str::from_utf8(list[i]).unwrap();
                if read_buf[i] != expected {
                    println!("{}-Expected: `{}`\nGot: `{}`", i, expected, read_buf[i]);
                    panic!();
                }
            }
            // TODO: Need to let the BufStream know we consumed all those bytes.
        }
    }
}
