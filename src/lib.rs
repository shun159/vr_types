// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

extern crate libc;

pub mod genetlink;
pub mod sandesh;
pub mod utils;
pub mod vr_drop_stats;
pub mod vr_fc_map;
pub mod vr_flow;
pub mod vr_interface;
pub mod vr_mem_stats;
pub mod vr_mirror;
pub mod vr_mpls;
pub mod vr_nexthop;
pub mod vr_pkt_droplog;
pub mod vr_qos_map;
pub mod vr_response;
pub mod vr_route;
pub mod vr_types;
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(improper_ctypes)]
pub mod vr_types_binding;
pub mod vr_vrf;
pub mod vr_vrf_assign;
pub mod vr_vrf_stats;
pub mod vr_vxlan;
pub mod vrouter_ops;
