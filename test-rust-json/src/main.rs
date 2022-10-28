// https://crates.io/crates/json
// 7692

use json;

use std::time::{SystemTime};//, UNIX_EPOCH};

use std::fs;

const LOOPS_COUNT: u32 = 1000;

fn main() {
    let buf = fs::read_to_string("/home/test/test-xml-json/data/mondial-3.0-mini.json").unwrap();

    let start = SystemTime::now();
	for _ in 0..LOOPS_COUNT {
        let parsed = json::parse(&buf).unwrap();
	}
	println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}

