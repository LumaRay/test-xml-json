// https://crates.io/crates/vtd_xml
// 7519 DOM

use std::time::{SystemTime};//, UNIX_EPOCH};
use std::path::Path;

use vtd_xml::VtdGen;

const LOOPS_COUNT: u32 = 1000;

fn main() {
    let mut vtg = VtdGen::new();

    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        vtg.parse_file(true, "/home/test/test-xml-json/data/mondial-3.0-mini.xml").unwrap();
    }
    println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}
