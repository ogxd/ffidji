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

use std::fs::{read_to_string, File};
use std::io::BufWriter;
use std::path::PathBuf;

// Define a struct called Opts
#[derive(Debug, StructOpt)]
pub struct Opts {

    #[structopt(short = "f", long = "from_lang", default_value = "csharp")]
    pub from_lang: String,
    pub from_output_path: Option<PathBuf>,

    #[structopt(short = "t", long = "to_lang", default_value = "rust")]
    pub to_lang: String,
    pub to_output_path: Option<PathBuf>,

    #[structopt(short = "i", long = "interface")]
    pub interface_path: Option<PathBuf>,
}

pub fn execute(opts: &Opts) {
    
    // INTERFACE
    let interface_path = match opts.interface_path.clone() {
        Some(path) => path,
        None => PathBuf::from(r"sample/interface.xml"),
    };

    println!("Generating FFI to call {} from {} :)", opts.to_lang, opts.from_lang);
    println!("Interface file: {}", interface_path.display());
    let interface_str = read_to_string(interface_path).unwrap();
    let interface_str_no_bom = interface_str.strip_bom().to_string();
    let mut interface: Interface = from_reader(interface_str_no_bom.as_bytes()).unwrap();
    interface.initialize();
    interface.check_valid();

    // FROM
    let mut from_output_writer: Box<dyn base::Writer> = match opts.from_lang.to_ascii_lowercase().as_str() {
        "c#" | "cs" | "csharp" => Box::new(CsharpWriter),
        _ => panic!("From {} is not supported!", opts.from_lang)
    };
    interface.check_reserved(&*from_output_writer);
    let from_output_path = match opts.from_output_path.clone() {
        Some(path) => path,
        None => PathBuf::from(format!(r"sample/from/{}/output.{}", opts.from_lang, from_output_writer.file_extension())),
    };
    let from_output_file = File::create(&from_output_path).unwrap();
    let mut from_output_buf_writer = BufWriter::new(from_output_file);

    println!("Output {} file: {}", opts.from_lang, from_output_path.display());
    from_output_writer.write(&mut from_output_buf_writer, &interface);

    // TO
    let mut to_output_writer: Box<dyn base::Writer> = match opts.to_lang.to_ascii_lowercase().as_str() {
        "c" => Box::new(CWriter),
        "rust" | "rs" => Box::new(RustWriter),
        _ => panic!("To {} is not supported!", opts.to_lang)
    };
    interface.check_reserved(&*to_output_writer);
    let to_output_path = match opts.to_output_path.clone() {
        Some(path) => path,
        None => PathBuf::from(format!(r"sample/to/{}/output.{}", opts.to_lang, to_output_writer.file_extension())),
    };
    let to_output_file = File::create(&to_output_path).unwrap();
    let mut to_output_buf_writer = BufWriter::new(to_output_file);

    println!("Output {} file: {}", opts.to_lang, to_output_path.display());
    to_output_writer.write(&mut to_output_buf_writer, &interface);
}
