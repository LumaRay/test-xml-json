// https://crates.io/crates/serde_json
// 18461

use serde_json::{Result, Value};

use std::time::{SystemTime};//, UNIX_EPOCH};

use std::fs::File;
use std::io::Read;

const LOOPS_COUNT: u32 = 1000;

/*enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}*/

fn main() {
    //untyped_example().unwrap();

    let mut f = File::open("/home/test/test-xml-json/data/mondial-3.0-mini.json").unwrap();
    let mut buf: Vec<u8> = vec![];
    f.read_to_end(&mut buf).unwrap();

    let start = SystemTime::now();
	for _ in 0..LOOPS_COUNT {
        let v: Value = serde_json::from_slice(&buf).unwrap();
	}
	println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}

/*fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}*/


