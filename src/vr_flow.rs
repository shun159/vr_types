// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::utils;
use crate::vr_types_binding::{flow_op, vr_flow_req};
use crate::vr_types::VrSandesh;
use std::convert::{TryFrom, TryInto};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub const VR_FLOW_RESP_FLAG_DELETED: u16 = 0x0001;

pub const VR_FLOW_FLAG_ACTIVE: u16 = 0x0001;
pub const VR_FLOW_FLAG_MODIFIED: u16 = 0x0100;
pub const VR_FLOW_FLAG_NEW_FLOW: u16 = 0x0200;
pub const VR_FLOW_FLAG_EVICT_CANDIDATE: u16 = 0x0400;
pub const VR_FLOW_FLAG_EVICTED: u16 = 0x0800;
pub const VR_RFLOW_VALID: u16 = 0x1000;
pub const VR_FLOW_FLAG_MIRROR: u16 = 0x2000;
pub const VR_FLOW_FLAG_VRFT: u16 = 0x4000;
pub const VR_FLOW_FLAG_LINK_LOCAL: u16 = 0x8000;

// for NAT
pub const VR_FLOW_FLAG_SNAT: u16 = 0x2;
pub const VR_FLOW_FLAG_SPAT: u16 = 0x4;
pub const VR_FLOW_FLAG_DNAT: u16 = 0x8;
pub const VR_FLOW_FLAG_DPAT: u16 = 0x10;
pub const VR_FLOW_FLAG_NAT_MASK: u16 = (VR_FLOW_FLAG_SNAT
    | VR_FLOW_FLAG_SPAT
    | VR_FLOW_FLAG_DNAT
    | VR_FLOW_FLAG_DPAT);

// for TRAP
pub const VR_FLOW_FLAG_TRAP_ECMP: u16 = 0x20;
pub const VR_FLOW_FLAG_DELETE_MARKED: u16 = 0x40;

pub const VR_IP6_ADDRESS_LEN: u32 = 16;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FlowOp {
    Set,
    List,
    Get,
}

impl TryFrom<flow_op> for FlowOp {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == FlowOp::Set as u32 => Ok(FlowOp::Set),
            x if x == FlowOp::List as u32 => Ok(FlowOp::List),
            x if x == FlowOp::Get as u32 => Ok(FlowOp::Get),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FlowAction {
    Drop,
    Hold,
    Forward,
    Nat,
}

impl TryFrom<i16> for FlowAction {
    type Error = ();

    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            x if x == FlowAction::Drop as i16 => Ok(FlowAction::Drop),
            x if x == FlowAction::Hold as i16 => Ok(FlowAction::Hold),
            x if x == FlowAction::Forward as i16 => Ok(FlowAction::Forward),
            x if x == FlowAction::Nat as i16 => Ok(FlowAction::Nat),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FlowDropReason {
    Unknown,
    UnavailableIntf,
    Ipv4FwdDis,
    UnavailableVrf,
    NoSrcRoute,
    NoDstRoute,
    AuditEntry,
    VrfChange,
    NoReverseFlow,
    ReverseFlowChange,
    NatChange,
    FlowLimit,
    LinkLocalSrcNat,
    FailedVrouterInstall,
    InvalidL2Flow,
    FlowOnTsn,
    NoMirrorEntry,
    SameFlowRflowKey,
    PortMapDrop,
    NoSrcRouteL2Rpf,
    FatFlowNatConflict,
    Policy,
    OutPolicy,
    Sg,
    OutSg,
    ReverseSg,
    ReverseOutSg,
    FwPolicy,
    OutFwPolicy,
    ReverseFwPolicy,
    ReverseOutFwPolicy,
    FwaasPolicy,
    OutFwaasPolicy,
    ReverseFwaasPolicy,
    ReverseOutFwaasPolicy,
}

impl TryFrom<u16> for FlowDropReason {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == FlowDropReason::Unknown as u16 => {
                Ok(FlowDropReason::Unknown)
            }
            x if x == FlowDropReason::UnavailableIntf as u16 => {
                Ok(FlowDropReason::UnavailableIntf)
            }
            x if x == FlowDropReason::Ipv4FwdDis as u16 => {
                Ok(FlowDropReason::Ipv4FwdDis)
            }
            x if x == FlowDropReason::UnavailableVrf as u16 => {
                Ok(FlowDropReason::UnavailableVrf)
            }
            x if x == FlowDropReason::NoSrcRoute as u16 => {
                Ok(FlowDropReason::NoSrcRoute)
            }
            x if x == FlowDropReason::NoDstRoute as u16 => {
                Ok(FlowDropReason::NoDstRoute)
            }
            x if x == FlowDropReason::AuditEntry as u16 => {
                Ok(FlowDropReason::AuditEntry)
            }
            x if x == FlowDropReason::VrfChange as u16 => {
                Ok(FlowDropReason::VrfChange)
            }
            x if x == FlowDropReason::NoReverseFlow as u16 => {
                Ok(FlowDropReason::NoReverseFlow)
            }
            x if x == FlowDropReason::ReverseFlowChange as u16 => {
                Ok(FlowDropReason::ReverseFlowChange)
            }
            x if x == FlowDropReason::NatChange as u16 => {
                Ok(FlowDropReason::NatChange)
            }
            x if x == FlowDropReason::FlowLimit as u16 => {
                Ok(FlowDropReason::FlowLimit)
            }
            x if x == FlowDropReason::LinkLocalSrcNat as u16 => {
                Ok(FlowDropReason::LinkLocalSrcNat)
            }
            x if x == FlowDropReason::FailedVrouterInstall as u16 => {
                Ok(FlowDropReason::FailedVrouterInstall)
            }
            x if x == FlowDropReason::InvalidL2Flow as u16 => {
                Ok(FlowDropReason::InvalidL2Flow)
            }
            x if x == FlowDropReason::FlowOnTsn as u16 => {
                Ok(FlowDropReason::FlowOnTsn)
            }
            x if x == FlowDropReason::NoMirrorEntry as u16 => {
                Ok(FlowDropReason::NoMirrorEntry)
            }
            x if x == FlowDropReason::SameFlowRflowKey as u16 => {
                Ok(FlowDropReason::SameFlowRflowKey)
            }
            x if x == FlowDropReason::PortMapDrop as u16 => {
                Ok(FlowDropReason::PortMapDrop)
            }
            x if x == FlowDropReason::NoSrcRouteL2Rpf as u16 => {
                Ok(FlowDropReason::NoSrcRouteL2Rpf)
            }
            x if x == FlowDropReason::FatFlowNatConflict as u16 => {
                Ok(FlowDropReason::FatFlowNatConflict)
            }
            x if x == FlowDropReason::Policy as u16 => {
                Ok(FlowDropReason::Policy)
            }
            x if x == FlowDropReason::OutPolicy as u16 => {
                Ok(FlowDropReason::OutPolicy)
            }
            x if x == FlowDropReason::Sg as u16 => Ok(FlowDropReason::Sg),
            x if x == FlowDropReason::OutSg as u16 => Ok(FlowDropReason::OutSg),
            x if x == FlowDropReason::ReverseSg as u16 => {
                Ok(FlowDropReason::ReverseSg)
            }
            x if x == FlowDropReason::ReverseOutSg as u16 => {
                Ok(FlowDropReason::ReverseOutSg)
            }
            x if x == FlowDropReason::FwPolicy as u16 => {
                Ok(FlowDropReason::FwPolicy)
            }
            x if x == FlowDropReason::OutFwPolicy as u16 => {
                Ok(FlowDropReason::OutFwPolicy)
            }
            x if x == FlowDropReason::ReverseFwPolicy as u16 => {
                Ok(FlowDropReason::ReverseFwPolicy)
            }
            x if x == FlowDropReason::ReverseOutFwPolicy as u16 => {
                Ok(FlowDropReason::ReverseOutFwPolicy)
            }
            x if x == FlowDropReason::FwaasPolicy as u16 => {
                Ok(FlowDropReason::FwaasPolicy)
            }
            x if x == FlowDropReason::OutFwaasPolicy as u16 => {
                Ok(FlowDropReason::OutFwaasPolicy)
            }
            x if x == FlowDropReason::ReverseFwaasPolicy as u16 => {
                Ok(FlowDropReason::ReverseFwaasPolicy)
            }
            x if x == FlowDropReason::ReverseOutFwaasPolicy as u16 => {
                Ok(FlowDropReason::ReverseOutFwaasPolicy)
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowRequest {
    pub op: FlowOp,
    pub rid: i16,
    pub index: i32,
    pub action: FlowAction,
    pub flags: i16,
    pub rindex: i32,
    pub family: i32,
    pub flow_sip: Option<IpAddr>,
    pub flow_dip: Option<IpAddr>,
    pub flow_sport: u16,
    pub flow_dport: u16,
    pub flow_proto: i8,
    pub flow_vrf: u16,
    pub flow_dvrf: u16,
    pub mirror_id: u16,
    pub sec_mirror_id: u16,
    pub mirror_sip: IpAddr,
    pub mirror_sport: u16,
    pub pcap_meta_data: Vec<u8>,
    pub mirror_vrf: u16,
    pub ecmp_nh_index: u32,
    pub src_nh_index: u32,
    pub flow_nh_id: u32,
    pub drop_reason: FlowDropReason,
    pub gen_id: i8,
    pub reverse_flow_sip: Option<IpAddr>,
    pub reverse_flow_dip: Option<IpAddr>,
    pub reverse_flow_nh_id: u32,
    pub reverse_flow_sport: u16,
    pub reverse_flow_dport: u16,
    pub qos_id: u16,
    pub ttl: i8,
    pub extflags: i16,
    pub flags1: i16,
}

impl Default for FlowRequest {
    fn default() -> FlowRequest {
        FlowRequest {
            op: FlowOp::Set,
            rid: 0,
            index: 0,
            action: FlowAction::Drop,
            flags: 0,
            rindex: 0,
            family: 0,
            flow_sip: None,
            flow_dip: None,
            flow_sport: 0,
            flow_dport: 0,
            flow_proto: 0,
            flow_vrf: 0,
            flow_dvrf: 0,
            mirror_id: 0,
            sec_mirror_id: 0,
            mirror_sip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            mirror_sport: 0,
            pcap_meta_data: vec![],
            mirror_vrf: 0,
            ecmp_nh_index: 0,
            src_nh_index: 0,
            flow_nh_id: 0,
            drop_reason: FlowDropReason::Unknown,
            gen_id: 0,
            reverse_flow_sip: None,
            reverse_flow_dip: None,
            reverse_flow_nh_id: 0,
            reverse_flow_sport: 0,
            reverse_flow_dport: 0,
            qos_id: 0,
            ttl: 0,
            extflags: 0,
            flags1: 0,
        }
    }
}

impl FlowRequest {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_flow_req = vr_flow_req::new();
        encoder.fr_op = self.op as u32;
        encoder.fr_rid = self.rid;
        encoder.fr_index = self.index;
        encoder.fr_action = self.action as i16;
        encoder.fr_rindex = self.rindex;
        encoder.fr_family = self.family;
        let flow_sip = Self::write_ip(&self.flow_sip);
        encoder.fr_flow_sip_u = flow_sip.0;
        encoder.fr_flow_sip_l = flow_sip.1;
        let flow_dip = Self::write_ip(&self.flow_dip);
        encoder.fr_flow_dip_u = flow_dip.0;
        encoder.fr_flow_dip_l = flow_dip.1;
        encoder.fr_flow_sport = self.flow_sport;
        encoder.fr_flow_dport = self.flow_dport;
        encoder.fr_flow_proto = self.flow_proto;
        encoder.fr_flow_vrf = self.flow_vrf;
        encoder.fr_flow_dvrf = self.flow_dvrf;
        encoder.fr_mir_id = self.mirror_id;
        encoder.fr_sec_mir_id = self.sec_mirror_id;
        let mirror_sip = Self::write_ip(&Some(self.mirror_sip)).1 as u32;
        encoder.fr_mir_sip = mirror_sip;
        encoder.fr_mir_sport = self.mirror_sport;
        encoder.fr_pcap_meta_data = utils::into_mut_ptr(&self.pcap_meta_data) as *mut i8;
        encoder.fr_pcap_meta_data_size = self.pcap_meta_data.len() as u32;
        encoder.fr_mir_vrf = self.mirror_vrf;
        encoder.fr_ecmp_nh_index = self.ecmp_nh_index;
        encoder.fr_src_nh_index = self.src_nh_index;
        encoder.fr_flow_nh_id = self.flow_nh_id;
        encoder.fr_drop_reason = self.drop_reason as u16;
        encoder.fr_gen_id = self.gen_id;
        let rflow_sip = Self::write_ip(&self.reverse_flow_sip);
        encoder.fr_rflow_sip_u = rflow_sip.0;
        encoder.fr_rflow_sip_l = rflow_sip.1;
        let rflow_dip = Self::write_ip(&self.reverse_flow_dip);
        encoder.fr_rflow_dip_u = rflow_dip.0;
        encoder.fr_rflow_dip_l = rflow_dip.1;
        encoder.fr_rflow_nh_id = self.reverse_flow_nh_id;
        encoder.fr_rflow_sport = self.reverse_flow_sport;
        encoder.fr_rflow_dport = self.reverse_flow_dport;
        encoder.fr_qos_id = self.qos_id;
        encoder.fr_ttl = self.ttl;
        encoder.fr_extflags = self.extflags;
        encoder.fr_flags1 = self.flags1;

        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<FlowRequest, &'a str> {
        let decoder: vr_flow_req = vr_flow_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut fr: FlowRequest = FlowRequest::default();
                fr.op = decoder.fr_op.try_into().unwrap();
                fr.rid = decoder.fr_rid;
                fr.index = decoder.fr_index;
                fr.action = decoder.fr_action.try_into().unwrap();
                fr.rindex = decoder.fr_rindex;
                fr.family = decoder.fr_family;
                fr.flow_sip = Self::read_ip(
                    decoder.fr_family,
                    decoder.fr_flow_sip_u,
                    decoder.fr_flow_sip_l,
                );
                fr.flow_dip = Self::read_ip(
                    decoder.fr_family,
                    decoder.fr_flow_dip_u,
                    decoder.fr_flow_dip_l,
                );
                fr.flow_sport = decoder.fr_flow_sport;
                fr.flow_dport = decoder.fr_flow_dport;
                fr.flow_proto = decoder.fr_flow_proto;
                fr.flow_vrf = decoder.fr_flow_vrf;
                fr.flow_dvrf = decoder.fr_flow_dvrf;
                fr.mirror_id = decoder.fr_mir_id;
                fr.sec_mirror_id = decoder.fr_sec_mir_id;
                fr.mirror_sip = Self::read_ip4(decoder.fr_mir_sip as u128);
                fr.mirror_sport = decoder.fr_mir_sport;
                fr.ecmp_nh_index = decoder.fr_ecmp_nh_index;
                fr.src_nh_index = decoder.fr_src_nh_index;
                fr.flow_nh_id = decoder.fr_flow_nh_id;
                fr.drop_reason = decoder.fr_drop_reason.try_into().unwrap();
                fr.gen_id = decoder.fr_gen_id;
                fr.reverse_flow_sip = Self::read_ip(
                    decoder.fr_family,
                    decoder.fr_rflow_sip_u,
                    decoder.fr_rflow_sip_l,
                );
                fr.reverse_flow_dip = Self::read_ip(
                    decoder.fr_family,
                    decoder.fr_rflow_dip_u,
                    decoder.fr_rflow_dip_l,
                );
                fr.reverse_flow_nh_id = decoder.fr_rflow_nh_id;
                fr.reverse_flow_sport = decoder.fr_rflow_sport;
                fr.reverse_flow_dport = decoder.fr_rflow_dport;
                fr.qos_id = decoder.fr_qos_id;
                fr.ttl = decoder.fr_ttl;
                fr.extflags = decoder.fr_extflags;
                fr.flags1 = decoder.fr_flags1;
                Ok(fr)
            }
        }
    }

    fn write_ip(ip: &Option<IpAddr>) -> (u64, u64) {
        match ip {
            None => (0u64, 0u64),
            Some(IpAddr::V4(ip)) => {
                let ip = utils::write_ip4(*ip) as u64;
                (0u64, ip)
            }
            Some(IpAddr::V6(ip)) => {
                let ip = utils::write_ip6(*ip);
                let ip_u = ((ip & 0xffffffffffffffff_0000000000000000) >> 64) as u64;
                let ip_l = (ip & 0x0000000000000000_ffffffffffffffff) as u64;
                (ip_u, ip_l)
            }
        }
    }

    fn read_ip(family: i32, ip_u: u64, ip_l: u64) -> Option<IpAddr> {
        let ip: u128 = (ip_u as u128) << 64 | ip_l as u128;
        match family {
            x if x == libc::AF_INET6 as i32 => Some(Self::read_ip6(ip)),
            x if x == libc::AF_INET as i32 => Some(Self::read_ip4(ip)),
            _ => None,
        }
    }

    fn read_ip6(ip: u128) -> IpAddr {
        IpAddr::V6(Ipv6Addr::from(ip as u128))
    }

    fn read_ip4(ip: u128) -> IpAddr {
        IpAddr::V4(Ipv4Addr::from((ip & 0x00000000_ffffffff) as u32))
    }
}
