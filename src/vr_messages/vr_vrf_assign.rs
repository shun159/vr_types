// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_vrf_assign_req;
use std::convert::TryInto;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct VrfAssignRequest {
    pub op: SandeshOp,
    pub rid: i16,
    pub vif_index: i16,
    pub vif_vrf: i32,
    pub vlan_id: i16,
    pub marker: i16,
    pub nh_id: i32,
}

impl VrfAssignRequest {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_vrf_assign_req = vr_vrf_assign_req::new();
        encoder.h_op = self.op as u32;
        encoder.var_rid = self.rid;
        encoder.var_vif_index = self.vif_index;
        encoder.var_vif_vrf = self.vif_vrf;
        encoder.var_vlan_id = self.vlan_id;
        encoder.var_marker = self.marker;
        encoder.var_nh_id = self.nh_id;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<VrfAssignRequest, &'a str> {
        let decoder: vr_vrf_assign_req = vr_vrf_assign_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut var: VrfAssignRequest = VrfAssignRequest::default();
                var.op = decoder.h_op.try_into().unwrap();
                var.rid = decoder.var_rid;
                var.vif_index = decoder.var_vif_index;
                var.vif_vrf = decoder.var_vif_vrf;
                var.vlan_id = decoder.var_vlan_id;
                var.marker = decoder.var_marker;
                var.nh_id = decoder.var_nh_id;
                Ok(var)
            }
        }
    }
}
