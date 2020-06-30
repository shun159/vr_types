// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_vrf_stats_req;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
pub struct VrfStatsRequest {
    pub op: SandeshOp,
    pub rid: i16,
    pub family: i16,
    pub _type: i16,
    pub vrf: i32,
    pub discards: i64,
    pub resolves: i64,
    pub receives: i64,
    pub ecmp_composites: i64,
    pub l2_mcast_composites: i64,
    pub fabric_composites: i64,
    pub udp_tunnels: i64,
    pub udp_mpls_tunnels: i64,
    pub gre_mpls_tunnels: i64,
    pub l2_encaps: i64,
    pub encaps: i64,
    pub marker: i16,
    pub gros: i64,
    pub diags: i64,
    pub encap_composites: i64,
    pub evpn_composites: i64,
    pub vrf_translates: i64,
    pub vxlan_tunnels: i64,
    pub arp_virtual_proxy: i64,
    pub arp_virtual_stitch: i64,
    pub arp_virtual_flood: i64,
    pub arp_physical_stitch: i64,
    pub arp_tor_proxy: i64,
    pub arp_physical_flood: i64,
    pub l2_receives: i64,
    pub uuc_floods: i64,
    pub pbb_tunnels: i64,
    pub udp_mpls_over_mpls_tunnels: i64,
}

impl Default for VrfStatsRequest {
    fn default() -> VrfStatsRequest {
        VrfStatsRequest {
            op: SandeshOp::Add,
            rid: 0,
            family: 0,
            _type: 0,
            vrf: 0,
            discards: 0,
            resolves: 0,
            receives: 0,
            ecmp_composites: 0,
            l2_mcast_composites: 0,
            fabric_composites: 0,
            udp_tunnels: 0,
            udp_mpls_tunnels: 0,
            gre_mpls_tunnels: 0,
            l2_encaps: 0,
            encaps: 0,
            marker: 0,
            gros: 0,
            diags: 0,
            encap_composites: 0,
            evpn_composites: 0,
            vrf_translates: 0,
            vxlan_tunnels: 0,
            arp_virtual_proxy: 0,
            arp_virtual_stitch: 0,
            arp_virtual_flood: 0,
            arp_physical_stitch: 0,
            arp_tor_proxy: 0,
            arp_physical_flood: 0,
            l2_receives: 0,
            uuc_floods: 0,
            pbb_tunnels: 0,
            udp_mpls_over_mpls_tunnels: 0,
        }
    }
}
