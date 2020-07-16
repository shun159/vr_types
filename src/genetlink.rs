// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use libc::NLMSG_MIN_TYPE;
use netlink_sys::{Protocol, Socket, SocketAddr};
use std::convert::From;
use std::process;

pub const GROUP_ID: u32 = 0x4;
pub const GENL_ID_VROUTER: i32 = (NLMSG_MIN_TYPE + 0x10);
pub const FAMILY_NAME: &str = "vrouter";
pub const MCAST_GROUP: &str = "VRouterGroup";

pub struct NlClient {
    pub sockaddr: SocketAddr,
    pub sock: Socket,
}

impl NlClient {
}
