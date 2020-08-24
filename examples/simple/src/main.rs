// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

extern crate eui48;
extern crate mac_address;
extern crate nix;
extern crate pnet;
extern crate vr_type;

pub mod nexthop;
pub mod vif;
pub mod vrf;

pub use crate::nexthop::*;
pub use crate::vif::*;
pub use crate::vrf::*;

fn main() {
    let _res = init_vrf();
    let _res = init_ifaces();
    let _res = init_nexthops();
    let _res = assign_vrf(1, 1);
    let _res = assign_vrf(1, 2);
    let _res = assign_vrf(1, 99);

    //let res = init_bcast_bridge(3, 99);
    //println!("Bridge result: {:#?}", res);
}

// private functions
