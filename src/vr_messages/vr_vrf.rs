// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_vrf_req;
use std::convert::TryInto;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VrfRequest {
    pub op: SandeshOp,
    pub rid: i16,
    pub idx: i32,
    pub flags: i32,
    pub hbfl_vif_idx: i32,
    pub hbfr_vif_idx: i32,
    pub marker: i32,
}

impl Default for VrfRequest {
    fn default() -> VrfRequest {
        VrfRequest {
            op: SandeshOp::Add,
            rid: 0,
            idx: 0,
            flags: 0,
            hbfl_vif_idx: 0,
            hbfr_vif_idx: 0,
            marker: 0,
        }
    }
}

impl VrfRequest {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_vrf_req = vr_vrf_req::new();
        encoder.h_op = self.op as u32;
        encoder.vrf_rid = self.rid;
        encoder.vrf_idx = self.idx;
        encoder.vrf_flags = self.flags;
        encoder.vrf_hbfl_vif_idx = self.hbfl_vif_idx;
        encoder.vrf_hbfr_vif_idx = self.hbfr_vif_idx;
        encoder.vrf_marker = self.marker;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<VrfRequest, &'a str> {
        let decoder: vr_vrf_req = vr_vrf_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut vrf: VrfRequest = VrfRequest::default();
                vrf.op = decoder.h_op.try_into().unwrap();
                vrf.rid = decoder.vrf_rid;
                vrf.idx = decoder.vrf_idx;
                vrf.flags = decoder.vrf_flags;
                vrf.hbfl_vif_idx = decoder.vrf_hbfl_vif_idx;
                vrf.hbfr_vif_idx = decoder.vrf_hbfr_vif_idx;
                vrf.marker = decoder.vrf_marker;
                Ok(vrf)
            }
        }
    }
}
