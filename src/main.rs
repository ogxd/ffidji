#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

mod interface;

use std::env;
use std::fs;
use serde_xml_rs::from_reader;
use strip_bom::StripBom;

fn main() {

    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let filename = r"sample\interface.xml";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let contents_no_bom = contents.strip_bom().to_string();
    
    let interface: interface::Interface = from_reader(contents_no_bom.as_bytes()).unwrap();
    println!("{:#?}", interface);
}