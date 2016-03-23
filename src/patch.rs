use std::io::{Error,ErrorKind,Read,Result,Write};
use std::mem;
use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt};

/// The possible HTML element names.
#[repr(u8)]
pub enum ElementName {
	Anchor = 0,
	Applet = 1,
	Area,
	Audio,
	Base,
	Body,
	Br,
	Button,
	Canvas,
	Data,
	DataList,
	Details,
	Dialog,
	Directory,
	Div,
	DList,
	Embed,
	FieldSet,
	Font,
	Form,
	Frame,
	FrameSet,
	Head,
	Heading,
	Hr,
	Html,
	Image,
	Input,
	Label,
	Legend,
	Li,
	Link,
	Map,
	Media,
	Meta,
	Meter,
	Mod,
	Object,
	OList,
	OptGroup,
	Option,
	Output,
	Paragraph,
	Param,
	Pre,
	Progress,
	Quote,
	Select,
	Source,
	Span,
	Style,
	TableCaption,
	TableCol,
	TableDataCell,
	Table,
	TableHeaderCell,
	TableRow,
	TableSection,
	Template,
	Textarea,
	Time,
	Title,
	Track,
	UList,
	Unknown,
	Video
}

/// The possible HTML attributes.
pub enum AttributeRef<'a> {
	Class(&'a str)
}

/// The possible HTML attributes.
pub enum AttributeVal {
	Class(String)
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
	/// Writes a dom string to the writer.
	fn write_dom_str(&mut self, val : &str) -> Result<()> {
		let bytes = val.as_bytes();
		try!(self.write_u32::<BE>(bytes.len() as u32));
		self.write_all(bytes)
	}

	/// Writes an element header to the writer.
	fn write_el(&mut self, id: u64, name: ElementName) -> Result<()> {
		let name_u8 = unsafe { mem::transmute(name) };
		try!(self.write_u8(0).map_err(|_| io_error()));
		try!(self.write_u64::<BE>(id));
		self.write_u8(name_u8).map_err(|_| io_error())
	}

	/// Writes an attribute to the writer.
	fn write_attr(&mut self, attr: &AttributeRef) -> Result<()> {
		match *attr {
			AttributeRef::Class(val) => {
				try!(self.write_u8(0).map_err(|_| io_error()));
				self.write_dom_str(val)
			}
		}
	}

	/// Writes the end of a attribute list to the writer.
	fn write_end_attrs(&mut self) -> Result<()> {
		self.write_u8(255).map_err(|_| io_error())
	}

	/// Writes a text node to the writer.
	fn write_text(&mut self, id: u64, val: &String) -> Result<()> {
		try!(self.write_u64::<BE>(id));
		self.write_dom_str(val)
	}

	/// Writes the end of a node list to the writer.
	fn write_end_nodes(&mut self) -> Result<()> {
		self.write_u8(255).map_err(|_| io_error())
	}
}

pub trait ReadPatchExt: Read {
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
			match discriminant {
				0 => Ok(Some(NodeType::Element)),
				1 => Ok(Some(NodeType::Text)),
				_ => {
					Err(Error::new(ErrorKind::InvalidData, "invalid node discriminant"))
				}
			}
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
}