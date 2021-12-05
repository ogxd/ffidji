use crate::interface::Interface;
use crate::base::Writer;

use std::fs::File;
use std::io::{BufWriter, Write};

pub struct RustWriter;

impl Writer for RustWriter {
    
    fn file_extension(&self) -> String { "rs".to_string() }

    fn write(&mut self, writer: &mut BufWriter<File>, interface: &Interface) {

        let mut indentation = 0;

        macro_rules! write {
            () => {{
                writer.write("\n".as_bytes()).unwrap();
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
                writer.write(" ".repeat(4 * indentation).as_bytes()).unwrap();
                writer.write($text.as_bytes()).unwrap();
                write!();
            }};
            ($text:expr, $($args:expr),*) => {{
                writer.write(" ".repeat(4 * indentation).as_bytes()).unwrap();
                writer.write(format!($text, $($args), *).as_bytes()).unwrap();
                write!();
            }}
        }

        macro_rules! param_name {
            ($param:expr) => {{
                let return_type = $param;
                let mut return_type_name = return_type.r#type.clone();
                if return_type.array.unwrap_or(false) {
                    return_type_name = return_type_name + "[]"; // This is not supported... Needs a change on Array types
                } else if !interface.is_param_blittable(&return_type) {
                    return_type_name = return_type_name;
                }
                return_type_name
            }}
        }

        write!("// Autogenerated by FFIDJI");
        write!();

        write!("extern crate libc;");

        write!("// To get strings, use CString. Example:");
        write!("// let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };");
        write!("use std::ffi::CStr; ");

        write!();
        write!("use std::alloc::GlobalAlloc;");
        write!("use std::alloc::System;");
        write!("use std::alloc::Layout;");

        write!();
        write!("use u16 as char16;");
        write!("use i8 as int8;");
        write!("use u8 as uint8;");
        write!("use i16 as int16;");
        write!("use u16 as uint16;");
        write!("use i32 as int32;");
        write!("use u32 as uint32;");
        write!("use i64 as int64;");
        write!("use u64 as uint64;");
        write!("use f32 as float32;");
        write!("use f64 as float64;");

        write!();
        write!("#[no_mangle]");
        write!("pub extern \"C\" fn Alloc_FFI(length :int32) -> *mut libc::c_void");
        write!("{");
        write!("unsafe");
        write!("{");
        write!("return libc::malloc(length as usize);");
        write!("}"); 
        write!("}");

        write!();
        write!("#[no_mangle]");
        write!("pub extern \"C\" fn Free_FFI(ptr: *mut libc::c_void, length :int32)");
        write!("{");
        write!("unsafe");
        write!("{");
        write!("libc::free(ptr);");
        write!("}");
        write!("}");

        for r#type in &interface.types {
            if r#type.base_type {
                continue;
            }

            // FFI type and conversions (if not blittable)
            write!();
            write!("struct {}", r#type.name);
            write!("{");
            for field in &r#type.fields {
                if field.array.unwrap_or(false) {
                    write!("{}_ptr: *const {},", field.name, field.r#type);
                    write!("{}_len: int32,", field.name);
                } else {
                    write!("{}: {},", field.name, field.r#type);
                }
            }
            write!("}");
        }

        for method in &interface.methods {
            let parameters = &method.parameters;
            let parameters_str = parameters
                .into_iter()
                .map(|p| [p.name.clone(), p.r#type.clone()].join(": "))
                .collect::<Vec<String>>()
                .join(", ");

            let mut return_type_name = String::from("void");
            if method.returns.len() != 0 {
                let return_type = &method.returns[0];
                return_type_name = param_name!(return_type);
            }

            write!();
            write!("#[no_mangle]");
            write!("pub extern \"C\" fn {}({}) -> {}", method.name, parameters_str, return_type_name);
            write!("{");
            write!("}");
        }
    }

    fn is_name_reserved(&self, name: &String) -> bool {
        return match name.to_lowercase().as_ref() {
            "abstract" | "as" | "base" | "bool" | "break" | "byte" | "case" | 
            "catch" | "char" | "checked" | "class" | "const" | "continue" | "decimal" | 
            "default" | "delegate" | "do" | "double" | "else" | "enum" | "event" | 
            "explicit" | "extern" | "false" | "finally" | "fixed" | "float" | "for" | 
            "foreach" | "goto" | "if" | "implicit" | "in" | "int" | 
            "interface" | "internal" | "is" | "lock" | "long" | "namespace" | "new" | 
            "null" | "object" | "operator" | "out" | "override" | "params" | 
            "private" | "protected" | "public" | "readonly" | "ref" | "return" | "sbyte" | 
            "sealed" | "short" | "sizeof" | "stackalloc" | "static" | "string" | "struct" | 
            "switch" | "this" | "throw" | "true" | "try" | "typeof" | "uint" | 
            "ulong" | "unchecked" | "unsafe" | "ushort" | "using" | "virtual" | "void" | 
            "volatile" | "while" => true,
            _ => false
        }
    }
}