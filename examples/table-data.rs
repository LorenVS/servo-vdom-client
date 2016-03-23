extern crate servo_vdom_client;

use servo_vdom_client::client::{ServoVdomOptions,ServoVdomClient};
use servo_vdom_client::patch::*;
use std::io::Result;
use std::thread::sleep;
use std::time::Duration;

fn main() {
	let options = ServoVdomOptions {
		servo_bin_path: "/Users/lorenvs/git/servo-vdom/servo/target/debug/servo".to_string(),
	};

	let mut client = ServoVdomClient::new(options);
	client.open().unwrap();

	let msg = try_write_init_patch().unwrap();
	assert!(client.send(msg).is_ok());

	let mut num : u64 = 0;
	loop {
		sleep(Duration::new(0, 1000000));

		num += 1;
		let msg = try_write_update_patch(num).unwrap();
		assert!(client.send(msg).is_ok());
	}
}

fn try_write_init_patch() -> Result<Vec<u8>> {
	let mut msg : Vec<u8> = Vec::new();
	try!(msg.write_msg_type(MessageType::Patch));

	try!(msg.write_patch_type(PatchType::Replace, 1));
	try!(msg.write_el(1, ElementName::Body));
	try!(msg.write_end_list());

	try!(msg.write_el(100, ElementName::Style));
	try!(msg.write_end_list());
	try!(msg.write_text(101, r#"

	"#));
	try!(msg.write_end_list());

	try!(msg.write_el(150, ElementName::Div));
	try!(msg.write_end_list());
	try!(msg.write_text(152, "Patches: "));
	try!(msg.write_text(151, ""));
	try!(msg.write_end_list());

	try!(msg.write_el(200, ElementName::Table));
	try!(msg.write_end_list());

	try!(msg.write_el(201, ElementName::Tbody));
	try!(msg.write_end_list());
	try!(msg.write_end_list());

	try!(msg.write_end_list());

	// end nodes
	try!(msg.write_end_list());

	// end patches
	try!(msg.write_end_list());
	Ok(msg)
}


fn try_write_update_patch(num : u64) -> Result<Vec<u8>> {
	let mut msg : Vec<u8> = Vec::new();

	try!(msg.write_msg_type(MessageType::Patch));

	try!(msg.write_patch_type(PatchType::Replace, 151));
	try!(msg.write_text(151, &num.to_string()));

	if num > 25 {
		try!(msg.write_patch_type(PatchType::Remove, 1000 + (1000 * (num-25))));
	}

	let base_id = 1000 + (1000 * num);

	try!(msg.write_patch_type(PatchType::Append, 201));
	try!(msg.write_el(base_id, ElementName::Tr));
	try!(msg.write_end_list());

	for x in 1..20 {
		try!(msg.write_el(base_id + (2*x), ElementName::Td));
		try!(msg.write_end_list());
		try!(msg.write_text(base_id + (2*x)+1, &(num*x).to_string()));
		try!(msg.write_end_list());
	}

	try!(msg.write_end_list());
	try!(msg.write_end_list());

	// end patches
	try!(msg.write_end_list());

	Ok(msg)
}
