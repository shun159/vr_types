// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::utils;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_pkt_drop_log_req;
use std::convert::TryInto;

pub const VR_PKT_DROP_LOG_MAX: u32 = 200;

#[derive(Debug, PartialEq)]
pub struct PktDropLog {
    pub op: SandeshOp,
    pub rid: i16,
    pub core: i16,
    pub log_idx: i16,
    pub max_num_cores: i16,
    pub pkt_droplog_max_bufsz: i16,
    pub pkt_droplog_en: i16,
    pub pkt_droplog_sysctl_en: i16,
    pub pkt_droplog_arr: Vec<i8>,
}

impl Default for PktDropLog {
    fn default() -> PktDropLog {
        PktDropLog {
            op: SandeshOp::Add,
            rid: 0,
            core: 0,
            log_idx: 0,
            max_num_cores: 0,
            pkt_droplog_max_bufsz: 0,
            pkt_droplog_en: 0,
            pkt_droplog_sysctl_en: 0,
            pkt_droplog_arr: vec![],
        }
    }
}

impl PktDropLog {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_pkt_drop_log_req = vr_pkt_drop_log_req::new();
        encoder.vdl_rid = self.rid;
        encoder.vdl_core = self.core;
        encoder.vdl_log_idx = self.log_idx;
        encoder.vdl_max_num_cores = self.max_num_cores;
        encoder.vdl_pkt_droplog_max_bufsz = self.pkt_droplog_max_bufsz;
        encoder.vdl_pkt_droplog_en = self.pkt_droplog_en;
        encoder.vdl_pkt_droplog_sysctl_en = self.pkt_droplog_sysctl_en;
        encoder.vdl_pkt_droplog_arr =
            utils::into_mut_ptr(&self.pkt_droplog_arr);
        encoder.vdl_pkt_droplog_arr_size = self.pkt_droplog_arr.len() as u32;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<PktDropLog, &'a str> {
        let decoder: vr_pkt_drop_log_req = vr_pkt_drop_log_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut vdl: PktDropLog = PktDropLog::default();
                vdl.op = decoder.h_op.try_into().unwrap();
                vdl.rid = decoder.vdl_rid;
                vdl.core = decoder.vdl_core;
                vdl.log_idx = decoder.vdl_log_idx;
                vdl.max_num_cores = decoder.vdl_max_num_cores;
                vdl.pkt_droplog_max_bufsz = decoder.vdl_pkt_droplog_max_bufsz;
                vdl.pkt_droplog_en = decoder.vdl_pkt_droplog_en;
                vdl.pkt_droplog_sysctl_en = decoder.vdl_pkt_droplog_sysctl_en;
                vdl.pkt_droplog_arr = utils::free_buf(
                    decoder.vdl_pkt_droplog_arr,
                    decoder.vdl_pkt_droplog_arr_size as usize,
                );
                Ok(vdl)
            }
        }
    }
}

#[cfg(test)]
mod test_vr_pkt_droplog {
    use crate::sandesh::SandeshOp;
    use crate::vr_pkt_droplog::PktDropLog;

    #[test]
    fn empty_request() {
        let vdl: PktDropLog = PktDropLog::default();
        let bytes = vdl.write().unwrap();
        let vdl: PktDropLog = PktDropLog::read(bytes).unwrap();
        assert_eq!(vdl.op, SandeshOp::Add);
        assert_eq!(vdl.rid, 0);
        assert_eq!(vdl.core, 0);
        assert_eq!(vdl.log_idx, 0);
        assert_eq!(vdl.max_num_cores, 0);
        assert_eq!(vdl.pkt_droplog_max_bufsz, 0);
        assert_eq!(vdl.pkt_droplog_en, 0);
        assert_eq!(vdl.pkt_droplog_sysctl_en, 0);
        assert_eq!(vdl.pkt_droplog_arr, vec![]);
    }

    #[test]
    fn complex_request() {
        let mut vdl: PktDropLog = PktDropLog::default();

        vdl.op = SandeshOp::Dump;
        vdl.rid = 1;
        vdl.core = 2;
        vdl.log_idx = 3;
        vdl.max_num_cores = 4;
        vdl.pkt_droplog_max_bufsz = 5;
        vdl.pkt_droplog_en = 6;
        vdl.pkt_droplog_sysctl_en = 7;
        vdl.pkt_droplog_arr = vec![1, 2, 3, 4, 5];

        let bytes = vdl.write().unwrap();
        let vdl: PktDropLog = PktDropLog::read(bytes).unwrap();

        assert_eq!(vdl.rid, 1);
        assert_eq!(vdl.core, 2);
        assert_eq!(vdl.log_idx, 3);
        assert_eq!(vdl.max_num_cores, 4);
        assert_eq!(vdl.pkt_droplog_max_bufsz, 5);
        assert_eq!(vdl.pkt_droplog_en, 6);
        assert_eq!(vdl.pkt_droplog_sysctl_en, 7);
        assert_eq!(vdl.pkt_droplog_arr, vec![1, 2, 3, 4, 5]);
    }
}
