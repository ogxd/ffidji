// Autogenerated by FFIDJI

// To get strings, use CString. Example:
// let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
use std::ffi::CStr; 

use std::alloc::GlobalAlloc;
use std::alloc::System;
use std::alloc::Layout;

use u16 as char16;
use i8 as int8;
use u8 as uint8;
use i16 as int16;
use u16 as uint16;
use i32 as int32;
use u32 as uint32;
use i64 as int64;
use u64 as uint64;
use f32 as float32;
use f64 as float64;

extern crate libc; // 0.2.65

#[no_mangle]
pub extern "C" fn Alloc_FFI(length :int32) -> *mut libc::c_void
{ 
    unsafe
    {
        return libc::malloc(length as usize);
        // let layout = Layout::from_size_align_unchecked(length as usize, 1);
        // return System.alloc(layout);
    } 
} 

#[no_mangle]
pub extern "C" fn Free_FFI(ptr: *mut libc::c_void, length :int32)
{ 
    unsafe
    { 
        //let layout = Layout::from_size_align_unchecked(length as usize, 1);
        //System.dealloc(ptr, layout);
        libc::free(ptr);
    } 
} 

struct PairToSum
{ 
    a: int32,
    b: int32,
} 

pub struct ArrayToSum
{ 
    intsToSum_ptr: *mut int32,
    intsToSum_len: int32,
} 

struct string
{ 
    utf16_char_ptr: *const char16,
    utf16_char_len: int32,
} 

#[no_mangle]
pub extern "C" fn Sum(A: int32, B: int32) -> int32
{ 
    return A + B;
} 

#[no_mangle]
pub extern "C" fn Reverse(input: ArrayToSum) -> ArrayToSum
{ 
    unsafe
    { 
        let ptr: *mut i32 = libc::malloc(std::mem::size_of::<int32>() * (input.intsToSum_len as usize)) as *mut i32;
        for i in 0..input.intsToSum_len {
            let d: *mut i32 = ptr.offset(i as isize);
            let s: *mut i32 = input.intsToSum_ptr.offset((input.intsToSum_len - i - 1) as isize);
            *d = *s;
        }
        return ArrayToSum {
            intsToSum_ptr: ptr,
            intsToSum_len: input.intsToSum_len
        }
    } 
    
} 
