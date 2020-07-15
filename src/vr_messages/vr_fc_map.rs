// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::sandesh::SandeshOp;
use crate::utils;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_fc_map_req;
use std::convert::TryInto;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FcMapRequest {
    pub op: SandeshOp,
    pub rid: u16,
    pub id: Vec<i16>,
    pub dscp: Vec<i8>,
    pub mpls_qos: Vec<i8>,
    pub dotonep: Vec<i8>,
    pub queue_id: Vec<i8>,
    pub marker: i16,
}

impl Default for FcMapRequest {
    fn default() -> FcMapRequest {
        FcMapRequest {
            op: SandeshOp::Add,
            rid: 0,
            id: vec![],
            dscp: vec![],
            mpls_qos: vec![],
            dotonep: vec![],
            queue_id: vec![],
            marker: 0,
        }
    }
}

impl FcMapRequest {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_fc_map_req = vr_fc_map_req::new();
        encoder.h_op = self.op as u32;
        encoder.fmr_rid = self.rid;
        encoder.fmr_id = utils::into_mut_ptr(&self.id);
        encoder.fmr_id_size = self.id.len() as u32;
        encoder.fmr_dscp = utils::into_mut_ptr(&self.dscp);
        encoder.fmr_dscp_size = self.dscp.len() as u32;
        encoder.fmr_mpls_qos = utils::into_mut_ptr(&self.mpls_qos);
        encoder.fmr_mpls_qos_size = self.mpls_qos.len() as u32;
        encoder.fmr_dotonep = utils::into_mut_ptr(&self.dotonep);
        encoder.fmr_dotonep_size = self.dotonep.len() as u32;
        encoder.fmr_queue_id = utils::into_mut_ptr(&self.queue_id);
        encoder.fmr_queue_id_size = self.queue_id.len() as u32;
        encoder.fmr_marker = self.marker;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<FcMapRequest, &'a str> {
        let decoder: vr_fc_map_req = vr_fc_map_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut fmr: FcMapRequest = FcMapRequest::default();
                fmr.op = decoder.h_op.try_into().unwrap();
                fmr.rid = decoder.fmr_rid;
                fmr.id = utils::free_buf(
                    decoder.fmr_id,
                    decoder.fmr_id_size as usize,
                );
                fmr.dscp = utils::free_buf(
                    decoder.fmr_dscp,
                    decoder.fmr_dscp_size as usize,
                );
                fmr.mpls_qos = utils::free_buf(
                    decoder.fmr_mpls_qos,
                    decoder.fmr_mpls_qos_size as usize,
                );
                fmr.dotonep = utils::free_buf(
                    decoder.fmr_dotonep,
                    decoder.fmr_dotonep_size as usize,
                );
                fmr.queue_id = utils::free_buf(
                    decoder.fmr_queue_id,
                    decoder.fmr_queue_id_size as usize,
                );
                fmr.marker = decoder.fmr_marker;
                Ok(fmr)
            }
        }
    }
}
