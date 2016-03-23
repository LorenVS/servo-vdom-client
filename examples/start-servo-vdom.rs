extern crate servo_vdom_client;

use servo_vdom_client::client::{ServoVdomOptions,ServoVdomClient};
use std::thread::sleep;
use std::time::Duration;

fn main() {
	let options = ServoVdomOptions {
		servo_bin_path: "/Users/lorenvs/git/servo-vdom/servo/target/debug/servo".to_string(),
		servo_ipc_path: "/Users/lorenvs/git/servo-vdom/servo/target/debug/servo-vdom.ipc".to_string()
	};

	let mut client = ServoVdomClient::new(options);
	client.open().unwrap();
	sleep(Duration::new(5, 0));
}