#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

mod interface;
mod from;
mod to;

use crate::from::writer::FromWriter;
use crate::to::writer::ToWriter;

use std::fs;
use serde_xml_rs::from_reader;
use strip_bom::StripBom;

use std::fs::File;
use std::io::BufWriter;

use std::path::PathBuf;
use structopt::StructOpt;

// Define a struct called Opts
#[derive(Debug, StructOpt)]
struct Opts {

    #[structopt(short = "f", long = "from_lang", default_value = "cs")]
    from_lang: String,
    from_output_path: Option<PathBuf>,

    #[structopt(short = "t", long = "to_lang", default_value = "c")]
    to_lang: String,
    to_output_path: Option<PathBuf>,

    #[structopt(short = "i", long = "interface")]
    interface_path: Option<PathBuf>,
}

fn main() {

    let opts: Opts = Opts::from_args();

    // INTERFACE
    let interface_path = match opts.interface_path {
        Some(path) => path,
        None => PathBuf::from(r"sample/interface.xml"),
    };

    println!("Generating FFI to call {} from {} :)", opts.to_lang, opts.from_lang);
    println!("Interface file: {}", interface_path.display());
    let interface_str = fs::read_to_string(interface_path).unwrap();
    let interface_str_no_bom = interface_str.strip_bom().to_string();
    let interface: interface::Interface = from_reader(interface_str_no_bom.as_bytes()).unwrap();


    // FROM
    let mut from_output_writer = match opts.from_lang.to_ascii_lowercase().as_str() {
        "c#" | "cs" | "csharp" => crate::from::csharp::CsharpWriter,
        _ => panic!("From {} is not supported!", opts.from_lang)
    };
    let from_output_path = match opts.from_output_path {
        Some(path) => path,
        None => PathBuf::from(format!(r"sample/output.{}", from_output_writer.file_extension())),
    };
    let from_output_file = File::create(&from_output_path).unwrap();
    let mut from_output_buf_writer = BufWriter::new(from_output_file);

    println!("Output {} file: {}", opts.from_lang, from_output_path.display());
    from_output_writer.write(&mut from_output_buf_writer, &interface);


    // TO
    let mut to_output_writer = match opts.to_lang.to_ascii_lowercase().as_str() {
        "c" => crate::to::c::CWriter,
        _ => panic!("To {} is not supported!", opts.to_lang)
    };
    let to_output_path = match opts.to_output_path {
        Some(path) => path,
        None => PathBuf::from(format!(r"sample/output.{}", to_output_writer.file_extension())),
    };
    let to_output_file = File::create(&to_output_path).unwrap();
    let mut to_output_buf_writer = BufWriter::new(to_output_file);

    println!("Output {} file: {}", opts.to_lang, to_output_path.display());
    to_output_writer.write(&mut to_output_buf_writer, &interface);
}
