// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::error::CodecError;
use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_response;
use std::convert::TryInto;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct VrResponse {
    pub op: SandeshOp,
    pub read_length: usize,
    pub code: i32,
}

impl VrResponse {
    pub fn write(&self) -> Result<Vec<u8>, CodecError> {
        let mut encoder: vr_response = vr_response::new();
        encoder.h_op = self.op as u32;
        encoder.resp_code = self.code;
        encoder.write()
    }

    pub fn read(buf: Vec<u8>) -> Result<VrResponse, CodecError> {
        let decoder: vr_response = vr_response::new();
        let rxfer = decoder.read(&buf)?;
        let mut resp: VrResponse = VrResponse::default();
        resp.read_length = rxfer as usize;
        resp.op = decoder.h_op.try_into().unwrap();
        resp.code = decoder.resp_code;
        Ok(resp)
    }
}
