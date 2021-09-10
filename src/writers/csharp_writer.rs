use crate::interface::Interface as Interface;

use std::fs::File;
use std::io::{BufWriter,Write};

// macro_rules! calculate {
//     ($($args:expr),*) => {{
//         $(
//             writer.write_all(format!("public function {}", $args).as_bytes()).unwrap();
//         )*
//     }}
// }

pub trait InterfaceWriter {
    fn write(&self, _interface: &Interface, writer: &BufWriter<File>);
}

pub struct CsharpWriter<'a> {
    writer: &'a BufWriter<File>,
}

impl<'a> InterfaceWriter for CsharpWriter<'a> {
    fn write<'b>(&self, interface: &Interface, writer: &'b BufWriter<File>) {
        self.writer = writer;
        for method in &interface.methods {
            //calculate!("d");
            writer.write_all("public function {method.name}".as_bytes()).unwrap();
        }
    }
}