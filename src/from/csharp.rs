use crate::interface;
use crate::from::writer::FromWriter;

use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CsharpWriter;

macro_rules! cc {
    ($text:expr => $args1:expr, $args2:expr) => {{
        if $text {
            $args1
        } else {
            $args2
        }
    }}
}

impl FromWriter for CsharpWriter {
    
    fn file_extension(&self) -> String { "cs".to_string() }

    fn write(&mut self, writer: &mut BufWriter<File>, interface: &interface::Interface)
    {
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
                    return_type_name = return_type_name + "[]";
                } else if !interface.is_param_blittable(&return_type) {
                    return_type_name = return_type_name + "_FFI";
                }
                return_type_name
            }}
        }

        macro_rules! param_name_or_ptr {
            ($param:expr) => {{
                let return_type = $param;
                let mut return_type_name = return_type.r#type.clone();
                if return_type.array.unwrap_or(false) {
                    return_type_name = "IntPtr".to_string();
                } else if !interface.is_param_blittable(&return_type) {
                    return_type_name = "IntPtr".to_string();
                }
                return_type_name
            }}
        }

        write!("// Autogenerated by FFIDJI");

        write!();
        write!("using System;");
        write!("using System.Runtime.InteropServices;");

        write!();
        write!("using int8 = System.SByte;");
        write!("using uint8 = System.Byte;");
        write!("using int16 = System.Int16;");
        write!("using uint16 = System.UInt16;");
        write!("using int32 = System.Int32;");
        write!("using uint32 = System.UInt32;");
        write!("using int64 = System.Int64;");
        write!("using uint64 = System.UInt64;");
        write!("using float16 = System.Half;");
        write!("using float32 = System.Single;");
        write!("using float64 = System.Double;");
        
        write!();
        write!("namespace {}", match &interface.namespace { Some(n) => n, None => "FFIDJI" }); // Could use unwrap_or() ?
        write!("{");
        write!("public static class {}", match &interface.name { Some(n) => n, None => "MyInterface" });
        write!("{");

        write!("public const string LIBRARY_NAME = \"MyLibrary.dll\";");

        for r#type in &interface.types
        {
            if r#type.base_type
            {
                continue;
            }

            // Public type
            write!();
            write!("[StructLayout(LayoutKind.Sequential)]");
            write!("public struct {}", r#type.name);
            write!("{");
            for field in &r#type.fields
            {
                if field.description.is_some()
                {
                    write!("// {}", field.description.as_ref().unwrap());
                }
                write!("public {} {};", param_name!(field), field.name);
            }
            write!("}");

            // FFI type and conversions (if not blittable)
            if !interface.is_type_blittable(r#type) {
                write!();
                write!("[StructLayout(LayoutKind.Sequential)]");
                write!("private struct {}_FFI", r#type.name);
                write!("{");
                for field in &r#type.fields
                {
                    write!("public {} {};", param_name_or_ptr!(field), field.name);
                }
                write!("}");

                write!();
                write!("private static {} Convert({}_FFI data_FFI)", r#type.name, r#type.name);
                write!("{");
                write!("return new {}", r#type.name);
                write!("{");
                for field in &r#type.fields
                {
                    // Todo:
                    // if base type, = assignment
                    // if blittable, marshal struct, (Generic method ?)
                    // if array of blittable, marshal array, (Generic method ?)
                    // otherwise, call convert
                    if interface.is_param_blittable(field) {

                    } else {
                        write!("{} = Convert(data_FFI.{}),", field.name, field.name);
                    }
                }
                indentation = indentation - 1;
                write!("};");
                write!("}");

                write!();
                write!("private static {}_FFI Convert({} data)", r#type.name, r#type.name);
                write!("{");
                write!("return new {}_FFI", r#type.name);
                write!("{");
                for field in &r#type.fields
                {
                    write!("{} = Convert(data.{}),", field.name, field.name);
                }
                indentation = indentation - 1;
                write!("};");
                write!("}");
            }
        }

        // Write methods
        for method in &interface.methods
        {
            {
                let parameters = &method.parameters;
                let parameters_str = parameters
                    .into_iter()
                    .map(|p| format!("{} {}", param_name!(p), p.name))
                    .collect::<Vec<String>>()
                    .join(", ");

                let mut return_type_name = String::from("void");
                if method.returns.len() != 0 {
                    let return_type = &method.returns[0];
                    return_type_name = param_name!(return_type);
                }

                write!();
                write!("[DllImport(LIBRARY_NAME)]");
                write!("private extern static {} {}_FFI({});", return_type_name, method.name, parameters_str);
            }

            {
                let parameters = &method.parameters;
                let parameters_str = parameters
                    .into_iter()
                    .map(|p| [p.r#type.clone(), p.name.clone()].join(" "))
                    .collect::<Vec<String>>()
                    .join(", ");

                let return_type_name = cc!(method.returns.len() == 0 => "void".to_string(), method.returns[0].r#type.clone());

                write!();
                write!("public static {} {}({})", return_type_name, method.name, parameters_str);
                write!("{");
                write!("}");
            }
        }

        write!("}");
        write!("}");
    }
}