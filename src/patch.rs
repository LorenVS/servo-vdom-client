use std::io::{Error,ErrorKind,Read,Result,Write};
use std::mem;
use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt};

/// The possible message types.
#[repr(u8)]
pub enum MessageType {
	Patch = 0,
	Event = 1,
	EventAck = 2
}

/// The possible patch types.
#[repr(u8)]
pub enum PatchType {
	Replace = 0,
	ModifyAttrs = 1,
	Remove = 2,
	Append = 3,
	AppendMultiple = 4
}

/// The possible HTML element names.
#[repr(u8)]
pub enum ElementName {
	A,
	Acronym,
	Address,
	Applet,
	Area,
	Article,
	Aside,
	Audio,
	B,
	Base,
	Bdi,
	Bdo,
	Bgsound,
	Big,
	Blink,
	Blockquote,
	Body,
	Br,
	Button,
	Canvas,
	Caption,
	Center,
	Cite,
	Code,
	Col,
	Colgroup,
	Data,
	Datalist,
	Dd,
	Del,
	Details,
	Dfn,
	Dialog,
	Dir,
	Div,
	Dl,
	Dt,
	Em,
	Embed,
	Fieldset,
	Figcaption,
	Figure,
	Font,
	Footer,
	Form,
	Frame,
	Frameset,
	H1,
	H2,
	H3,
	H4,
	H5,
	H6,
	Head,
	Header,
	Hgroup,
	Hr,
	Html,
	I,
	Img,
	Input,
	Ins,
	Isindex,
	Kbd,
	Label,
	Legend,
	Li,
	Link,
	Listing,
	Main,
	Map,
	Mark,
	Marquee,
	Meta,
	Meter,
	Multicol,
	Nav,
	Nextid,
	Nobr,
	Noframes,
	Noscript,
	Object,
	Ol,
	Optgroup,
	Option,
	Output,
	P,
	Param,
	Plaintext,
	Pre,
	Progress,
	Q,
	Rp,
	Rt,
	Ruby,
	S,
	Samp,
	Section,
	Select,
	Small,
	Source,
	Spacer,
	Span,
	Strike,
	Strong,
	Style,
	Sub,
	Summary,
	Sup,
	Table,
	Tbody,
	Td,
	Template,
	Textarea,
	Tfoot,
	Th,
	Thead,
	Time,
	Title,
	Tr,
	Tt,
	Track,
	U,
	Ul,
	Var,
	Video,
	Wbr,
	Xmp,
}

pub enum EventType {
	Blur,
	Resize,
	Change,
	Click,
	DoubleClick,
	Input,
	KeyDown,
	KeyPress,
	KeyUp,
	MouseMove,
	MouseOut,
	MouseOver,
	Load
}

/// The possible event synchronization values. If an event
/// is synchronized, servo will wait for the connected application
/// to respond to the event with an EventAck message before
/// proceeding.
pub enum EventSubscription {
	None,
	NotSynchronized,
	Synchronized
}

/// The possible HTML attributes.
pub enum AttributeRef<'a> {
	Class(&'a str),
	Style(&'a str, &'a str),
	Event(EventType, EventSubscription)
}

/// The possible HTML attributes.
pub enum AttributeVal {
	Class(String),
	Style(String, String),
	Event(EventType, EventSubscription)
}

/// The possible node types.
pub enum NodeType {
	Element,
	Text
}

type BE = BigEndian;

fn io_error() -> Error {
	Error::new(ErrorKind::InvalidData, "weird u8 issues")
}

pub trait WritePatchExt: Write {

	fn write_msg_type(&mut self, ty: MessageType) -> Result<()> {
		let ty_u8 = unsafe { mem::transmute(ty) };
		self.write_u8(ty_u8).map_err(|_| io_error())
	}

	fn write_patch_type(&mut self, ty: PatchType, id : u64) -> Result<()> {
		let ty_u8 = unsafe { mem::transmute(ty) };
		try!(self.write_u8(ty_u8).map_err(|_| io_error()));
		self.write_u64::<BE>(id).map_err(|_| io_error())
	}

	fn write_event_type(&mut self, ty: EventType) -> Result<()> {
		let ty_u8 = unsafe { mem::transmute(ty) };
		self.write_u8(ty_u8).map_err(|_| io_error())
	}

	fn write_event_sub(&mut self, sub: EventSubscription) -> Result<()> {
		let sub_u8 = unsafe { mem::transmute(sub) };
		self.write_u8(sub_u8).map_err(|_| io_error())
	}

	fn write_node_type(&mut self, ty: NodeType) -> Result<()> {
		let ty_u8 = unsafe { mem::transmute(ty) };
		self.write_u8(ty_u8).map_err(|_| io_error())
	}

	/// Writes a dom string to the writer.
	fn write_dom_str(&mut self, val : &str) -> Result<()> {
		let bytes = val.as_bytes();
		try!(self.write_u32::<BE>(bytes.len() as u32));
		self.write_all(bytes)
	}

	/// Writes an element header to the writer.
	fn write_el(&mut self, id: u64, name: ElementName) -> Result<()> {
		try!(self.write_node_type(NodeType::Element));
		let name_u8 = unsafe { mem::transmute(name) };
		try!(self.write_u64::<BE>(id));
		self.write_u8(name_u8).map_err(|_| io_error())
	}

	/// Writes an attribute to the writer.
	fn write_attr(&mut self, attr: AttributeRef) -> Result<()> {
		match attr {
			AttributeRef::Class(val) => {
				try!(self.write_u8(0).map_err(|_| io_error()));
				self.write_dom_str(val)
			},
			AttributeRef::Style(key, value) => {
				try!(self.write_u8(1).map_err(|_| io_error()));
				try!(self.write_dom_str(key));
				self.write_dom_str(value)
			},
			AttributeRef::Event(ty, sub) => {
				try!(self.write_u8(2).map_err(|_| io_error()));
				try!(self.write_event_type(ty));
				self.write_event_sub(sub)
			}
		}
	}

	/// Writes a text node to the writer.
	fn write_text(&mut self, id: u64, val: &str) -> Result<()> {
		try!(self.write_node_type(NodeType::Text));
		try!(self.write_u64::<BE>(id));
		self.write_dom_str(val)
	}

	/// Writes the end of a patch, node or attr list.
	fn write_end_list(&mut self) -> Result<()> {
		self.write_u8(255).map_err(|_| io_error())
	}
}

impl <T:Write> WritePatchExt for T {}

pub trait ReadPatchExt: Read {

	/// Reads a message type from the reader
	fn read_msg_type(&mut self) -> Result<MessageType> {
		let ty_u8 = try!(self.read_u8().map_err(|_| io_error()));
		let ty = unsafe { mem::transmute(ty_u8) };
		Ok(ty)
	}

	/// Reads a patch type from the reader.
	fn read_patch_type(&mut self) -> Result<Option<(PatchType, u64)>> {
		let discriminant = try!(self.read_u8());
		if discriminant == 255 {
			Ok(None)
		} else {
			let patch_ty = unsafe { mem::transmute(discriminant) };
			let id = try!(self.read_u64::<BE>());
			Ok(Some((patch_ty, id)))
		}
	}

	/// Reads a dom string from the reader.
	fn read_dom_str(&mut self) -> Result<String> {
		let len = try!(self.read_u32::<BE>()) as usize;
		let mut buf = Vec::with_capacity(len);
		unsafe { buf.set_len(len); }
		try!(self.read_exact(&mut buf[0..]));
		let res = String::from_utf8(buf);
		if res.is_err() {
			Err(Error::new(ErrorKind::InvalidData, "invalid utf8 data"))
		} else {
			Ok(res.unwrap())
		}
	}

	/// Reads a node type from the reader, returning None
	/// if the end of a node list was read.
	fn read_node_type(&mut self) -> Result<Option<NodeType>> {
		let discriminant = try!(self.read_u8());
		if discriminant == 255 {
			Ok(None)
		} else {
			Ok(Some(unsafe { mem::transmute(discriminant) }))
		}
	}

	/// Reads an element id and name from the reader.
	fn read_el(&mut self) -> Result<(u64,ElementName)> {
		let id = try!(self.read_u64::<BE>());
		let name_u8 = try!(self.read_u8());
		let name = unsafe { mem::transmute(name_u8) };
		Ok((id, name))
	}

	/// Reads an attribute from the reader, returning None if the
	/// end of an attribute list was consumed.
	fn read_attr(&mut self) -> Result<Option<AttributeVal>> {
		let discriminant = try!(self.read_u8());
		if discriminant == 255 {
			Ok(None)
		} else {
			match discriminant {
				0 => {
					let class = try!(self.read_dom_str());
					Ok(Some(AttributeVal::Class(class)))
				},
				1 => {
					let key = try!(self.read_dom_str());
					let val = try!(self.read_dom_str());
					Ok(Some(AttributeVal::Style(key, val)))
				},
				2 => {
					let ty = try!(self.read_event_type());
					let sub = try!(self.read_event_sub());
					Ok(Some(AttributeVal::Event(ty, sub)))
				},
				_ => {
					Err(Error::new(ErrorKind::InvalidData, "invalid attribute discriminant"))
				}
			}
		}
	}

	/// Reads a text node from the reader.
	fn read_text(&mut self) -> Result<(u64,String)> {
		let id = try!(self.read_u64::<BE>());
		let text = try!(self.read_dom_str());
		Ok((id, text))
	}

	/// Reads an event type from the reader.
	fn read_event_type(&mut self) -> Result<EventType> {
		let ty_u8 = try!(self.read_u8());
		Ok(unsafe { mem::transmute(ty_u8) })
	}

	/// Reads an event subscription from the reader.
	fn read_event_sub(&mut self) -> Result<EventSubscription> {
		let sub_u8 = try!(self.read_u8());
		Ok(unsafe { mem::transmute(sub_u8) })
	}

	/// Reads a node id list from the reader.
	fn read_node_id_list(&mut self) -> Result<Vec<u64>> {
		let len = try!(self.read_u32::<BE>()) as usize;
		let mut vec = Vec::with_capacity(len);
		for _ in 0..len {
			let id = try!(self.read_u64::<BE>());
			vec.push(id);
		}
		Ok(vec)
	}

	/// Reads an event from the reader.
	fn read_event(&mut self) -> Result<(EventType, EventSubscription, Vec<u64>)> {
		let ty = try!(self.read_event_type());
		let sub = try!(self.read_event_sub());
		let ids = try!(self.read_node_id_list());
		Ok((ty, sub, ids))
	}
}

impl <T:Read> ReadPatchExt for T {}