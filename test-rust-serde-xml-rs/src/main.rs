// https://crates.io/crates/serde-xml-rs
// 

use std::time::{SystemTime};//, UNIX_EPOCH};

use std::fs;

//use std::iter::Map;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

const LOOPS_COUNT: u32 = 1000;

#[derive(Serialize, Deserialize)]
enum Value {
    Null,
    Bool(bool),
    Number(f32),
    #[serde(rename = "$value")]
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[derive(Serialize, Deserialize)]
struct Data {
    #[serde(rename = "continent")]
    mondial: HashMap<String, Value>,
}

fn main() {
	//let mut buf = Vec::new();
    let buf = fs::read_to_string("/home/test/test-xml-json/data/mondial-3.0-mini.xml").unwrap();//.parse().unwrap();

	let start = SystemTime::now();
	for _ in 0..LOOPS_COUNT {
        println!("{}", buf);
        let data: Data = from_str(&buf).unwrap();
        //assert_eq!(document.content.value, "Lorem ipsum");
	}
	println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}
// /mondial/country/province/city/name


