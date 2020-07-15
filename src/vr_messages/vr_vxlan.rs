// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_vxlan_req;
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
