// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use crate::sandesh;
use eui48::MacAddress;
use libc;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::io::Cursor;

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
    Max,
}

// Defined in vr_nexthop.h
#[derive(Debug, PartialEq)]
pub enum NhEcmpConfigHash {
    Proto = 0b0000_0001,
    SrcIP = 0b0000_0010,
    SrcPort = 0b0000_0100,
    DstIP = 0b0000_1000,
    DstPort = 0b0001_0000,
}

#[derive(Debug, PartialEq)]
pub enum NhFlag {
    Valid = 0x00000001,
    PolicyEnabled = 0x00000002,
    TunnelGre = 0x00000008,
    TunnelUdp = 0x00000010,
    Mcast = 0x00000020,
    TunnelUdpMpls = 0x00000040,
    TunnelVxlan = 0x00000080,
    RelaxedPolicy = 0x00000100,
    CompositeFabric = 0x00000200,
    CompositeEcmp = 0x00000400,
    CompositeLuEcmp = 0x00000800,
    CompositeEvpn = 0x00001000,
    CompositeEncap = 0x00002000,
    CompositeTor = 0x00004000,
    RouteLookUP = 0x00008000,
    UnknownUcFlood = 0x00010000,
    TunnelSipCopy = 0x00020000,
    FlowLookup = 0x00040000,
    TunnelPbb = 0x00080000,
    MacLearn = 0x00100000,
    EtreeRoot = 0x00200000,
    Indirect = 0x00400000,
    L2ControlData = 0x00800000,
    CryptTraffic = 0x01000000,
    L3Vxlan = 0x02000000,
    TunnelMplsOMpls = 0x04000000,
    ValidateMcastSrc = 0x08000000,
}

#[derive(Debug, PartialEq)]
pub struct VlanHeader {
    pub tpid: u16,
    pub pcp:  u8,
    pub vlan_id: u16
}

impl VlanHeader {
    pub fn new() -> VlanHeader {
        VlanHeader { tpid: 0, pcp: 0, vlan_id: 0 }
    }

    pub fn write(&self) -> Vec<u8> {
        let buf: Vec<u8> = Vec::new();
        let mut c = Cursor::new(buf);
        c.write_u16::<NetworkEndian>(self.tpid).unwrap();
        c.write_u16::<NetworkEndian>(
            (self.pcp as u16) << 13 |
            self.vlan_id & 0x1fff
        ).unwrap();
        c.into_inner()
    }

    pub fn read<'a>(buf: &'a Vec<u8>) -> Result<VlanHeader, &str> {
        if buf.len() < 4 { return Err("Byte too short(< 32bit)") }
        let mut c = Cursor::new(&buf);
        let tpid = c.read_u16::<NetworkEndian>().unwrap();
        let tci = c.read_u16::<NetworkEndian>().unwrap();
        let vlan_h = VlanHeader {
            tpid: tpid,
            pcp: (tci >> 13) as u8,
            vlan_id: tci & 0x1fff
        };

        Ok(vlan_h)
    }
}

#[derive(Debug, PartialEq)]
pub struct NhEncap {
    pub dst_mac: MacAddress,
    pub src_mac: MacAddress,
    pub vlan_h: VlanHeader,
    pub eth_type: i32,
}

impl NhEncap {
    pub fn new() -> NhEncap {
        NhEncap {
            dst_mac: MacAddress::nil(),
            src_mac: MacAddress::nil(),
            vlan_h: VlanHeader::new(),
            eth_type: libc::ETH_P_IP
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct NhRequest {
    pub op: sandesh::Op,
    pub _type: NhType,
    pub family: i8, // One of AF_*
    pub id: i32,
    pub rid: i32,
    pub encap_oif_id: i32,
    pub encap_len: i32,
    pub encap_family: i32,
    pub vrf: i32,
    pub tun_sip: Ipv4Addr,
    pub tun_dip: Ipv4Addr,
    pub tun_sport: i16,
    pub tun_dport: i16,
    pub ref_cnt: i32,
    pub marker: i32,
    pub flags: Vec<NhFlag>,
    pub encap: Option<NhEncap>,
    pub nh_list: Vec<i32>,
    pub label_list: Vec<i32>,
    pub nh_count: i16,
    pub tun_sip6: Ipv6Addr,
    pub tun_dip6: Ipv6Addr,
    pub ecmp_config_hash: Vec<NhEcmpConfigHash>,
    pub pbb_mac: MacAddress,
    pub encap_crypt_oif_id: i32,
    pub crypt_traffic: i32,
    pub crypt_path_available: i32,
    pub rw_dst_mac: MacAddress,
    pub transport_label: u32,
}
