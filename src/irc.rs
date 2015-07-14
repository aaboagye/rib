use std::net;
use std::io;
use std::io::{Read,Write};

pub struct IrcCon<'a> {
    pub stream: net::TcpStream,
    pub nick: &'a str,
}

pub enum IrcMessage {
	Pass,
	Nick,
	User,
	Quit,
}

impl <'a> IrcCon<'a> {
	pub fn send_message(&mut self, t: IrcMessage, msg: &str) -> Result<(), io::Error> {
		/* Format a message and send it to the server. */
		let mut s = String::with_capacity(512);
		match t {
			IrcMessage::User => {
				s.push_str("USER ");
				s.push_str(self.nick);
				s.push_str(" 0 * :*");
			},
			IrcMessage::Nick => {
				s.push_str("NICK ");
				s.push_str(self.nick);
			},
			_ => panic!("That message type is not yet implemented."),
		};
		s.push_str("\r\n");
		println!("<{}", s.as_str());

		/* Send it out the TcpStream. */
		self.stream.write_all(s.as_bytes())
	}
}