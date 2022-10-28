// https://crates.io/crates/simd-json
// 14503

use std::time::{SystemTime};//, UNIX_EPOCH};

use simd_json;

use std::fs;
use std::io::prelude::*;
use std::fs::File;

const LOOPS_COUNT: u32 = 1000;

fn main() {
    let mut f = File::open("/home/test/test-xml-json/data/mondial-3.0-mini.json").unwrap();
    let mut buf: Vec<u8> = vec![];
    f.read_to_end(&mut buf).unwrap();

    let start = SystemTime::now();
	for _ in 0..LOOPS_COUNT {
        //let mut d = br#"{"some": ["key", "value", 2]}"#.to_vec();
        let v: simd_json::OwnedValue = simd_json::to_owned_value(&mut buf).unwrap();
	}
	println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}
