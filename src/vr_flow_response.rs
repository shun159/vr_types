// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::vr_flow::FlowOp;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_flow_response;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct FlowResponse {
    pub op: FlowOp,
    pub rid: u16,
    pub flags: u16,
    pub index: u32,
    pub bytes: u32,
    pub packets: u32,
    pub stats_oflow: u32,
    pub gen_id: i8
}

impl Default for FlowResponse {
    fn default() -> FlowResponse {
        FlowResponse {
            op: FlowOp::Get,
            rid: 0,
            flags: 0,
            index: 0,
            bytes: 0,
            packets: 0,
            stats_oflow: 0,
            gen_id: 0
        }
    }
}

impl FlowResponse {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
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
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<FlowResponse, &'a str> {
        let decoder: vr_flow_response = vr_flow_response::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to write binary"),
            Ok(_) => {
                let mut fresp: FlowResponse = FlowResponse::default();
                fresp.op = decoder.fresp_op.try_into().unwrap();
                fresp.rid = decoder.fresp_rid;
                fresp.flags = decoder.fresp_flags;
                fresp.index = decoder.fresp_index;
                fresp.bytes = decoder.fresp_bytes;
                fresp.packets = decoder.fresp_packets;
                fresp.stats_oflow = decoder.fresp_stats_oflow;
                fresp.gen_id = decoder.fresp_gen_id;
                Ok(fresp)
            },
        }
    }
}

#[cfg(test)]
mod test_vr_vxlan {
    use crate::vr_flow::FlowOp;
    use crate::vr_flow_response::FlowResponse;

    #[test]
    fn empty_request() {
        let fresp: FlowResponse = FlowResponse::default();
        let bytes = fresp.write().unwrap();
        let fresp: FlowResponse = FlowResponse::read(bytes).unwrap();
        assert_eq!(fresp.op, FlowOp::Get);
        assert_eq!(fresp.rid, 0);
        assert_eq!(fresp.flags, 0);
        assert_eq!(fresp.index, 0);
        assert_eq!(fresp.bytes, 0);
        assert_eq!(fresp.packets, 0);
        assert_eq!(fresp.stats_oflow, 0);
        assert_eq!(fresp.gen_id, 0);
    }

    #[test]
    fn complex_request() {
        let mut fresp: FlowResponse = FlowResponse::default();

        fresp.op = FlowOp::List;
        fresp.rid = 1;
        fresp.flags = 2;
        fresp.index = 3;
        fresp.bytes = 4;
        fresp.packets = 5;
        fresp.stats_oflow = 6;
        fresp.gen_id = 7;

        let bytes = fresp.write().unwrap();
        let fresp: FlowResponse = FlowResponse::read(bytes).unwrap();

        assert_eq!(fresp.op, FlowOp::List);
        assert_eq!(fresp.rid, 1);
        assert_eq!(fresp.flags, 2);
        assert_eq!(fresp.index, 3);
        assert_eq!(fresp.bytes, 4);
        assert_eq!(fresp.packets, 5);
        assert_eq!(fresp.stats_oflow, 6);
        assert_eq!(fresp.gen_id, 7);
    }
}
