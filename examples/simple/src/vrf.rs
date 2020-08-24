// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::vr_type::genetlink::MessageHandleError;
use crate::vr_type::vr_messages::*;

pub fn init_vrf() -> Result<Vec<Message>, MessageHandleError> {
    let mut vrf: VrfRequest = VrfRequest::default();
    vrf.op = SandeshOp::Add;
    vrf.idx = 1;
    vrf.flags = 1; // VRF_FLAG_VALID
    let request = Message::VrfRequest(vrf);
    request.send_nl()
}

pub fn assign_vrf(vif_idx: i16, nh_id: i32) -> Result<Vec<Message>, MessageHandleError> {
    let mut var: VrfAssignRequest = VrfAssignRequest::default();
    var.op = SandeshOp::Add;
    var.vif_vrf = 1;
    var.vif_index = vif_idx;
    var.nh_id = nh_id;
    let request = Message::VrfAssignRequest(var);
    request.send_nl()
}
