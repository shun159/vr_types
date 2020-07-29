// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::error::CodecError;
use super::sandesh::SandeshOp;
use super::vr_flow::VR_IP6_ADDRESS_LEN;
use super::vr_types::VrSandesh;
use super::vr_types_binding::*;
use crate::utils;
use eui48::MacAddress;
use std::convert::{From, TryFrom, TryInto};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

impl TryFrom<i8> for NhType {
    type Error = ();
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        match v {
            x if x == NhType::Dead as i8 => Ok(NhType::Dead),
            x if x == NhType::Rcv as i8 => Ok(NhType::Rcv),
            x if x == NhType::Encap as i8 => Ok(NhType::Encap),
            x if x == NhType::Tunnel as i8 => Ok(NhType::Tunnel),
            x if x == NhType::Resolve as i8 => Ok(NhType::Resolve),
            x if x == NhType::Discard as i8 => Ok(NhType::Discard),
            x if x == NhType::Composite as i8 => Ok(NhType::Composite),
            x if x == NhType::VrfTranslate as i8 => Ok(NhType::VrfTranslate),
            x if x == NhType::L2Rcv as i8 => Ok(NhType::L2Rcv),
            x if x == NhType::Max as i8 => Ok(NhType::Max),
            _ => Err(()),
        }
    }
}

// Defined in vr_nexthop.h
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NhEcmpConfigHash {
    Proto = 0x01,
    SrcIP = 0x02,
    SrcPort = 0x04,
    DstIP = 0x08,
    DstPort = 0x10,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NexthopRequest {
    pub op: SandeshOp,
    pub read_length: usize,
    pub _type: NhType,
    pub family: i8, // One of AF_*
    pub id: i32,
    pub rid: i32,
    pub encap_oif_id: i32,
    pub encap_len: usize,
    pub encap_family: i32,
    pub vrf: i32,
    pub tun_sip: Ipv4Addr,
    pub tun_dip: Ipv4Addr,
    pub tun_sport: i16,
    pub tun_dport: i16,
    pub ref_cnt: i32,
    pub marker: i32,
    pub flags: u32,
    pub encap: Vec<i8>,
    pub nh_list: Vec<i32>,
    pub label_list: Vec<i32>,
    pub nh_count: i16,
    pub tun_sip6: Ipv6Addr,
    pub tun_dip6: Ipv6Addr,
    pub ecmp_config_hash: i8,
    pub pbb_mac: MacAddress,
    pub encap_crypt_oif_id: i32,
    pub crypt_traffic: i32,
    pub crypt_path_available: i32,
    pub rw_dst_mac: MacAddress,
    pub transport_label: u32,
}

impl Default for NexthopRequest {
    fn default() -> NexthopRequest {
        NexthopRequest {
            op: SandeshOp::Add,
            read_length: 0,
            _type: NhType::Dead,
            family: 0,
            id: 0,
            rid: 0,
            encap_oif_id: 0,
            encap_len: 0,
            encap_family: 0,
            vrf: 0,
            tun_sip: Ipv4Addr::new(0, 0, 0, 0),
            tun_dip: Ipv4Addr::new(0, 0, 0, 0),
            tun_sport: 0,
            tun_dport: 0,
            ref_cnt: 0,
            marker: 0,
            flags: 0,
            encap: vec![],
            nh_list: vec![],
            label_list: vec![],
            nh_count: 0,
            tun_sip6: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
            tun_dip6: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
            ecmp_config_hash: 0,
            pbb_mac: MacAddress::nil(),
            encap_crypt_oif_id: 0,
            crypt_traffic: 0,
            crypt_path_available: 0,
            rw_dst_mac: MacAddress::nil(),
            transport_label: 0,
        }
    }
}

impl NexthopRequest {
    pub fn write(&self) -> Result<Vec<u8>, CodecError> {
        let mut encoder: vr_nexthop_req = vr_nexthop_req::new();
        encoder.h_op = self.op as u32;
        encoder.nhr_type = self._type as i8;
        encoder.nhr_family = self.family;
        encoder.nhr_id = self.id;
        encoder.nhr_rid = self.rid;
        encoder.nhr_encap_oif_id = self.encap_crypt_oif_id;
        encoder.nhr_encap_len = self.encap.len() as i32;
        encoder.nhr_encap = utils::into_mut_ptr::<i8>(&self.encap);
        encoder.nhr_encap_size = self.encap.len() as u32;
        encoder.nhr_encap_family = self.encap_family;
        encoder.nhr_vrf = self.vrf;
        encoder.nhr_tun_sip = Self::in_addr_to_u32(self.tun_sip);
        encoder.nhr_tun_dip = Self::in_addr_to_u32(self.tun_dip);
        encoder.nhr_tun_sport = self.tun_sport;
        encoder.nhr_tun_dport = self.tun_dport;
        encoder.nhr_ref_cnt = self.ref_cnt;
        encoder.nhr_marker = self.marker;
        encoder.nhr_flags = self.flags;
        encoder.nhr_nh_list = utils::into_mut_ptr::<i32>(&self.nh_list);
        encoder.nhr_nh_list_size = self.nh_list.len() as u32;
        encoder.nhr_label_list = utils::into_mut_ptr::<i32>(&self.label_list);
        encoder.nhr_label_list_size = self.label_list.len() as u32;
        encoder.nhr_nh_count = self.nh_count;
        encoder.nhr_tun_sip6 = Self::in6_addr_to_vec(self.tun_sip6);
        encoder.nhr_tun_sip6_size = if self.tun_sip6.is_unspecified() {
            0u32
        } else {
            VR_IP6_ADDRESS_LEN
        };
        encoder.nhr_tun_dip6 = Self::in6_addr_to_vec(self.tun_dip6);
        encoder.nhr_tun_dip6_size = if self.tun_dip6.is_unspecified() {
            0u32
        } else {
            VR_IP6_ADDRESS_LEN
        };
        encoder.nhr_ecmp_config_hash = self.ecmp_config_hash;
        encoder.nhr_pbb_mac = Self::mac_to_vec(self.pbb_mac);
        encoder.nhr_pbb_mac_size = if self.pbb_mac.is_nil() {
            0u32
        } else {
            libc::ETH_ALEN as u32
        };
        encoder.nhr_encap_crypt_oif_id = self.encap_crypt_oif_id;
        encoder.nhr_crypt_traffic = self.crypt_traffic;
        encoder.nhr_crypt_path_available = self.crypt_path_available;
        encoder.nhr_rw_dst_mac = Self::mac_to_vec(self.rw_dst_mac);
        encoder.nhr_rw_dst_mac_size = if self.rw_dst_mac.is_nil() {
            0u32
        } else {
            libc::ETH_ALEN as u32
        };
        encoder.nhr_transport_label = self.transport_label;

        match encoder.write() {
            Err(e) => Err(e),
            Ok(v) => Ok(v),
        }
    }

    pub fn read(buf: Vec<u8>) -> Result<NexthopRequest, CodecError> {
        let decoder = vr_nexthop_req::new();
        match decoder.read(&buf) {
            Err(e) => Err(e),
            Ok(rxfer) => {
                let mut nhr = NexthopRequest::default();
                nhr.read_length = rxfer as usize;
                nhr.op = decoder.h_op.try_into().unwrap();
                nhr._type = decoder.nhr_type.try_into().unwrap();
                nhr.family = decoder.nhr_family;
                nhr.id = decoder.nhr_id;
                nhr.rid = decoder.nhr_rid;
                nhr.encap_oif_id = decoder.nhr_encap_oif_id;
                nhr.encap_len = decoder.nhr_encap_len as usize;
                nhr.encap_family = decoder.nhr_encap_family;
                nhr.vrf = decoder.nhr_vrf;
                nhr.tun_sip = Ipv4Addr::from(decoder.nhr_tun_sip);
                nhr.tun_dip = Ipv4Addr::from(decoder.nhr_tun_dip);
                nhr.tun_sport = decoder.nhr_tun_sport;
                nhr.tun_dport = decoder.nhr_tun_dport;
                nhr.ref_cnt = decoder.nhr_ref_cnt;
                nhr.marker = decoder.nhr_marker;
                nhr.flags = decoder.nhr_flags;
                nhr.encap = utils::free_buf::<i8>(
                    decoder.nhr_encap as *mut i8,
                    nhr.encap_len,
                );
                nhr.nh_list = utils::free_buf::<i32>(
                    decoder.nhr_nh_list as *mut i32,
                    decoder.nhr_nh_list_size as usize,
                );
                nhr.nh_count = decoder.nhr_nh_count;
                nhr.label_list = utils::free_buf::<i32>(
                    decoder.nhr_label_list as *mut i32,
                    decoder.nhr_label_list_size as usize,
                );

                // Decode tunnel in6addr
                nhr.tun_sip6 = Self::read_tun_ip6(
                    decoder.nhr_tun_sip6,
                    decoder.nhr_tun_sip6_size,
                );

                nhr.tun_dip6 = Self::read_tun_ip6(
                    decoder.nhr_tun_dip6,
                    decoder.nhr_tun_dip6_size,
                );

                nhr.ecmp_config_hash = decoder.nhr_ecmp_config_hash;

                // MAC Address
                nhr.pbb_mac = utils::read_mac_addr(
                    decoder.nhr_pbb_mac,
                    decoder.nhr_pbb_mac_size,
                );

                nhr.encap_crypt_oif_id = decoder.nhr_encap_crypt_oif_id;
                nhr.crypt_traffic = decoder.nhr_crypt_traffic;
                nhr.crypt_path_available = decoder.nhr_crypt_path_available;
                nhr.rw_dst_mac = utils::read_mac_addr(
                    decoder.nhr_rw_dst_mac,
                    decoder.nhr_rw_dst_mac_size,
                );
                nhr.transport_label = decoder.nhr_transport_label;

                Ok(nhr)
            }
        }
    }

    fn mac_to_vec(addr: MacAddress) -> *mut i8 {
        let octets = if addr.is_nil() {
            vec![]
        } else {
            addr.as_bytes().to_vec()
        };

        utils::into_mut_ptr(&octets) as *mut i8
    }

    fn in_addr_to_u32(addr: Ipv4Addr) -> u32 {
        let v = addr.octets().to_vec();
        ((v[0] as u32) << 24)
            | ((v[1] as u32) << 16)
            | ((v[2] as u32) << 8)
            | (v[3] as u32)
    }

    fn in6_addr_to_vec(addr: Ipv6Addr) -> *mut i8 {
        let octets = if addr.is_unspecified() {
            vec![]
        } else {
            let v: Vec<i8> = Vec::new();
            addr.octets().iter().fold(v, |mut acc, &o| {
                acc.push(o as i8);
                acc
            })
        };

        utils::into_mut_ptr(&octets)
    }

    fn read_tun_ip6(tun_ip6: *mut i8, ip6_size: u32) -> Ipv6Addr {
        if ip6_size == VR_IP6_ADDRESS_LEN {
            let ip6_v: Vec<i8> =
                utils::free_buf(tun_ip6, VR_IP6_ADDRESS_LEN as usize);
            Ipv6Addr::from(
                ((ip6_v[0] as u128) << 120)
                    | ((ip6_v[1] as u128) << 112)
                    | ((ip6_v[2] as u128) << 104)
                    | ((ip6_v[3] as u128) << 96)
                    | ((ip6_v[4] as u128) << 88)
                    | ((ip6_v[5] as u128) << 80)
                    | ((ip6_v[6] as u128) << 72)
                    | ((ip6_v[7] as u128) << 64)
                    | ((ip6_v[8] as u128) << 56)
                    | ((ip6_v[9] as u128) << 48)
                    | ((ip6_v[10] as u128) << 40)
                    | ((ip6_v[11] as u128) << 32)
                    | ((ip6_v[12] as u128) << 24)
                    | ((ip6_v[13] as u128) << 16)
                    | ((ip6_v[14] as u128) << 8)
                    | (ip6_v[15] as u128),
            )
        } else {
            Ipv6Addr::UNSPECIFIED
        }
    }
}