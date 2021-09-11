#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

mod interface;
mod writers;

use crate::writers::csharp_writer::InterfaceWriter as InterfaceWriter;

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

    #[structopt(short = "t", long = "to_lang", default_value = "c")]
    to_lang: String,

    #[structopt(short = "i", long = "interface")]
    interface_path: Option<PathBuf>,

    #[structopt(short = "o", long = "from_output")]
    from_output_path: Option<PathBuf>,

    #[structopt(short = "x", long = "to_output")]
    to_output_path: Option<PathBuf>,
}

fn main() {

    let opts: Opts = Opts::from_args();

    let interface_path = match opts.interface_path {
        Some(path) => path,
        None => PathBuf::from(r"sample/interface.xml"),
    };

    let from_output_path = match opts.from_output_path {
        Some(path) => path,
        None => PathBuf::from(format!(r"sample/output.{}", opts.from_lang)),
    };

    let to_output_path = match opts.to_output_path {
        Some(path) => path,
        None => PathBuf::from(format!(r"sample/output.{}", opts.to_lang)),
    };

    println!("Generating FFI to call {} from {} :)", opts.to_lang, opts.from_lang);
    println!("Interface file: {}", interface_path.display());
    println!("Output {} file: {}", opts.from_lang, from_output_path.display());
    println!("Output {} file: {}", opts.to_lang, to_output_path.display());

    let interface_str = fs::read_to_string(interface_path).unwrap();
    let interface_str_no_bom = interface_str.strip_bom().to_string();
    let interface: interface::Interface = from_reader(interface_str_no_bom.as_bytes()).unwrap();

    let output_file = File::create(from_output_path).unwrap();
    let output_buf_writer = BufWriter::new(output_file);

    let mut output_writer = match opts.from_lang.to_ascii_lowercase().as_str() {
        "c#" | "cs" | "csharp" => writers::csharp_writer::CsharpWriter::new(output_buf_writer),
        _ => panic!("From {} is not supported!", opts.from_lang)
    };

    output_writer.write(&interface);

    // Todo: Write to_lang
}
