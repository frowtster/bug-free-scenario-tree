use std::{fs::File};
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
	
	let mut depth = 0;

	for event in parser {
		match event {
			Ok(XmlEvent::StartElement { name, .. }) => {
				println!("{:spaces$}+{name}", "", spaces = depth * 2);
				depth += 1;
			}
			Ok(XmlEvent::EndElement { name }) => {
				depth -= 1;
				println!("{:spaces$}-{name}", "", spaces = depth * 2);
			}
			Err(event) => {
				eprintln!("Error: {event}");
                break;
			}
			_ => {}
		}
	}

	Ok(())
}