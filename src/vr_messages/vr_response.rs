// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_response;
use std::convert::TryInto;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct VrResponse {
    pub op: SandeshOp,
    pub read_length: usize,
    pub code: i32
}

impl VrResponse {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_response = vr_response::new();
        encoder.h_op = self.op as u32;
        encoder.resp_code = self.code;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<VrResponse, &'a str> {
        let decoder: vr_response = vr_response::new();
        match decoder.read(&buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(rxfer) => {
                let mut resp: VrResponse = VrResponse::default();
                resp.read_length = rxfer as usize;
                resp.op = decoder.h_op.try_into().unwrap();
                resp.code = decoder.resp_code;
                Ok(resp)
            }
        }
    }
}
