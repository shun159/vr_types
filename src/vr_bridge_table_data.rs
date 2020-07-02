// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::utils;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_bridge_table_data;
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod test_vr_bridge_table_data {
    use crate::sandesh::SandeshOp;
    use crate::vr_bridge_table_data::BridgeTableData;

    #[test]
    fn empty_request() {
        let btable: BridgeTableData = BridgeTableData::default();
        let bytes = btable.write().unwrap();
        let btable: BridgeTableData = BridgeTableData::read(bytes).unwrap();
        assert_eq!(btable.op, SandeshOp::Add);
        assert_eq!(btable.rid, 0);
        assert_eq!(btable.size, 0);
        assert_eq!(btable.dev, 0);
        assert_eq!(btable.file_path, "".to_string());
    }

    #[test]
    fn complex_request() {
        let mut btable: BridgeTableData = BridgeTableData::default();

        btable.op = SandeshOp::Dump;
        btable.rid = 1;
        btable.size = 2;
        btable.dev = 3;
        btable.file_path = "/path/to/bridge_table_data".to_string();

        let bytes = btable.write().unwrap();
        let btable: BridgeTableData = BridgeTableData::read(bytes).unwrap();

        assert_eq!(btable.op, SandeshOp::Dump);
        assert_eq!(btable.rid, 1);
        assert_eq!(btable.size, 2);
        assert_eq!(btable.dev, 3);
        assert_eq!(btable.file_path, "/path/to/bridge_table_data".to_string());
    }
}
