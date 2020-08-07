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

fn main() {
    // prepare interfaces
    let res = init_ifaces();
    println!("res: {:#?}", res)
}

// private functions
