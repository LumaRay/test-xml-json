// https://crates.io/crates/xml_oxide
// 43509

use std::time::{SystemTime};//, UNIX_EPOCH};
use std::io::{Seek, SeekFrom};
use std::fs::File;
use xml_oxide::sax::{parser::Parser, Event, Attribute};

const LOOPS_COUNT: u32 = 1000;

fn main() {
    //println!("Starting...");

    //let now = std::time::Instant::now();

	let start = SystemTime::now();
	for _ in 0..LOOPS_COUNT {
        let mut f = File::open("/home/test/test-xml-json/data/mondial-3.0-mini.xml").unwrap();
        //f.seek(SeekFrom::Start(0));
        let mut p = Parser::from_reader(f);
        read(&mut p);
    }
    println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());

    //println!("Start event count:{}", counter);
    //println!("End event count:{}", end_counter);

    //let elapsed = now.elapsed();
    //println!("Time elapsed: {:.2?}", elapsed);
}

fn read(p :&mut Parser<File>) {
    let mut counter: usize = 0;
    let mut end_counter: usize = 0;

    loop {
        let res = p.read_event();

        match res {
            Ok(event) => match event {
                Event::StartDocument => {}
                Event::EndDocument => {
                    break;
                }
                Event::StartElement(el) => {
                    //You can differantiate between Starting Tag and Empty Element Tag
                    if !el.is_empty {
                        counter = counter + 1;
                        // print every 10000th element name
                        if counter % 10000 == 0 {
                            //println!("%10000 start {}", el.name);
                        }
                        for Attribute {name, value, ..} in el.attributes() {
                            //println!("{}:{}", name, value);                 
                        }
                    }
                }
                Event::EndElement(el) => {
                    end_counter += 1;
                    if el.name == "feed" {
                        break;
                    }
                }
                Event::Characters(_) => {}
                Event::Reference(_) => {}
                _ => {}
            },
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
    }
}
