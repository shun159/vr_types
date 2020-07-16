// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_mpls_req;
use std::convert::TryInto;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct MplsRequest {
    pub op: SandeshOp,
    pub rid: i16,
    pub label: i32,
    pub nhid: i32,
    pub marker: i32,
}

impl MplsRequest {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_mpls_req = vr_mpls_req::new();
        encoder.h_op = self.op as u32;
        encoder.mr_rid = self.rid;
        encoder.mr_label = self.label;
        encoder.mr_nhid = self.nhid;
        encoder.mr_label = self.label;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<MplsRequest, &'a str> {
        let decoder: vr_mpls_req = vr_mpls_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut mr: MplsRequest = MplsRequest::default();
                mr.op = decoder.h_op.try_into().unwrap();
                mr.rid = decoder.mr_rid;
                mr.label = decoder.mr_label;
                mr.nhid = decoder.mr_nhid;
                mr.label = decoder.mr_label;
                Ok(mr)
            }
        }
    }
}
