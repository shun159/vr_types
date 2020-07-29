// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::error::CodecError;
use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_vxlan_req;
use std::convert::TryInto;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct VxlanRequest {
    pub op: SandeshOp,
    pub read_length: usize,
    pub rid: i16,
    pub vnid: i32,
    pub nhid: i32,
}

impl VxlanRequest {
    pub fn write(&self) -> Result<Vec<u8>, CodecError> {
        let mut encoder: vr_vxlan_req = vr_vxlan_req::new();
        encoder.h_op = self.op as u32;
        encoder.vxlanr_rid = self.rid;
        encoder.vxlanr_vnid = self.vnid;
        encoder.vxlanr_nhid = self.nhid;
        encoder.write()
    }

    pub fn read(buf: Vec<u8>) -> Result<VxlanRequest, CodecError> {
        let decoder: vr_vxlan_req = vr_vxlan_req::new();
        let rxfer = decoder.read(&buf)?;
        let mut vxlanr: VxlanRequest = VxlanRequest::default();
        vxlanr.read_length = rxfer as usize;
        vxlanr.op = decoder.h_op.try_into().unwrap();
        vxlanr.rid = decoder.vxlanr_rid;
        vxlanr.vnid = decoder.vxlanr_vnid;
        vxlanr.nhid = decoder.vxlanr_nhid;
        Ok(vxlanr)
    }
}
