// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_vrf_req;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
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

#[cfg(test)]
mod test_vr_vrf {
    use crate::sandesh::SandeshOp;
    use crate::vr_vrf::VrfRequest;

    #[test]
    fn empty_request() {
        let vrf: VrfRequest = VrfRequest::default();
        let bytes = vrf.write().unwrap();
        let vrf: VrfRequest = VrfRequest::read(bytes).unwrap();
        assert_eq!(vrf.op, SandeshOp::Add);
        assert_eq!(vrf.rid, 0);
        assert_eq!(vrf.idx, 0);
        assert_eq!(vrf.flags, 0);
        assert_eq!(vrf.hbfl_vif_idx, 0);
        assert_eq!(vrf.hbfr_vif_idx, 0);
        assert_eq!(vrf.marker, 0);
    }
    #[test]
    fn complex_request() {
        let mut vrf: VrfRequest = VrfRequest::default();
        vrf.op = SandeshOp::Dump;
        vrf.rid = 1;
        vrf.idx = 1;
        vrf.flags = 1;
        vrf.hbfl_vif_idx = 1;
        vrf.hbfr_vif_idx = 2;
        vrf.marker = 1;

        let bytes = vrf.write().unwrap();
        let vrf: VrfRequest = VrfRequest::read(bytes).unwrap();

        assert_eq!(vrf.op, SandeshOp::Dump);
        assert_eq!(vrf.rid, 1);
        assert_eq!(vrf.idx, 1);
        assert_eq!(vrf.flags, 1);
        assert_eq!(vrf.hbfl_vif_idx, 1);
        assert_eq!(vrf.hbfr_vif_idx, 2);
        assert_eq!(vrf.marker, 1);
    }
}
