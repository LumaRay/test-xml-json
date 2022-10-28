// https://crates.io/crates/xml-rs
// 70167

use std::time::{SystemTime};//, UNIX_EPOCH};

//extern crate xml;

use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};
use xml::reader::Result;
use xml::attribute::OwnedAttribute;

const LOOPS_COUNT: u32 = 1000;

fn main() {

	let mut count = 0;
	//let mut buf = Vec::new();

	let start = SystemTime::now();
	for _ in 0..LOOPS_COUNT {
        let file = File::open("/home/test/test-xml-json/data/mondial-3.0-mini.xml").unwrap();
        let reader = BufReader::new(file);
        let parser = EventReader::new(reader);
		read(parser);
        //drop(reader);
		//reader.seek(SeekFrom::Start(0));
        //reader = BufReader::new(file);
	}
	println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());

	println!("{:?}", count);
}

fn read(parser: EventReader<BufReader<File>>) {
    let mut depth = 0;
    //let events = parser.into_iter();
    //let evts_readers = events.into_iter();
    let e: Result<XmlEvent>;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                //print!("{}+{}", indent(depth), name);
                for OwnedAttribute {name:name, value:value} in attributes {
                    //print!(" {}:{}", name, value);
                }
                //println!("");
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                //println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                //println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    //evts_readers.into_inner().into_inner()
    //e.unwrap()
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}
