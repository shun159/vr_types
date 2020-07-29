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

pub const NH_DEAD: i8 = 0;
pub const NH_RCV: i8 = 1;
pub const NH_ENCAP: i8 = 2;
pub const NH_TUNNEL: i8 = 3;
pub const NH_RESOLVE: i8 = 4;
pub const NH_DISCARD: i8 = 5;
pub const NH_COMPOSITE: i8 = 6;
pub const NH_VRF_TRANSLATE: i8 = 7;
pub const NH_L2_RCV: i8 = 8;
pub const NH_MAX: i8 = 9;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NhType {
    Dead = NH_DEAD as isize,
    Rcv = NH_RCV as isize,
    Encap = NH_ENCAP as isize,
    Tunnel = NH_TUNNEL as isize,
    Resolve = NH_RESOLVE as isize,
    Discard = NH_DISCARD as isize,
    Composite = NH_COMPOSITE as isize,
    VrfTranslate = NH_VRF_TRANSLATE as isize,
    L2Rcv = NH_L2_RCV as isize,
    Max = NH_MAX as isize,
}

impl TryFrom<i8> for NhType {
    type Error = ();
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        match v {
            NH_DEAD => Ok(NhType::Dead),
            NH_RCV => Ok(NhType::Rcv),
            NH_ENCAP => Ok(NhType::Encap),
            NH_TUNNEL => Ok(NhType::Tunnel),
            NH_RESOLVE => Ok(NhType::Resolve),
            NH_DISCARD => Ok(NhType::Discard),
            NH_COMPOSITE => Ok(NhType::Composite),
            NH_VRF_TRANSLATE => Ok(NhType::VrfTranslate),
            NH_L2_RCV => Ok(NhType::L2Rcv),
            NH_MAX => Ok(NhType::Max),
            _ => Err(()),
        }
    }
}

pub const NH_ECMP_CONFIG_HASH_BITS: i8 = 5;
pub const NH_ECMP_CONFIG_HASH_MASK: i8 = (1 << NH_ECMP_CONFIG_HASH_BITS) - 1;
pub const NH_ECMP_CONFIG_HASH_PROTO: i8 = 0x01;
pub const NH_ECMP_CONFIG_HASH_SRC_IP: i8 = 0x02;
pub const NH_ECMP_CONFIG_HASH_SRC_PORT: i8 = 0x04;
pub const NH_ECMP_CONFIG_HASH_DST_IP: i8 = 0x08;
pub const NH_ECMP_CONFIG_HASH_DST_PORT: i8 = 0x10;

pub const NH_FLAG_VALID: u32 = 0x000001;
pub const NH_FLAG_POLICY_ENABLED: u32 = 0x000002;
/*: u32 = 0x000004 is free */
pub const NH_FLAG_TUNNEL_GRE: u32 = 0x000008;
pub const NH_FLAG_TUNNEL_UDP: u32 = 0x000010;
/*
 * Mcast flag can be appended to any type of nexthop, either an Encap,
 * composite etc
 */
pub const NH_FLAG_MCAST: u32 = 0x000020;
pub const NH_FLAG_TUNNEL_UDP_MPLS: u32 = 0x000040;
pub const NH_FLAG_TUNNEL_VXLAN: u32 = 0x000080;
pub const NH_FLAG_RELAXED_POLICY: u32 = 0x000100;
pub const NH_FLAG_COMPOSITE_FABRIC: u32 = 0x000200;
pub const NH_FLAG_COMPOSITE_ECMP: u32 = 0x000400;
pub const NH_FLAG_COMPOSITE_LU_ECMP: u32 = 0x000800;
pub const NH_FLAG_COMPOSITE_EVPN: u32 = 0x001000;
pub const NH_FLAG_COMPOSITE_ENCAP: u32 = 0x002000;
pub const NH_FLAG_COMPOSITE_TOR: u32 = 0x004000;
pub const NH_FLAG_ROUTE_LOOKUP: u32 = 0x008000;
pub const NH_FLAG_UNKNOWN_UC_FLOOD: u32 = 0x010000;
pub const NH_FLAG_TUNNEL_SIP_COPY: u32 = 0x020000;
pub const NH_FLAG_FLOW_LOOKUP: u32 = 0x040000;
pub const NH_FLAG_TUNNEL_PBB: u32 = 0x080000;
pub const NH_FLAG_MAC_LEARN: u32 = 0x100000;
pub const NH_FLAG_ETREE_ROOT: u32 = 0x200000;
pub const NH_FLAG_INDIRECT: u32 = 0x400000;
pub const NH_FLAG_L2_CONTROL_DATA: u32 = 0x800000;
pub const NH_FLAG_CRYPT_TRAFFIC: u32 = 0x01000000;
pub const NH_FLAG_L3_VXLAN: u32 = 0x02000000;
pub const NH_FLAG_TUNNEL_MPLS_O_MPLS: u32 = 0x04000000;
pub const NH_FLAG_VALIDATE_MCAST_SRC: u32 = 0x08000000;

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
        encoder.write()
    }

    pub fn read(buf: Vec<u8>) -> Result<NexthopRequest, CodecError> {
        let decoder = vr_nexthop_req::new();
        let rxfer = decoder.read(&buf)?;
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
        nhr.encap = utils::free_buf::<i8>(decoder.nhr_encap as *mut i8, nhr.encap_len);
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
        nhr.tun_sip6 =
            Self::read_tun_ip6(decoder.nhr_tun_sip6, decoder.nhr_tun_sip6_size);

        nhr.tun_dip6 =
            Self::read_tun_ip6(decoder.nhr_tun_dip6, decoder.nhr_tun_dip6_size);

        nhr.ecmp_config_hash = decoder.nhr_ecmp_config_hash;

        // MAC Address
        nhr.pbb_mac = utils::read_mac_addr(decoder.nhr_pbb_mac, decoder.nhr_pbb_mac_size);

        nhr.encap_crypt_oif_id = decoder.nhr_encap_crypt_oif_id;
        nhr.crypt_traffic = decoder.nhr_crypt_traffic;
        nhr.crypt_path_available = decoder.nhr_crypt_path_available;
        nhr.rw_dst_mac =
            utils::read_mac_addr(decoder.nhr_rw_dst_mac, decoder.nhr_rw_dst_mac_size);
        nhr.transport_label = decoder.nhr_transport_label;

        Ok(nhr)
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
            let ip6_v: Vec<i8> = utils::free_buf(tun_ip6, VR_IP6_ADDRESS_LEN as usize);
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
