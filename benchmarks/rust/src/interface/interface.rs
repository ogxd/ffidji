// Autogenerated by FFIDJI

// Implement the methods here
use super::implementation;

extern crate libc;
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

#[no_mangle]
pub extern "C" fn Alloc_FFI(length :int32) -> *mut libc::c_void
{ 
    unsafe
    { 
        return libc::malloc(length as usize);
    } 
} 

#[no_mangle]
pub extern "C" fn Free_FFI(ptr: *mut libc::c_void, length :int32)
{ 
    unsafe
    { 
        libc::free(ptr);
    } 
} 

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strings
{ 
    pub array_ptr: *const string,
    pub array_len: int32,
} 

#[repr(C)]
#[derive(Copy, Clone)]
pub struct string
{ 
    pub utf8bytes_ptr: *const int8,
    pub utf8bytes_len: int32,
} 

#[no_mangle]
pub extern "C" fn Concat(a: string, b: string) -> string
{ 
    implementation::concat(a, b)
} 

#[no_mangle]
pub extern "C" fn ConcatOptimized(a: string, b: string) -> string
{ 
    implementation::concat_optimized(a, b)
} 