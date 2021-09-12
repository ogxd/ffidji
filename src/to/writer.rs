use crate::interface::{Interface as Interface};

use std::fs::File;
use std::io::BufWriter;

pub trait ToWriter {
    fn file_extension(&self) -> String;
    fn write(&mut self, writer: &mut BufWriter<File>, interface: &Interface);
}
