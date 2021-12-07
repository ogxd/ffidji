#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

mod base;
mod interface;
mod from;
mod to;

use crate::interface::*;
use crate::from::*;
use crate::to::*;

mod output;

use serde_xml_rs::from_reader;
use strip_bom::StripBom;
use structopt::StructOpt;

use std::{fmt::Display, fs::{read_to_string, File}};
use std::io::BufWriter;
use std::path::PathBuf;

// Define a struct called Opts
#[derive(Debug, StructOpt)]
struct Opts {

    #[structopt(short = "f", long = "from", default_value = "Interface.cs")]
    from: PathBuf,

    #[structopt(short = "t", long = "to", default_value = "Interface.rs")]
    to: PathBuf,

    #[structopt(short = "i", long = "Interface.xml")]
    interface: PathBuf,
}

fn main() {
    
    let opts: Opts = Opts::from_args();

    // INTERFACE
    println!("Interface file: {}", opts.interface.display());
    let interface_str = read_to_string(opts.interface).unwrap();
    let interface_str_no_bom = interface_str.strip_bom().to_string();
    let mut interface: Interface = from_reader(interface_str_no_bom.as_bytes()).unwrap();
    interface.initialize();
    interface.check_valid();

    // FROM
    let from_extension = opts.from.extension().unwrap().to_str().unwrap();
    let mut from_output_writer: Box<dyn base::Writer> = match from_extension {
        "cs" => Box::new(CsharpWriter),
        _ => panic!("From {} is not supported!", from_extension)
    };
    interface.check_reserved(&*from_output_writer);
    let from_output_file = File::create(&opts.from).unwrap();
    let mut from_output_buf_writer = BufWriter::new(from_output_file);

    println!("Output 'from' file: {}", opts.from.display());
    from_output_writer.write(&mut from_output_buf_writer, &interface);

    // TO
    let to_extension = opts.to.extension().unwrap().to_str().unwrap();
    let mut to_output_writer: Box<dyn base::Writer> = match to_extension {
        "c" => Box::new(CWriter),
        "rs" => Box::new(RustWriter),
        _ => panic!("To {} is not supported!", opts.to.display())
    };
    interface.check_reserved(&*to_output_writer);
    let to_output_file = File::create(&opts.to).unwrap();
    let mut to_output_buf_writer = BufWriter::new(to_output_file);

    println!("Output 'to' file: {}", opts.to.display());
    to_output_writer.write(&mut to_output_buf_writer, &interface);
}