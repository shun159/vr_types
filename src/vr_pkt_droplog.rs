// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::utils;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_pkt_drop_log_req;

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
            pkt_droplog_arr: vec![]
        }
    }
}

impl PktDropLog {
    
}
