#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

mod interface;
mod writers;

use crate::writers::csharp_writer::InterfaceWriter as InterfaceWriter;

use std::env;
use std::fs;
use serde_xml_rs::from_reader;
use strip_bom::StripBom;

use std::fs::File;
use std::io::BufWriter;

mod foo {
    fn f() { println!("hello world"); }
    pub macro m() { f(); }
}
fn main() { foo::m!(); }

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

    let f = File::create(r"sample\output.txt").expect("Unable to create file");
    let mut buf_writer = BufWriter::new(f);

    let writer = writers::csharp_writer::CsharpWriter{};
    writer.write(&interface, &mut buf_writer);
}