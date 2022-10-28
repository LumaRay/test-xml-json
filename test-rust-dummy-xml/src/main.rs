// https://crates.io/crates/dummy_xml
// 26170 DOM

use dummy_xml::parser;

use std::time::{SystemTime};//, UNIX_EPOCH};

use std::fs;

const LOOPS_COUNT: u32 = 1000;

fn main() {
    let buf = fs::read_to_string("/home/test/test-xml-json/data/mondial-3.0-mini.xml").unwrap();

	let start = SystemTime::now();
	for _ in 0..LOOPS_COUNT {
        let result = parser::parse_str(&buf);
        match result {
            Ok(document) => {
                let root = document.root();
                //println!("root is {:?}", root);
            }
            Err(error) => panic!("{:?}", error),
        }
	}
	println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}
