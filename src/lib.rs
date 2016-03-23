extern crate byteorder;
extern crate ipc_channel;
extern crate serde;
extern crate serde_json;

use serde_json::ser::Serializer;
use serde_json::de::from_reader;
use serde::ser::Serialize;
use std::fs::File;
use std::io::{Result, Error, ErrorKind};
use std::path::Path;
use ipc_channel::ipc;

pub mod client;
pub mod patch;

pub const MESSAGE_TYPE_PATCH : u8 = 0x00;

fn io_error() -> Error {
	Error::new(ErrorKind::InvalidData, "Serialization error")
}

pub struct ServoSide {
	pub chan: ipc::IpcSender<Vec<u8>>,
	pub port: ipc::IpcReceiver<Vec<u8>>
}

impl ServoSide {
	pub fn new(chan: ipc::IpcSender<Vec<u8>>, port: ipc::IpcReceiver<Vec<u8>>) -> ServoSide {
		ServoSide {
			chan: chan,
			port: port
		}
	}

	pub fn load(path : &Path) -> Result<ServoSide> {
		let mut file = try!(File::open(path));
		let (chan,port) = try!(from_reader(&mut file).map_err(|_| io_error()));
		Ok(ServoSide {
			chan: chan,
			port: port
		})
	}

	pub fn save(self, path : &Path) -> Result<()> {
		let file = try!(File::create(path));
		let mut serializer = Serializer::new(file);
		try!((self.chan,self.port).serialize(&mut serializer).map_err(|_| io_error()));
		Ok(())
	}
}

pub struct ClientSide {
	pub chan: ipc::IpcSender<Vec<u8>>,
	pub port: ipc::IpcReceiver<Vec<u8>>
}

impl ClientSide {
	pub fn new(chan: ipc::IpcSender<Vec<u8>>, port: ipc::IpcReceiver<Vec<u8>>) -> ClientSide {
		ClientSide {
			chan: chan,
			port: port
		}
	}

	pub fn load(path : &Path) -> Result<ClientSide> {
		let mut file = try!(File::open(path));
		let (chan,port) = try!(from_reader(&mut file).map_err(|_| io_error()));
		Ok(ClientSide {
			chan: chan,
			port: port
		})
	}

	pub fn save(self, path : &Path) -> Result<()> {
		let file = try!(File::create(path));
		let mut serializer = Serializer::new(file);
		try!((self.chan,self.port).serialize(&mut serializer).map_err(|_| io_error()));
		Ok(())
	}
}