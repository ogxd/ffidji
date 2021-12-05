use crate::interface::{Interface as Interface};

use std::fs::File;
use std::io::BufWriter;

pub trait Writer {
    fn file_extension(&self) -> String;
    fn write(&mut self, writer: &mut BufWriter<File>, interface: &Interface);
    fn is_name_reserved(&self, name: &String) -> bool;
}
