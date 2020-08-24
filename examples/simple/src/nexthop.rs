// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::nix::libc::{AF_BRIDGE, AF_UNSPEC};
use crate::vr_type::genetlink::MessageHandleError;
use crate::vr_type::vr_messages::*;

pub fn init_nexthops() -> Result<Vec<Message>, MessageHandleError> {
    init_bcast_nexthop(99)?;
    init_br_nexthop(1, 1)?;
    init_br_nexthop(2, 2)
}

pub fn init_br_nexthop(nh_id: i32, oif: i32) -> Result<Vec<Message>, MessageHandleError> {
    let mut nhr: NexthopRequest = NexthopRequest::default();
    nhr.op = SandeshOp::Add;
    nhr.id = nh_id;
    nhr.vrf = 1;
    nhr.family = AF_BRIDGE as i8;
    nhr._type = NhType::Encap;
    nhr.encap_oif_id = oif;
    nhr.flags =
        NH_FLAG_VALID | NH_FLAG_MAC_LEARN | NH_FLAG_ETREE_ROOT | NH_FLAG_UNKNOWN_UC_FLOOD;
    let request = Message::NexthopRequest(nhr);
    request.send_nl()
}

pub fn init_bcast_nexthop(nh_id: i32) -> Result<Vec<Message>, MessageHandleError> {
    let mut nhr: NexthopRequest = NexthopRequest::default();
    nhr.op = SandeshOp::Add;
    nhr.id = nh_id;
    nhr.vrf = 1;
    nhr.family = AF_UNSPEC as i8;
    nhr._type = NhType::Composite;
    nhr.nh_list = vec![1, 2];
    nhr.label_list = vec![-1, -1];
    nhr.flags = NH_FLAG_VALID
        | NH_FLAG_COMPOSITE_ENCAP
        | NH_FLAG_UNKNOWN_UC_FLOOD
        | NH_FLAG_MAC_LEARN;
    let request = Message::NexthopRequest(nhr);
    request.send_nl()
}
