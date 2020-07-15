// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_bridge_table_data;
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BridgeTableData {
    pub op: SandeshOp,
    pub rid: u16,
    pub size: u32,
    pub dev: u16,
    pub file_path: String,
}

impl Default for BridgeTableData {
    fn default() -> BridgeTableData {
        BridgeTableData {
            op: SandeshOp::Add,
            rid: 0,
            size: 0,
            dev: 0,
            file_path: "".to_string(),
        }
    }
}

impl BridgeTableData {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_bridge_table_data = vr_bridge_table_data::new();
        encoder.btable_op = self.op as u32;
        encoder.btable_rid = self.rid;
        encoder.btable_size = self.size;
        encoder.btable_dev = self.dev;
        encoder.btable_file_path = Self::write_cstring(&self.file_path);
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<BridgeTableData, &'a str> {
        let decoder: vr_bridge_table_data = vr_bridge_table_data::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to write binary"),
            Ok(_) => {
                let mut btable: BridgeTableData = BridgeTableData::default();
                btable.op = decoder.btable_op.try_into().unwrap();
                btable.rid = decoder.btable_rid;
                btable.size = decoder.btable_size;
                btable.dev = decoder.btable_dev;
                btable.file_path = Self::read_cstring(decoder.btable_file_path);
                Ok(btable)
            }
        }
    }

    // private functions

    fn write_cstring(s: &String) -> *mut c_char {
        let cs = CString::new(s.as_str()).unwrap();
        cs.into_raw()
    }

    fn read_cstring(ptr: *mut c_char) -> String {
        unsafe {
            if ptr.is_null() {
                return String::from("");
            }
            let cs = CString::from_raw(ptr);
            cs.to_string_lossy().into_owned()
        }
    }
}
