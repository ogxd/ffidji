use crate::interface::Interface;
use crate::base::Writer;

use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CWriter;

impl Writer for CWriter {
    
    fn file_extension(&self) -> String { "h".to_string() }

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

        write!("#include <stdint.h>");
        write!("#include <stdlib.h>");

        write!();
        write!("#ifdef __cplusplus");
        write!("extern \"C\"");
        write!("{ ");
        write!("#endif");

        write!();
        write!("typedef char16_t char16;");
        write!("typedef int8_t int8;");
        write!("typedef uint8_t uint8;");
        write!("typedef int16_t int16;");
        write!("typedef uint16_t uint16;");
        write!("typedef int32_t int32;");
        write!("typedef uint32_t uint32;");
        write!("typedef long int64;");
        write!("typedef unsigned long uint64;");
        write!("typedef float float32;");
        write!("typedef double float64;");

        write!();
        write!("__declspec(dllexport) inline void* Alloc_FFI(int32 length)");
        write!("{");
        write!("return (void*)malloc(length);");
        write!("}");

        write!();
        write!("__declspec(dllexport) inline void Free_FFI(void* ptr, int32 length)");
        write!("{");
        write!("free(ptr);");
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
                    write!("{}* {}_ptr;", field.r#type, field.name);
                    write!("int32 {}_len;", field.name);
                } else {
                    write!("{} {};", field.r#type, field.name);
                }
            }
            indentation = indentation - 1;
            write!("};");
        }

        for method in &interface.methods {
            let parameters = &method.parameters;
            let parameters_str = parameters
                .into_iter()
                .map(|p| [p.r#type.clone(), p.name.clone()].join(" "))
                .collect::<Vec<String>>()
                .join(", ");

            let mut return_type_name = String::from("void");
            if method.returns.len() != 0 {
                let return_type = &method.returns[0];
                return_type_name = param_name!(return_type);
            }

            write!();
            write!("__declspec(dllexport) {} {}({});", return_type_name, method.name, parameters_str);
        }

        write!();
        write!("#ifdef __cplusplus");
        write!("} ");
        write!("#endif");
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