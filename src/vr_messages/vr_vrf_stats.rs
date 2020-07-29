// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::error::CodecError;
use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_vrf_stats_req;
use std::convert::TryInto;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct VrfStatsRequest {
    pub op: SandeshOp,
    pub read_length: usize,
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

impl VrfStatsRequest {
    pub fn write(&self) -> Result<Vec<u8>, CodecError> {
        let mut encoder: vr_vrf_stats_req = vr_vrf_stats_req::new();
        encoder.h_op = self.op as u32;
        encoder.vsr_rid = self.rid;
        encoder.vsr_family = self.family;
        encoder.vsr_type = self._type;
        encoder.vsr_vrf = self.vrf;
        encoder.vsr_discards = self.discards;
        encoder.vsr_resolves = self.resolves;
        encoder.vsr_receives = self.receives;
        encoder.vsr_ecmp_composites = self.ecmp_composites;
        encoder.vsr_l2_mcast_composites = self.l2_mcast_composites;
        encoder.vsr_fabric_composites = self.fabric_composites;
        encoder.vsr_udp_tunnels = self.udp_tunnels;
        encoder.vsr_udp_mpls_tunnels = self.udp_mpls_tunnels;
        encoder.vsr_gre_mpls_tunnels = self.gre_mpls_tunnels;
        encoder.vsr_l2_encaps = self.l2_encaps;
        encoder.vsr_encaps = self.encaps;
        encoder.vsr_marker = self.marker;
        encoder.vsr_gros = self.gros;
        encoder.vsr_diags = self.diags;
        encoder.vsr_encap_composites = self.encap_composites;
        encoder.vsr_evpn_composites = self.evpn_composites;
        encoder.vsr_vrf_translates = self.vrf_translates;
        encoder.vsr_vxlan_tunnels = self.vxlan_tunnels;
        encoder.vsr_arp_virtual_proxy = self.arp_virtual_proxy;
        encoder.vsr_arp_virtual_stitch = self.arp_virtual_stitch;
        encoder.vsr_arp_virtual_flood = self.arp_virtual_flood;
        encoder.vsr_arp_physical_stitch = self.arp_physical_stitch;
        encoder.vsr_arp_tor_proxy = self.arp_tor_proxy;
        encoder.vsr_arp_physical_flood = self.arp_physical_flood;
        encoder.vsr_l2_receives = self.l2_receives;
        encoder.vsr_uuc_floods = self.uuc_floods;
        encoder.vsr_pbb_tunnels = self.pbb_tunnels;
        encoder.vsr_udp_mpls_over_mpls_tunnels = self.udp_mpls_over_mpls_tunnels;
        encoder.write()
    }

    pub fn read(buf: Vec<u8>) -> Result<VrfStatsRequest, CodecError> {
        let decoder: vr_vrf_stats_req = vr_vrf_stats_req::new();
        let rxfer = decoder.read(&buf)?;
        let mut vsr: VrfStatsRequest = VrfStatsRequest::default();
        vsr.read_length = rxfer as usize;
        vsr.op = decoder.h_op.try_into().unwrap();
        vsr.rid = decoder.vsr_rid;
        vsr.family = decoder.vsr_family;
        vsr._type = decoder.vsr_type;
        vsr.vrf = decoder.vsr_vrf;
        vsr.discards = decoder.vsr_discards;
        vsr.resolves = decoder.vsr_resolves;
        vsr.receives = decoder.vsr_receives;
        vsr.ecmp_composites = decoder.vsr_ecmp_composites;
        vsr.l2_mcast_composites = decoder.vsr_l2_mcast_composites;
        vsr.fabric_composites = decoder.vsr_fabric_composites;
        vsr.udp_tunnels = decoder.vsr_udp_tunnels;
        vsr.udp_mpls_tunnels = decoder.vsr_udp_mpls_tunnels;
        vsr.gre_mpls_tunnels = decoder.vsr_gre_mpls_tunnels;
        vsr.l2_encaps = decoder.vsr_l2_encaps;
        vsr.encaps = decoder.vsr_encaps;
        vsr.marker = decoder.vsr_marker;
        vsr.gros = decoder.vsr_gros;
        vsr.diags = decoder.vsr_diags;
        vsr.encap_composites = decoder.vsr_encap_composites;
        vsr.evpn_composites = decoder.vsr_evpn_composites;
        vsr.vrf_translates = decoder.vsr_vrf_translates;
        vsr.vxlan_tunnels = decoder.vsr_vxlan_tunnels;
        vsr.arp_virtual_proxy = decoder.vsr_arp_virtual_proxy;
        vsr.arp_virtual_stitch = decoder.vsr_arp_virtual_stitch;
        vsr.arp_virtual_flood = decoder.vsr_arp_virtual_flood;
        vsr.arp_physical_stitch = decoder.vsr_arp_physical_stitch;
        vsr.arp_tor_proxy = decoder.vsr_arp_tor_proxy;
        vsr.arp_physical_flood = decoder.vsr_arp_physical_flood;
        vsr.l2_receives = decoder.vsr_l2_receives;
        vsr.uuc_floods = decoder.vsr_uuc_floods;
        vsr.pbb_tunnels = decoder.vsr_pbb_tunnels;
        vsr.udp_mpls_over_mpls_tunnels = decoder.vsr_udp_mpls_over_mpls_tunnels;
        Ok(vsr)
    }
}
