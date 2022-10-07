// https://crates.io/crates/quick-xml
// 39417

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::{SystemTime};//, UNIX_EPOCH};
use std::io::Seek;
use std::io::SeekFrom;

const LOOPS_COUNT: u32 = 16 * 1024;

fn read(reader: &mut Reader<BufReader<File>>, count: &mut i32, buf: &mut Vec<u8>) {
	// The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
	loop {
	    // NOTE: this is the generic case when we don't know about the input BufRead.
	    // when the input is a &str or a &[u8], we don't actually need to use another
	    // buffer, we could directly call `reader.read_event()`
	    match reader.read_event_into(buf) {
		Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
		// exits the loop when reaching end of file
		Ok(Event::Eof) => break,

		Ok(Event::Empty(e)) => {
		    //println!("empty {:?}", e.name());//.as_ref());
		    //println!("empty attrs: {:?}", e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>());
		    *count += 1;
		},
		Ok(Event::Start(e)) => {
		    //println!("{:?}", e.name());//.as_ref());
		    //println!("attrs: {:?}", e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>());
		    *count += 1;
		}
		Ok(Event::Text(e)) => {
		    //println!("{:?}", e);//.as_ref());
		    *count +=1 ;
		},

		// There are several other `Event`s we do not consider here
		_ => (),
	    }
	    // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
	    buf.clear();
	}
}

fn main() {
	let mut reader = Reader::from_file("/home/test/test-xml-json/test-rust-quick-xml/data/mondial-3.0-mini.xml").unwrap();
	reader.trim_text(true);

	let mut count = 0;
	//let mut txt = Vec::new();
	let mut buf = Vec::new();

	let start = SystemTime::now();
	for _ in 0..LOOPS_COUNT {
		read(&mut reader, &mut count, &mut buf);
		let mut r = reader.into_inner();
		r.seek(SeekFrom::Start(0));
		reader = Reader::from_reader(r);
	}
	println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());

	println!("{:?}", count);
}
