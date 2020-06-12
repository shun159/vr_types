// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::utils;
use crate::vr_flow::VR_IP6_ADDRESS_LEN;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::*;
use eui48::MacAddress;
use std::convert::{From, TryFrom, TryInto};
use std::net::{Ipv4Addr, Ipv6Addr};

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
#[derive(Debug, PartialEq)]
pub enum NhEcmpConfigHash {
    Proto = 0x01,
    SrcIP = 0x02,
    SrcPort = 0x04,
    DstIP = 0x08,
    DstPort = 0x10,
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
pub struct NhRequest {
    pub op: SandeshOp,
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
    pub encap: Vec<u8>,
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

impl Default for NhRequest {
    fn default() -> NhRequest {
        NhRequest {
            op: SandeshOp::Add,
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

impl NhRequest {
    pub fn read<'a>(buf: Vec<u8>) -> Result<NhRequest, &'a str> {
        let decoder = vr_nexthop_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut nhr = NhRequest::default();
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
                nhr.encap = utils::free_buf::<u8>(
                    decoder.nhr_encap as *mut u8,
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
                let tun_sip6 = utils::free_buf::<u16>(
                    decoder.nhr_tun_sip6 as *mut u16,
                    (VR_IP6_ADDRESS_LEN / 2) as usize,
                );
                nhr.tun_sip6 = Ipv6Addr::new(
                    tun_sip6[0],
                    tun_sip6[1],
                    tun_sip6[2],
                    tun_sip6[3],
                    tun_sip6[4],
                    tun_sip6[5],
                    tun_sip6[6],
                    tun_sip6[7],
                );

                let tun_dip6 = utils::free_buf::<u16>(
                    decoder.nhr_tun_dip6 as *mut u16,
                    (VR_IP6_ADDRESS_LEN / 2) as usize,
                );
                nhr.tun_dip6 = Ipv6Addr::new(
                    tun_dip6[0],
                    tun_dip6[1],
                    tun_dip6[2],
                    tun_dip6[3],
                    tun_dip6[4],
                    tun_dip6[5],
                    tun_dip6[6],
                    tun_dip6[7],
                );

                nhr.ecmp_config_hash = decoder.nhr_ecmp_config_hash;

                // MAC Address
                nhr.pbb_mac = MacAddress::from_bytes(&*utils::free_buf::<u8>(
                    decoder.nhr_pbb_mac as *mut u8,
                    libc::ETH_ALEN as usize,
                ))
                .unwrap();

                nhr.encap_crypt_oif_id = decoder.nhr_encap_crypt_oif_id;
                nhr.crypt_traffic = decoder.nhr_crypt_traffic;
                nhr.crypt_path_available = decoder.nhr_crypt_path_available;
                nhr.rw_dst_mac =
                    MacAddress::from_bytes(&*utils::free_buf::<u8>(
                        decoder.nhr_rw_dst_mac as *mut u8,
                        libc::ETH_ALEN as usize,
                    ))
                    .unwrap();
                nhr.transport_label = decoder.nhr_transport_label;

                Ok(nhr)
            }
        }
    }
}
