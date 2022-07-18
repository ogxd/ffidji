use super::interface::*;

fn to_ffi_string(s: String) -> string {
    let len = s.len() as i32;
    let cs = std::ffi::CString::new(s).unwrap();
    let p = cs.as_ptr();
    std::mem::forget(cs);
    string {
        utf8bytes_ptr: p,
        utf8bytes_len: len,
    }
}

fn from_ffi_string<'a>(s_ffi: string) -> &'a str {
    let c_str: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(s_ffi.utf8bytes_ptr) };
    c_str.to_str().unwrap()
}

pub fn concat(a: string, b: string) -> string
{
    // Simply concatenate the strings together.
    // We are given FFI objects. This is not always super convenient however it allows
    // operations that are only possible when playing with low level objects
    // Here is for instance a straight forward implementation using standard rust string, but not optimal:
    let mut a_string: String = from_ffi_string(a).to_owned();
    let b_str: &str = from_ffi_string(b);
    a_string.push_str(b_str);
    return to_ffi_string(a_string);
}

pub fn concat_optimized(a: string, b: string) -> string
{
    // Here is another way, unsafe but more optimized (less copies):
    unsafe {
        // strings are null terminated, so we do -1 to ignore first null termination
        // from string a to do the concatenation.
        let size = a.utf8bytes_len + b.utf8bytes_len - 1;
        let dst = libc::malloc(size as usize) as *mut i8;
        let dst2 = dst.offset(a.utf8bytes_len as isize - 1);
        std::ptr::copy(a.utf8bytes_ptr, dst, a.utf8bytes_len as usize - 1);
        std::ptr::copy(b.utf8bytes_ptr, dst2, b.utf8bytes_len as usize);
        string {
            utf8bytes_ptr: dst,
            utf8bytes_len: size,
        }
    }
}