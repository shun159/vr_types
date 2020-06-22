// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use eui48::MacAddress;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

pub fn str_to_cchar(s: &str) -> *const c_char {
    let c_str = CString::new(s).unwrap();
    let c_string: *const c_char = c_str.as_ptr() as *const c_char;
    c_string
}

pub fn cchar_to_str(s: *const c_char) -> &'static str {
    unsafe {
        let char_ptr = CStr::from_ptr(s);
        char_ptr.to_str().unwrap()
    }
}

pub fn into_raw_ptr<T>(term: &T) -> *const c_void {
    &(*term) as *const _ as *const c_void
}

pub fn alloc_buf(len: usize) -> *mut u8 {
    let vec = vec![0; len];
    Box::into_raw(vec.into_boxed_slice()) as *mut u8
}

pub fn free_buf<T: Clone>(buf: *mut T, buf_len: usize) -> Vec<T> {
    unsafe {
        let s = std::slice::from_raw_parts_mut(buf, buf_len);
        let r = Box::from_raw(s);
        r.to_vec()
    }
}

pub fn into_mut_ptr<T: Clone>(vec: &Vec<T>) -> *mut T {
    let mut b: Box<[T]> = vec.clone().into_boxed_slice();
    b.as_mut_ptr()
}

pub fn read_mac_addr(mac_addr: *mut i8, mac_addr_size: u32) -> MacAddress {
    if mac_addr_size == libc::ETH_ALEN as u32 {
        MacAddress::from_bytes(&*free_buf::<u8>(
            mac_addr as *mut u8,
            libc::ETH_ALEN as usize,
        ))
        .unwrap()
    } else {
        MacAddress::nil()
    }
}
