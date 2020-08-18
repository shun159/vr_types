// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::vr_type::vr_messages::*;
use crate::vr_type::genetlink::MessageHandleError;

pub fn init_vrf() -> Result<Vec<Message>, MessageHandleError> {
    let mut vrf: VrfRequest = VrfRequest::default();
    vrf.op = SandeshOp::Add;
    vrf.idx = 1;
    vrf.flags = 1; // VRF_FLAG_VALID
    let request = Message::VrfRequest(vrf);
    request.send_nl()
}
