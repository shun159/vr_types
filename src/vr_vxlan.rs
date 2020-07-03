// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_vxlan_req;
use std::convert::TryInto;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VxlanRequest {
    pub op: SandeshOp,
    pub rid: i16,
    pub vnid: i32,
    pub nhid: i32,
}

impl Default for VxlanRequest {
    fn default() -> VxlanRequest {
        VxlanRequest {
            op: SandeshOp::Add,
            rid: 0,
            vnid: 0,
            nhid: 0,
        }
    }
}

impl VxlanRequest {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_vxlan_req = vr_vxlan_req::new();
        encoder.h_op = self.op as u32;
        encoder.vxlanr_rid = self.rid;
        encoder.vxlanr_vnid = self.vnid;
        encoder.vxlanr_nhid = self.nhid;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<VxlanRequest, &'a str> {
        let decoder: vr_vxlan_req = vr_vxlan_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut vxlanr: VxlanRequest = VxlanRequest::default();
                vxlanr.op = decoder.h_op.try_into().unwrap();
                vxlanr.rid = decoder.vxlanr_rid;
                vxlanr.vnid = decoder.vxlanr_vnid;
                vxlanr.nhid = decoder.vxlanr_nhid;
                Ok(vxlanr)
            }
        }
    }
}

#[cfg(test)]
mod test_vr_vxlan {
    use crate::sandesh::SandeshOp;
    use crate::vr_vxlan::VxlanRequest;

    #[test]
    fn empty_request() {
        let vxlanr: VxlanRequest = VxlanRequest::default();
        let bytes = vxlanr.write().unwrap();
        let vxlanr: VxlanRequest = VxlanRequest::read(bytes).unwrap();
        assert_eq!(vxlanr.op, SandeshOp::Add);
        assert_eq!(vxlanr.rid, 0);
        assert_eq!(vxlanr.vnid, 0);
        assert_eq!(vxlanr.nhid, 0);
    }

    #[test]
    fn complex_request() {
        let mut vxlanr: VxlanRequest = VxlanRequest::default();
        vxlanr.op = SandeshOp::Dump;
        vxlanr.rid = 1;
        vxlanr.vnid = 1;
        vxlanr.nhid = 1;
        let bytes = vxlanr.write().unwrap();
        let vxlanr: VxlanRequest = VxlanRequest::read(bytes).unwrap();
        assert_eq!(vxlanr.op, SandeshOp::Dump);
        assert_eq!(vxlanr.rid, 1);
        assert_eq!(vxlanr.vnid, 1);
        assert_eq!(vxlanr.nhid, 1);
    }
}
