// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_response;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
pub struct VrResponse {
    pub op: SandeshOp,
    pub code: i32,
}

impl Default for VrResponse {
    fn default() -> VrResponse {
        VrResponse {
            op: SandeshOp::Add,
            code: 0,
        }
    }
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
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut resp: VrResponse = VrResponse::default();
                resp.op = decoder.h_op.try_into().unwrap();
                resp.code = decoder.resp_code;
                Ok(resp)
            }
        }
    }
}
