// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::utils;
use crate::vr_flow::FlowOp;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_flow_table_data;
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, PartialEq)]
pub struct FlowTableData {
    pub op: FlowOp,
    pub rid: u16,
    pub size: u32,
    pub dev: u16,
    pub file_path: String,
    pub used_entries: u64,
    pub processed: u64,
    pub deleted: u64,
    pub added: u64,
    pub created: u64,
    pub changed: u64,
    pub hold_oflows: u32,
    pub cpus: u32,
    pub oflow_entries: u32,
    pub hold_stat: Vec<u32>,
    pub burst_free_tokens: u32,
    pub hold_entries: u32,
}

impl Default for FlowTableData {
    fn default() -> FlowTableData {
        FlowTableData {
            op: FlowOp::Get,
            rid: 0,
            size: 0,
            dev: 0,
            file_path: "".to_string(),
            used_entries: 0,
            processed: 0,
            deleted: 0,
            added: 0,
            created: 0,
            changed: 0,
            hold_oflows: 0,
            cpus: 0,
            oflow_entries: 0,
            hold_stat: vec![],
            burst_free_tokens: 0,
            hold_entries: 0,
        }
    }
}

impl FlowTableData {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_flow_table_data = vr_flow_table_data::new();
        encoder.ftable_op = self.op as u32;
        encoder.ftable_rid = self.rid;
        encoder.ftable_size = self.size;
        encoder.ftable_dev = self.dev;
        encoder.ftable_file_path = Self::write_cstring(&self.file_path);
        encoder.ftable_used_entries = self.used_entries;
        encoder.ftable_processed = self.processed;
        encoder.ftable_deleted = self.deleted;
        encoder.ftable_added = self.added;
        encoder.ftable_created = self.created;
        encoder.ftable_changed = self.changed;
        encoder.ftable_hold_oflows = self.hold_oflows;
        encoder.ftable_cpus = self.cpus;
        encoder.ftable_oflow_entries = self.oflow_entries;
        encoder.ftable_hold_stat = utils::into_mut_ptr(&self.hold_stat);
        encoder.ftable_hold_stat_size = self.hold_stat.len() as u32;
        encoder.ftable_burst_free_tokens = self.burst_free_tokens;
        encoder.ftable_hold_entries = self.hold_entries;

        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<FlowTableData, &'a str> {
        let decoder: vr_flow_table_data = vr_flow_table_data::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut ftable: FlowTableData = FlowTableData::default();
                ftable.op = decoder.ftable_op.try_into().unwrap();
                ftable.rid = decoder.ftable_rid;
                ftable.size = decoder.ftable_size;
                ftable.dev = decoder.ftable_dev;
                ftable.file_path = Self::read_cstring(decoder.ftable_file_path);
                ftable.used_entries = decoder.ftable_used_entries;
                ftable.processed = decoder.ftable_processed;
                ftable.deleted = decoder.ftable_deleted;
                ftable.added = decoder.ftable_added;
                ftable.created = decoder.ftable_created;
                ftable.changed = decoder.ftable_changed;
                ftable.hold_oflows = decoder.ftable_hold_oflows;
                ftable.cpus = decoder.ftable_cpus;
                ftable.oflow_entries = decoder.ftable_oflow_entries;
                ftable.hold_stat = utils::free_buf(
                    decoder.ftable_hold_stat,
                    decoder.ftable_hold_stat_size as usize,
                );
                ftable.burst_free_tokens = decoder.ftable_burst_free_tokens;
                ftable.hold_entries = decoder.ftable_hold_entries;
                Ok(ftable)
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
mod test_vr_flow_table_data {
    use crate::vr_flow::FlowOp;
    use crate::vr_flow_table_data::FlowTableData;

    #[test]
    fn empty_request() {
        let ftable: FlowTableData = FlowTableData::default();
        let bytes = ftable.write().unwrap();
        let ftable: FlowTableData = FlowTableData::read(bytes).unwrap();
        assert_eq!(ftable.op, FlowOp::Get);
        assert_eq!(ftable.rid, 0);
        assert_eq!(ftable.size, 0);
        assert_eq!(ftable.dev, 0);
        assert_eq!(ftable.file_path, "".to_string());
        assert_eq!(ftable.used_entries, 0);
        assert_eq!(ftable.processed, 0);
        assert_eq!(ftable.deleted, 0);
        assert_eq!(ftable.added, 0);
        assert_eq!(ftable.created, 0);
        assert_eq!(ftable.changed, 0);
        assert_eq!(ftable.hold_oflows, 0);
        assert_eq!(ftable.cpus, 0);
        assert_eq!(ftable.oflow_entries, 0);
        assert_eq!(ftable.hold_stat, vec![]);
        assert_eq!(ftable.burst_free_tokens, 0);
        assert_eq!(ftable.hold_entries, 0);
    }

    #[test]
    fn complex_request() {
        let mut ftable: FlowTableData = FlowTableData::default();

        ftable.op = FlowOp::List;
        ftable.rid = 1;
        ftable.size = 2;
        ftable.dev = 3;
        ftable.file_path = "/path/to/flow_table_data".to_string();
        ftable.used_entries = 5;
        ftable.processed = 6;
        ftable.deleted = 7;
        ftable.added = 8;
        ftable.created = 9;
        ftable.changed = 10;
        ftable.hold_oflows = 11;
        ftable.cpus = 12;
        ftable.oflow_entries = 13;
        ftable.hold_stat = vec![1, 2, 3, 4, 5];
        ftable.burst_free_tokens = 15;
        ftable.hold_entries = 16;

        let bytes = ftable.write().unwrap();
        let ftable: FlowTableData = FlowTableData::read(bytes).unwrap();

        assert_eq!(ftable.op, FlowOp::List);
        assert_eq!(ftable.rid, 1);
        assert_eq!(ftable.size, 2);
        assert_eq!(ftable.dev, 3);
        assert_eq!(ftable.file_path, "/path/to/flow_table_data".to_string());
        assert_eq!(ftable.used_entries, 5);
        assert_eq!(ftable.processed, 6);
        assert_eq!(ftable.deleted, 7);
        assert_eq!(ftable.added, 8);
        assert_eq!(ftable.created, 9);
        assert_eq!(ftable.changed, 10);
        assert_eq!(ftable.hold_oflows, 11);
        assert_eq!(ftable.cpus, 12);
        assert_eq!(ftable.oflow_entries, 13);
        assert_eq!(ftable.hold_stat, vec![1, 2, 3, 4, 5]);
        assert_eq!(ftable.burst_free_tokens, 15);
        assert_eq!(ftable.hold_entries, 16);
    }
}
