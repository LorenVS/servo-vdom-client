extern crate servo_vdom_client;

use servo_vdom_client::client::{ServoVdomOptions,ServoVdomClient};
use servo_vdom_client::patch::*;
use std::f64::consts;
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
		sleep(Duration::new(0, 100000000));

		num += 5;
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
div.container {
	width:500px;
}

div.bar {
	height:1px;
	background-color:red;
}
	"#));
	try!(msg.write_end_list());

	try!(msg.write_el(200, ElementName::Div));
	try!(msg.write_attr(AttributeRef::Class("container")));
	try!(msg.write_end_list());
	try!(msg.write_end_list());

	for i in 1..250 {
		try!(msg.write_el(200 + i, ElementName::Div));
		try!(msg.write_attr(AttributeRef::Class("bar")));
		try!(msg.write_end_list());
		try!(msg.write_end_list());
	}

	// end nodes
	try!(msg.write_end_list());

	// end patches
	try!(msg.write_end_list());
	Ok(msg)
}


fn try_write_update_patch(num : u64) -> Result<Vec<u8>> {
	let mut msg : Vec<u8> = Vec::new();

	try!(msg.write_msg_type(MessageType::Patch));

	for i in 1..250 {
		let as_f64 : f64 = ((num+i)%250) as f64;
		let sin = ((as_f64 * consts::PI / 125.0).sin() * 250.0) as i32;
		let left = if sin > 0 { 250 - sin } else { 250 };
		let width = sin.abs();
		let widthpx = width.to_string() + "px";
		let leftpx = left.to_string() + "px";

		try!(msg.write_patch_type(PatchType::ModifyAttrs, 200 + i));
		try!(msg.write_attr(AttributeRef::Style("margin-left", &leftpx)));
		try!(msg.write_attr(AttributeRef::Style("width", &widthpx)));
		try!(msg.write_end_list());
	}

	// end patches
	try!(msg.write_end_list());

	Ok(msg)
}
