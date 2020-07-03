// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::vr_vxlan::VxlanRequest;
use libc::NLMSG_MIN_TYPE;
use netlink_packet_core::{
    NetlinkDeserializable, NetlinkHeader, NetlinkMessage, NetlinkPayload,
    NetlinkSerializable,
};
use netlink_sys::{Protocol, Socket, SocketAddr};
use std::convert::From;

pub const GROUP_ID: u32 = 0x4;
pub const GENL_ID_VROUTER: i32 = (NLMSG_MIN_TYPE + 0x10);
pub const FAMILY_NAME: &str = "vrouter";
pub const MCAST_GROUP: &str = "VRouterGroup";

pub enum VnswNlAttrs {
    Unspec = 0,
    VrMessageProtocol = 1,
    Max = 3,
}

pub struct NlClient {
    pub sockaddr: SocketAddr,
    pub sock: Socket,
}

impl NlClient {
    pub fn connect() -> Result<Self, ()> {
        let sockaddr: SocketAddr = SocketAddr::new(0, 0);
        //let vxlanr: VxlanRequest = VxlanRequest::default();
        //let packet = NetlinkMessage::from(vxlanr.write().unwrap());
        match Socket::new(Protocol::Generic) {
            Ok(sock) => Ok(NlClient {
                sockaddr: sockaddr,
                sock: sock,
            }),
            Err(_) => Err(()),
        }
    }
}
