extern crate bufstream;

use std::net;
use std::io;
use std::io::{Read,Write,BufRead};
use std::str;
use bufstream::BufStream;

const IRC_MESSAGE_MAX_LEN: usize = 510;

pub struct IrcCon<'a, T: Read + Write> {
    pub stream: bufstream::BufStream<T>,
    pub nick: &'a str,
}

pub enum IrcMessage {
	Nick,
	Pass,
	Ping,
	User,
}

impl <'a, T: Read + Write> IrcCon<'a, T> {
	pub fn send_cmd(&mut self, t: IrcMessage, m: Option<&str>) -> Result<(), io::Error> {
		/* Format a message and send it to the server. */
		let mut s = String::with_capacity(IRC_MESSAGE_MAX_LEN);
		match t {
			IrcMessage::Nick => {
				s.push_str("NICK ");
				s.push_str(self.nick);
			},
			IrcMessage::Pass => {
				s.push_str("PASS ");
			},
			IrcMessage::Ping => {
				s.push_str("PING ");
				s.push_str(m.unwrap());
			},
			IrcMessage::User => {
				s.push_str("USER ");
				s.push_str(self.nick);
				s.push_str(" ");
				s.push_str(self.nick);
				s.push_str(" * :*");
			},
		};
		s.push_str("\r\n");
		println!("<{}", s.as_str());

		/* Send it out the stream. */
		self.stream.write(s.as_bytes()).unwrap();
		self.stream.flush()
	}

	pub fn read_socket<'b>(&'b mut self) -> Result<Vec<&str>, io::Error> {
		// Try and read all available data from the stream.
		match BufStream::fill_buf(&mut self.stream) {
			Ok(b) => Ok(str::from_utf8(b).unwrap().split("\r\n").collect()),
			Err(e) => Err(e),
		}
	}

	pub fn update(&mut self, bytes_consumed: usize) {
		self.stream.consume(bytes_consumed);
		println!("consumed: {}B", &bytes_consumed);
	}
}

pub fn connect_to_server() -> Result<net::TcpStream, io::Error> {
    // Obtain server IP address from hostname.
    println!("Enter server to connect: ");
    let mut hostname = String::new();
    io::stdin().read_line(&mut hostname)
        .ok()
        .expect("Error reading line.");

    hostname.trim();
    let mut results = match net::lookup_host(&hostname.as_str()) {
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
    let port: u16 = 6667;
    let conn = try!(net::TcpStream::connect((server_ip, port)));
    Ok(conn)
}

