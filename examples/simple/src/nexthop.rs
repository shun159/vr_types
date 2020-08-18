// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::vr_type::vr_messages::*;
use crate::vr_type::genetlink::MessageHandleError;
use crate::nix::libc::AF_INET;

pub fn init_nexthop() -> Result<Vec<Message>, MessageHandleError> {
    let mut nhr: NexthopRequest = NexthopRequest::default();
    nhr.op = SandeshOp::Add;
    nhr.id = 1;
    nhr.vrf = 1;
    nhr.family = AF_INET as i8;
    nhr._type = NhType::L2Rcv;
    nhr.flags = NH_FLAG_VALID | NH_FLAG_MAC_LEARN | NH_FLAG_UNKNOWN_UC_FLOOD | NH_FLAG_ROUTE_LOOKUP;
    let request = Message::NexthopRequest(nhr);
    request.send_nl()
}

pub fn get_nexthop(idx: i32) -> Result<Vec<Message>, MessageHandleError> {
    let mut nhr: NexthopRequest = NexthopRequest::default();
    nhr.op = SandeshOp::Get;
    nhr.id = idx;
    let request = Message::NexthopRequest(nhr);
    request.send_nl()
}
