// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::vr_type::vr_messages::*;
use crate::vr_type::genetlink::MessageHandleError;
use crate::eui48::MacAddress;
use crate::nix::libc::AF_BRIDGE;

pub fn init_bridge(idx: i32, mac: &[u8]) -> Result<Vec<Message>, MessageHandleError> {
    let mut rtr: RouteRequest = RouteRequest::default();
    rtr.op = SandeshOp::Add;
    rtr.index = idx;
    rtr.mac = MacAddress::from_bytes(mac).unwrap();
    rtr.vrf_id = 1;
    rtr.nh_id = 1;
    rtr.label_flags = 1 | 8;
    rtr.family = AF_BRIDGE;
    let request = Message::RouteRequest(rtr);
    request.send_nl()
}
