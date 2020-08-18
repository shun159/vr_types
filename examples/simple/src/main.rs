// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

extern crate nix;
extern crate vr_type;
extern crate mac_address;
extern crate eui48;
extern crate pnet;

pub mod nexthop;
pub mod vif;

pub use crate::vif::*;
pub use crate::nexthop::*;

fn main() {
    let res = init_nexthop();
    println!("nexthop result: {:#?}", res);
    // prepare interfaces
    let res = init_ifaces();
    println!("interface result: {:#?}", res);
    let res = get_iface(1);
    println!("interface result: {:#?}", res);
}

// private functions
