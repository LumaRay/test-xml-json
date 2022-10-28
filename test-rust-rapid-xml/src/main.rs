// https://crates.io/crates/rapid-xml
// 3946

use std::time::{SystemTime};//, UNIX_EPOCH};

//extern crate xml;

use std::fs::File;
//use std::io::Seek;
//use std::io::SeekFrom;
use std::io::BufReader;

use rapid_xml::parser::*;

const LOOPS_COUNT: u32 = 1000;

fn main() {

	//let mut count = 0;
	//let mut buf = Vec::new();

	let start = SystemTime::now();
	for _ in 0..LOOPS_COUNT {
        let file = File::open("/home/test/test-xml-json/data/mondial-3.0-mini.xml").unwrap();
        let reader = BufReader::new(file);
        let parser = Parser::new(reader);
		read(parser);
        //drop(reader);
		//reader.seek(SeekFrom::Start(0));
        //reader = BufReader::new(file);
	}
	println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());

	//println!("{:?}", count);
}

fn read(mut parser: Parser<BufReader<File>>) {
    let mut depth = 0;
    //let events = parser.into_iter();
    //let evts_readers = events.into_iter();
    //let e: Result<XmlEvent>;
    while let Ok(mut e) = parser.next() {
        //println!("{}", val);
        match &mut e.code() {
            EventCode::StartTag => {
                let val = e.get_str().unwrap();
                //println!("{}+{}", indent(depth), val);
                depth += 1;
            }
            EventCode::AttributeName => {
                let val = e.get_str().unwrap();
                //print!(" {}:", val);
            }
            EventCode::AttributeValue => {
                let val = e.get_str().unwrap();
                //print!("{}", val);
            }
            EventCode::EndTag => {
                let val = e.get_str().unwrap();
                depth -= 1;
                //println!("\n{}-{}", indent(depth), val);
            }
            EventCode::EndTagImmediate => {
                let val = e.get_str().unwrap();
                depth -= 1;
                //println!("{}+-{}", indent(depth), val);
            }
            EventCode::Eof => {
                break;
            }            
            //Err(e) => {
            //    //println!("Error: {}", e);
            //    break;
            //}
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
