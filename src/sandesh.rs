// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::vr_types_binding;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SandeshOp {
    Add,
    Get,
    Del,
    Dump,
    Response,
    Reset,
}

impl TryFrom<vr_types_binding::sandesh_op> for SandeshOp {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == SandeshOp::Add as u32 => Ok(SandeshOp::Add),
            x if x == SandeshOp::Get as u32 => Ok(SandeshOp::Get),
            x if x == SandeshOp::Del as u32 => Ok(SandeshOp::Del),
            x if x == SandeshOp::Dump as u32 => Ok(SandeshOp::Dump),
            x if x == SandeshOp::Response as u32 => Ok(SandeshOp::Response),
            x if x == SandeshOp::Reset as u32 => Ok(SandeshOp::Reset),
            _ => Err(()),
        }
    }
}
