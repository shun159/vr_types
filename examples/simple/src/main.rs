// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

extern crate nix;
extern crate vr_type;
extern crate mac_address;
extern crate eui48;
extern crate pnet;

pub mod nexthop;
pub mod vif;
pub mod vrf;
pub mod route;
pub mod flow;

pub use crate::vif::*;
pub use crate::vrf::*;
pub use crate::nexthop::*;
pub use crate::route::*;
pub use crate::flow::*;

fn main() {
    let res = init_nexthop();
    println!("nexthop result: {:#?}", res);
    let res = init_vrf();
    println!("VRF result: {:#?}", res);
    //let res = init_bridge(0, &[0x32, 0x50, 0x1c, 0xc1, 0x03, 0x8d]);
    //println!("Bridge result: {:#?}", res);
    //let res = init_bridge(1, &[0xc2, 0xd3, 0x91, 0xa2, 0xbb, 0xf4]);
    //println!("Bridge result: {:#?}", res);
    // prepare interfaces
    let res = init_ifaces();
    println!("interface result: {:#?}", res);
    let res = get_iface(1);
    println!("interface result: {:#?}", res);
}

// private functions
