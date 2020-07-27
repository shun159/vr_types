// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::error::CodecError;
use super::vr_flow::FlowOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_flow_response;
use std::convert::TryInto;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct FlowResponse {
    pub op: FlowOp,
    pub read_length: usize,
    pub rid: u16,
    pub flags: u16,
    pub index: u32,
    pub bytes: u32,
    pub packets: u32,
    pub stats_oflow: u32,
    pub gen_id: i8,
}

impl FlowResponse {
    pub fn write(&self) -> Result<Vec<u8>, CodecError> {
        let mut encoder: vr_flow_response = vr_flow_response::new();
        encoder.fresp_op = self.op as u32;
        encoder.fresp_rid = self.rid;
        encoder.fresp_flags = self.flags;
        encoder.fresp_index = self.index;
        encoder.fresp_bytes = self.bytes;
        encoder.fresp_packets = self.packets;
        encoder.fresp_stats_oflow = self.stats_oflow;
        encoder.fresp_gen_id = self.gen_id;
        match encoder.write() {
            Err(e) => Err(e),
            Ok(v) => Ok(v),
        }
    }

    pub fn read(buf: Vec<u8>) -> Result<FlowResponse, CodecError> {
        let decoder: vr_flow_response = vr_flow_response::new();
        match decoder.read(&buf) {
            Err(e) => Err(e),
            Ok(rxfer) => {
                let mut fresp: FlowResponse = FlowResponse::default();
                fresp.read_length = rxfer as usize;
                fresp.op = decoder.fresp_op.try_into().unwrap();
                fresp.rid = decoder.fresp_rid;
                fresp.flags = decoder.fresp_flags;
                fresp.index = decoder.fresp_index;
                fresp.bytes = decoder.fresp_bytes;
                fresp.packets = decoder.fresp_packets;
                fresp.stats_oflow = decoder.fresp_stats_oflow;
                fresp.gen_id = decoder.fresp_gen_id;
                Ok(fresp)
            }
        }
    }
}
