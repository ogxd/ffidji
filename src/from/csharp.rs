use crate::interface::Interface;
use crate::base::Writer;

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

impl Writer for CsharpWriter {
    
    fn file_extension(&self) -> String { "cs".to_string() }

    fn write(&mut self, writer: &mut BufWriter<File>, interface: &Interface)
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
                    return_type_name = format!("Arr<{}>", return_type_name);
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
        write!("using System.Runtime.CompilerServices;");
        write!("using System.Security;");

        write!();
        write!("using char16 = System.Char;");
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

        write!("public const string LIBRARY_NAME = \"MyNativeLibrary.dll\";");

        // Write utilities
        write!();
        write!("private readonly struct Arr<T>");
        write!("{");
        write!("public readonly IntPtr ptr;");
        write!("public readonly int size;");
        write!("public Arr(IntPtr ptr, int size)");
        write!("{");
        write!("this.ptr = ptr;");
        write!("this.size = size;");
        write!("}");
        write!("}");

        write!();
        write!("private unsafe static T[] CopyArray<T>(IntPtr ptr, int size) where T : unmanaged");
        write!("{");
        write!("int length = size * sizeof(T);");
        write!("T[] array = new T[size];");
        write!("void* u_src = ptr.ToPointer();");
        write!("fixed (T* u_dst = &array[0])");
        write!("{");
        write!("Unsafe.CopyBlock(u_dst, u_src, (uint)length);");
        write!("}");
        write!("return array;");
        write!("}");

        write!();
        write!("private static T[] Convert<T>(Arr<T> arr) where T : unmanaged");
        write!("{");
        write!("return CopyArray<T>(arr.ptr, arr.size);");
        write!("}");

        write!();
        write!("private static T Convert<T>(T obj) where T : unmanaged");
        write!("{");
        write!("return obj;");
        write!("}");

        write!();
        write!("private unsafe static Arr<T> Convert<T>(T[] array) where T : unmanaged");
        write!("{");
        write!("return Convert(new ReadOnlySpan<T>(array));");
        write!("}");

        write!();
        write!("private unsafe static Arr<T> Convert<T>(ReadOnlySpan<T> array) where T : unmanaged");
        write!("{");
        write!("int length = array.Length * sizeof(T);");
        write!("IntPtr ptr = Alloc(length);");
        write!("void* u_dst = ptr.ToPointer();");
        write!("fixed (T* u_src = &array[0])");
        write!("{");
        write!("Unsafe.CopyBlock(u_dst, u_src, (uint)length);");
        write!("}");
        write!("return new Arr<T>(ptr, array.Length);");
        write!("}");

        write!();
        write!("[SuppressUnmanagedCodeSecurity]");
        write!("[DllImport(LIBRARY_NAME, EntryPoint = \"Alloc_FFI\")]");
        write!("private static extern IntPtr Alloc(int length);");

        write!();
        write!("[SuppressUnmanagedCodeSecurity]");
        write!("[DllImport(LIBRARY_NAME, EntryPoint = \"Free_FFI\")]");
        write!("private static extern void Free(IntPtr ptr, int length);");

        for r#type in &interface.types {
            if r#type.base_type {
                continue;
            }

            let is_string = r#type.name == "string";

            // Public type
            if !is_string {
                write!();
                write!("[StructLayout(LayoutKind.Sequential)]");
                write!("public struct {}", r#type.name);
                write!("{");
                for field in &r#type.fields {
                    if field.description.is_some() {
                        write!("// {}", field.description.as_ref().unwrap());
                    }
                    write!("public {} {};", param_name!(field), field.name);
                }
                write!("}");
            }

            // FFI type and conversions (if not blittable)
            if !interface.is_type_blittable(r#type) {
                // FFI unmanaged structure
                write!();
                write!("[StructLayout(LayoutKind.Sequential)]");
                write!("private struct {}_FFI", r#type.name);
                write!("{");
                for field in &r#type.fields {
                    write!("public {} {};", param_name_or_ptr!(field), field.name);
                }
                write!("}");
                // Free FFI unmanaged data
                write!();
                write!("private static unsafe void Free({}_FFI input)", r#type.name);
                write!("{");
                for field in &r#type.fields {
                    if !interface.is_type_blittable(interface.get_type(&field.r#type)) {
                        if field.array.unwrap_or(false) {
                            write!("for (int i = 0; i < input.{}.size; i++)", field.name);
                            write!("{");
                            write!("Free(input.{}.ptr[i]);", field.name);
                            write!("}");
                        }
                        write!("Free(input.{});", field.name);
                    } else if field.array.unwrap_or(false) {
                        write!("Free(input.{}.ptr, input.{}.size * sizeof({}));", field.name, field.name, field.r#type);
                    }
                }
                write!("}");
                // Convert from unmanaged to managed
                write!();
                write!("private static {} Convert({}_FFI data_FFI)", r#type.name, r#type.name);
                write!("{");
                if is_string {
                    write!("unsafe");
                    write!("{");
                    write!("return new string((char*)data_FFI.utf16_char.ptr);");
                    write!("}");
                } else {
                    write!("return new {}", r#type.name);
                    write!("{");
                    for field in &r#type.fields {
                        if interface.is_param_blittable(field) {
                            write!("{} = data_FFI.{},", field.name, field.name);
                        } else {
                            write!("{} = Convert(data_FFI.{}),", field.name, field.name);
                        }
                    }
                    indentation = indentation - 1;
                    write!("};");
                }
                write!("}");
                // Convert from managed to unmanaged
                write!();
                write!("private static {}_FFI Convert({} data)", r#type.name, r#type.name);
                write!("{");
                write!("return new {}_FFI", r#type.name);
                write!("{");
                if is_string {
                    write!("utf16_char = Convert(data.AsSpan())");
                } else {
                    for field in &r#type.fields {
                        write!("{} = Convert(data.{}),", field.name, field.name);
                    }
                }
                indentation = indentation - 1;
                write!("};");
                write!("}");

                write!();
                write!("private unsafe static {}[] Convert(Arr<{}_FFI> arr)", r#type.name, r#type.name);
                write!("{");
                write!("var array_ffi = CopyArray<{}_FFI>(arr.ptr, arr.size);", r#type.name);
                write!("var array = new {}[arr.size];", r#type.name);
                write!("for (int i = 0; i < arr.size; ++i) array[i] = Convert(array_ffi[i]);");
                write!("return array;");
                write!("}");
            }
        }

        // Write methods
        for method in &interface.methods {
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
                write!("[SuppressUnmanagedCodeSecurity]");
                write!("[DllImport(LIBRARY_NAME, EntryPoint = \"{}\")]", method.name);
                write!("private extern static {} {}_FFI({});", return_type_name, method.name, parameters_str);
            }

            {
                let parameters = &method.parameters;
                let parameters_str = parameters
                    .into_iter()
                    .map(|p| [p.r#type.clone(), p.name.clone()].join(" "))
                    .collect::<Vec<String>>()
                    .join(", ");

                let convert_parameters_str = parameters
                    .into_iter()
                    .map(|p| format!("{}_ffi", p.name.clone()))
                    .collect::<Vec<String>>()
                    .join(", ");

                let return_type_name = cc!(method.returns.len() == 0 => "void".to_string(), method.returns[0].r#type.clone());

                write!();
                write!("public static {} {}({})", return_type_name, method.name, parameters_str);
                write!("{");
                for parameter in &method.parameters {
                    write!("var {}_ffi = Convert({});", parameter.name, parameter.name);
                }
                if method.returns.len() != 0 {
                    write!("var result_ffi = {}_FFI({});", method.name, convert_parameters_str);
                } else {
                    write!("{}_FFI({});", method.name, convert_parameters_str);
                }
                for parameter in &method.parameters {
                    if !interface.is_type_blittable(interface.get_type(&parameter.r#type)) {
                        write!("Free({}_ffi);", parameter.name);
                    }
                }
                if method.returns.len() != 0 {
                    let ret = &method.returns[0];
                    write!("var result = Convert(result_ffi);");
                    if !interface.is_type_blittable(interface.get_type(&ret.r#type)) {
                        write!("Free(result_ffi);");
                    }
                    write!("return result;");
                } else {
                    write!("Convert(result_ffi);");
                }
                write!("}");
            }
        }

        write!("}");
        write!("}");
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