// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh;

#[derive(Debug, PartialEq)]
pub enum NhType {
    Dead,
    Rcv,
    Encap,
    Tunnel,
    Resolve,
    Discard,
    Composite,
    VrfTranslate,
    L2Rcv,
    Max
}

// Defined in vr_nexthop.h
#[derive(Debug, PartialEq)]
pub enum EcmpConfigHash {
    Proto   = 0b0000_0001,
    SrcIP   = 0b0000_0010,
    SrcPort = 0b0000_0100,
    DstIP   = 0b0000_1000,
    DstPort = 0b0001_0000
}

#[derive(Debug, PartialEq)]
pub enum Flag {
    Valid            = 0x00000001,
    PolicyEnabled    = 0x00000002,
    TunnelGre        = 0x00000008,
    TunnelUdp        = 0x00000010,
    Mcast            = 0x00000020,
    TunnelUdpMpls    = 0x00000040,
    TunnelVxlan      = 0x00000080,
    RelaxedPolicy    = 0x00000100,
    CompositeFabric  = 0x00000200,
    CompositeEcmp    = 0x00000400,
    CompositeLuEcmp  = 0x00000800,
    CompositeEvpn    = 0x00001000,
    CompositeEncap   = 0x00002000,
    CompositeTor     = 0x00004000,
    RouteLookUP      = 0x00008000,
    UnknownUcFlood   = 0x00010000,
    TunnelSipCopy    = 0x00020000,
    FlowLookup       = 0x00040000,
    TunnelPbb        = 0x00080000,
    MacLearn         = 0x00100000,
    EtreeRoot        = 0x00200000,
    Indirect         = 0x00400000,
    L2ControlData    = 0x00800000,
    CryptTraffic     = 0x01000000,
    L3Vxlan          = 0x02000000,
    TunnelMplsOMpls  = 0x04000000,
    ValidateMcastSrc = 0x08000000,
}

#[derive(Debug, PartialEq)]
pub struct Request {
    pub op: sandesh::Op,
    pub _type: NhType,
    pub family: i8, // One of AF_*
    pub id: i32,
    pub rid: i32,
    pub encap_oif_id: i32,
    pub encap_len: i32,
    pub encap_family: i32,
    pub vrf: i32,
    pub tun_sip: u32,
    pub tun_dip: u32,
    pub tun_sport: i16,
    pub tun_dport: i16,
    pub ref_cnt: i32,
    pub marker: i32,
    pub flags: u32,
    pub encap: *mut i8,
    pub encap_size: u32,
    pub nh_list: *mut i32,
    pub nh_list_size: u32,
    pub label_list: *mut i32,
    pub label_list_size: u32,
    pub nh_count: i16,
    pub tun_sip6: *mut i8,
    pub tun_sip6_size: u32,
    pub tun_dip6: *mut i8,
    pub tun_dip6_size: u32,
    pub ecmp_config_hash: Vec<EcmpConfigHash>,
    pub pbb_mac: *mut i8,
    pub pbb_mac_size: u32,
    pub encap_crypt_oif_id: i32,
    pub crypt_traffic: i32,
    pub crypt_path_available: i32,
    pub rw_dst_mac: *mut i8,
    pub rw_dst_mac_size: u32,
    pub transport_label: u32,
}
