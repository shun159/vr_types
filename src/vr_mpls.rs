// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_mpls_req;
use std::convert::TryInto;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MplsRequest {
    pub op: SandeshOp,
    pub rid: i16,
    pub label: i32,
    pub nhid: i32,
    pub marker: i32,
}

impl Default for MplsRequest {
    fn default() -> MplsRequest {
        MplsRequest {
            op: SandeshOp::Add,
            rid: 0,
            label: 0,
            nhid: 0,
            marker: 0,
        }
    }
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

#[cfg(test)]
mod test_vr_mpls {
    use crate::sandesh::SandeshOp;
    use crate::vr_mpls::MplsRequest;

    #[test]
    fn empty_request() {
        let mr: MplsRequest = MplsRequest::default();
        let bytes = mr.write().unwrap();
        let mr: MplsRequest = MplsRequest::read(bytes).unwrap();
        assert_eq!(mr.op, SandeshOp::Add);
        assert_eq!(mr.rid, 0);
        assert_eq!(mr.label, 0);
        assert_eq!(mr.nhid, 0);
    }

    #[test]
    fn complex_request() {
        let mut mr: MplsRequest = MplsRequest::default();
        mr.op = SandeshOp::Dump;
        mr.rid = 1;
        mr.label = 1;
        mr.nhid = 1;
        let bytes = mr.write().unwrap();
        let mr: MplsRequest = MplsRequest::read(bytes).unwrap();
        assert_eq!(mr.op, SandeshOp::Dump);
        assert_eq!(mr.rid, 1);
        assert_eq!(mr.label, 1);
        assert_eq!(mr.nhid, 1);
    }
}
