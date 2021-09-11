use crate::interface::Interface as Interface;

use std::fs::File;
use std::io::{BufWriter, Write};

pub trait InterfaceWriter {
    fn new(writer: BufWriter<File>) -> Self;
    fn write(&mut self, _interface: &Interface);
}

pub struct CsharpWriter {
    writer: BufWriter<File>,
}

impl InterfaceWriter for CsharpWriter {
    
    fn new(_writer: BufWriter<File>) -> Self {
        CsharpWriter {
            writer: _writer,
        }
    }

    fn write(&mut self, interface: &Interface) {

        let mut indentation = 0;

        macro_rules! write {
            () => {{
                self.writer.write("\n".as_bytes()).unwrap();
            }};
            ("{") => {{
                write!("{ ");
                indentation = indentation + 1;
            }};
            ("}") => {{
                indentation = indentation - 1;
                write!("} ");
            }};
            ($text:expr) => {{
                self.writer.write(" ".repeat(4 * indentation).as_bytes()).unwrap();
                self.writer.write($text.as_bytes()).unwrap();
                write!();
            }};
            ($text:expr, $($args:expr),*) => {{
                self.writer.write(" ".repeat(4 * indentation).as_bytes()).unwrap();
                self.writer.write(format!($text, $($args), *).as_bytes()).unwrap();
                write!();
            }}
        }

        write!("using System;");
        write!("using System.Runtime.InteropServices;");
        write!();
        write!("namespace MyInterface");
        write!("{");

        for method in &interface.methods {
            write!("[DllImport({})]", "MyNativeLibrary.dll");
            write!("public {} {}", method.name, "prout");
            write!("{");
            write!("}");
        }

        write!("}");
    }
}