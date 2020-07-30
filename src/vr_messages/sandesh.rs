// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::vr_types_binding::{
    _sandesh_op_SANDESH_OP_ADD, _sandesh_op_SANDESH_OP_DEL, _sandesh_op_SANDESH_OP_DUMP,
    _sandesh_op_SANDESH_OP_GET, _sandesh_op_SANDESH_OP_RESET,
    _sandesh_op_SANDESH_OP_RESPONSE, sandesh_op,
};
use std::convert::TryFrom;

pub const SANDESH_OP_ADD: u32 = _sandesh_op_SANDESH_OP_ADD;
pub const SANDESH_OP_GET: u32 = _sandesh_op_SANDESH_OP_GET;
pub const SANDESH_OP_DEL: u32 = _sandesh_op_SANDESH_OP_DEL;
pub const SANDESH_OP_DUMP: u32 = _sandesh_op_SANDESH_OP_DUMP;
pub const SANDESH_OP_RESPONSE: u32 = _sandesh_op_SANDESH_OP_RESPONSE;
pub const SANDESH_OP_RESET: u32 = _sandesh_op_SANDESH_OP_RESET;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SandeshOp {
    Add,
    Get,
    Del,
    Dump,
    Response,
    Reset,
}

impl Default for SandeshOp {
    fn default() -> SandeshOp { SandeshOp::Add }
}

impl TryFrom<sandesh_op> for SandeshOp {
    type Error = ();

    fn try_from(v: sandesh_op) -> Result<Self, Self::Error> {
        match v {
            SANDESH_OP_ADD => Ok(SandeshOp::Add),
            SANDESH_OP_GET => Ok(SandeshOp::Get),
            SANDESH_OP_DEL => Ok(SandeshOp::Del),
            SANDESH_OP_DUMP => Ok(SandeshOp::Dump),
            SANDESH_OP_RESPONSE => Ok(SandeshOp::Response),
            SANDESH_OP_RESET => Ok(SandeshOp::Reset),
            _ => Err(()),
        }
    }
}
