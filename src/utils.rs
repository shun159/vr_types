// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use eui48::MacAddress;
use std::ffi::{CStr, CString};
use std::net::{Ipv4Addr, Ipv6Addr};
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
    let b: Box<[T]> = vec.clone().into_boxed_slice();
    Box::into_raw(b) as *mut _
}

pub fn write_mac(mac_addr: MacAddress) -> *mut i8 {
    let v: Vec<i8> = Vec::new();
    let octets = if mac_addr.is_nil() {
        vec![]
    } else {
        mac_addr.as_bytes().to_vec()
    };

    let mac = octets.iter().fold(v, |mut acc, &o| {
        acc.push(o as i8);
        acc
    });

    into_mut_ptr(&mac)
}

pub fn write_ip4(ip: Ipv4Addr) -> u32 {
    let v = ip.octets().to_vec();
    ((v[0] as u32) << 24) | ((v[1] as u32) << 16) | ((v[2] as u32) << 8) | (v[3] as u32)
}

pub fn write_ip6(ip: Ipv6Addr) -> u128 {
    let v: Vec<u8> = ip.octets().to_vec();
    ((v[0] as u128) << 120)
        | ((v[1] as u128) << 112)
        | ((v[2] as u128) << 104)
        | ((v[3] as u128) << 96)
        | ((v[4] as u128) << 88)
        | ((v[5] as u128) << 80)
        | ((v[6] as u128) << 72)
        | ((v[7] as u128) << 64)
        | ((v[8] as u128) << 56)
        | ((v[9] as u128) << 48)
        | ((v[10] as u128) << 40)
        | ((v[11] as u128) << 32)
        | ((v[12] as u128) << 24)
        | ((v[13] as u128) << 16)
        | ((v[14] as u128) << 8)
        | (v[15] as u128)
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
