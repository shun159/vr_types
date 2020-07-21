// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::vr_flow::FlowOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_flow_table_data;
use crate::utils;
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct FlowTableData {
    pub op: FlowOp,
    pub read_length: usize,
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
        match decoder.read(&buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(rxfer) => {
                let mut ftable: FlowTableData = FlowTableData::default();
                ftable.read_length = rxfer as usize;
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
