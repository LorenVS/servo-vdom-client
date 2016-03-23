use std::io::{Error, ErrorKind, Result};
use std::process::{Child, Command};
use ipc_channel::ipc;

pub struct ServoVdomOptions {
	pub servo_bin_path: String,
}

pub struct ServoVdomClient {
	opts : ServoVdomOptions,
	chan : Option<ipc::IpcSender<Vec<u8>>>,
	port: Option<ipc::IpcReceiver<Vec<u8>>>,
	child: Option<Child>
}

impl ServoVdomClient {
	pub fn new(opts: ServoVdomOptions) -> ServoVdomClient {
		ServoVdomClient {
			opts: opts,
			chan: None,
			port: None,
			child: None,
		}
	}

	pub fn open(&mut self) -> Result<()> {
		let (oss,token) = ipc::IpcOneShotServer::new().unwrap();
		println!("First token client: {}", token);

		let child = try!(
			Command::new(self.opts.servo_bin_path.clone())
				.arg("--vdom-ipc")
				.arg(token)
				.arg("-w")
				.spawn());

		let (port, first) = oss.accept().unwrap();
		let send_tok = String::from_utf8(first).unwrap();
		println!("Second token client: {}", send_tok);
		let chan = ipc::IpcSender::connect(send_tok).unwrap();
		chan.send(Vec::new()).unwrap();

		self.chan = Some(chan);
		self.port = Some(port);
		self.child = Some(child);
		Ok(())
	}

	pub fn send(&mut self, msg: Vec<u8>) -> Result<()> {
		if let Some(ref chan) = self.chan {
			chan.send(msg)
		} else {
			Err(Error::new(ErrorKind::NotConnected, "ServoVdomClient has not been opened"))
		}
	}
}
