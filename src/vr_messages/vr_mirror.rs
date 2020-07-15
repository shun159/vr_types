// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_mirror_req;
use std::convert::TryInto;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MirrorRequest {
    pub op: SandeshOp,
    pub index: i16,
    pub rid: i16,
    pub nhid: i32,
    pub users: i32,
    pub flags: i32,
    pub marker: i32,
    pub vni: i32,
    pub vlan: i16,
}

impl Default for MirrorRequest {
    fn default() -> MirrorRequest {
        MirrorRequest {
            op: SandeshOp::Add,
            index: 0,
            rid: 0,
            nhid: 0,
            users: 0,
            flags: 0,
            marker: 0,
            vni: 0,
            vlan: 0,
        }
    }
}

impl MirrorRequest {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_mirror_req = vr_mirror_req::new();
        encoder.h_op = self.op as u32;
        encoder.mirr_index = self.index;
        encoder.mirr_rid = self.rid;
        encoder.mirr_nhid = self.nhid;
        encoder.mirr_users = self.users;
        encoder.mirr_flags = self.flags;
        encoder.mirr_marker = self.marker;
        encoder.mirr_vni = self.vni;
        encoder.mirr_vlan = self.vlan;

        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<MirrorRequest, &'a str> {
        let decoder: vr_mirror_req = vr_mirror_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut mirr: MirrorRequest = MirrorRequest::default();
                mirr.op = decoder.h_op.try_into().unwrap();
                mirr.index = decoder.mirr_index;
                mirr.rid = decoder.mirr_rid;
                mirr.nhid = decoder.mirr_nhid;
                mirr.users = decoder.mirr_users;
                mirr.flags = decoder.mirr_flags;
                mirr.marker = decoder.mirr_marker;
                mirr.vni = decoder.mirr_vni;
                mirr.vlan = decoder.mirr_vlan;
                Ok(mirr)
            }
        }
    }
}
