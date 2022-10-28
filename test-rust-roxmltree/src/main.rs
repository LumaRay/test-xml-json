// https://crates.io/crates/roxmltree
// 10040 DOM

use std::time::{SystemTime};//, UNIX_EPOCH};

//use roxmltree::*;

use std::fs;

const LOOPS_COUNT: u32 = 1000;

fn main() {
	//let mut buf = Vec::new();
    let buf = fs::read_to_string("/home/test/test-xml-json/data/mondial-3.0-mini.xml").unwrap();//.parse().unwrap();

	let start = SystemTime::now();
	for _ in 0..LOOPS_COUNT {
        let doc = roxmltree::Document::parse(&buf).unwrap();
        //let elem = doc.descendants().find(|n| n.attribute("id") == Some("f0_24217")).unwrap();
        //println!("{:?}", elem);
        //assert!(elem.has_tag_name("rect"));
	}
	println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}
