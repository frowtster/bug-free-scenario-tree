use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn main() {
	let _ = config();
}

fn config() -> std::io::Result<()> {
	println!("Read Scenario.");
	let file = File::open("scenario/first.xml")?;
    let file = BufReader::new(file);
	let parser = EventReader::new(file);
	
	for event in parser {
		println!("{:?}", event.unwrap());
	}

	Ok(())
}