// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::vr_types_binding::flow_op;
use std::convert::TryFrom;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub const VR_FLOW_RESP_FLAG_DELETED: i16 = 0x0001;

pub const VR_FLOW_FLAG_ACTIVE: i16 = 0x0001;
pub const VR_FLOW_FLAG_MODIFIED: i16 = 0x0100;
pub const VR_FLOW_FLAG_NEW_FLOW: i16 = 0x0200;
pub const VR_FLOW_FLAG_EVICT_CANDIDATE: i16 = 0x0400;
pub const VR_FLOW_FLAG_EVICTED: i16 = 0x0800;
pub const VR_RFLOW_VALID: i16 = 0x1000;
pub const VR_FLOW_FLAG_MIRROR: i16 = 0x2000;
pub const VR_FLOW_FLAG_VRFT: i16 = 0x4000;
pub const VR_FLOW_FLAG_LINK_LOCAL: i16 = 0x8000;

// for NAT
pub const VR_FLOW_FLAG_SNAT: i16 = 0x2;
pub const VR_FLOW_FLAG_SPAT: i16 = 0x4;
pub const VR_FLOW_FLAG_DNAT: i16 = 0x8;
pub const VR_FLOW_FLAG_DPAT: i16 = 0x10;
pub const VR_FLOW_FLAG_NAT_MASK: i16 = (VR_FLOW_FLAG_SNAT
    | VR_FLOW_FLAG_SPAT
    | VR_FLOW_FLAG_DNAT
    | VR_FLOW_FLAG_DPAT);

// for TRAP
pub const VR_FLOW_FLAG_TRAP_ECMP: i16 = 0x20;
pub const VR_FLOW_FLAG_DELETE_MARKED: i16 = 0x40;

pub const VR_IP6_ADDRESS_LEN: u32 = 16;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

impl TryFrom<i16> for FlowDropReason {
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

#[derive(Debug, PartialEq)]
pub struct FlowRequest {
    pub op: FlowOp,
    pub rid: i16,
    pub index: i32,
    pub action: FlowAction,
    pub flags: i16,
    pub rindex: i32,
    pub family: i32,
    pub flow_sip: IpAddr,
    pub flow_dip: IpAddr,
    pub flow_sport: u16,
    pub flow_dport: u16,
    pub flow_proto: i8,
    pub flow_vrf: u16,
    pub flow_dvrf: u16,
    pub mir_id: u16,
    pub sec_mir_id: u16,
    pub mir_sip: Ipv4Addr,
    pub mir_sport: u16,
    pub pcap_meta_data: Vec<u8>,
    pub mir_vrf: u16,
    pub ecmp_nh_index: u32,
    pub src_nh_index: u32,
    pub flow_nh_id: u32,
    pub drop_reason: FlowDropReason,
    pub gen_id: i8,
    pub rflow_sip: IpAddr,
    pub rflow_dip: IpAddr,
    pub rflow_nh_id: u32,
    pub rflow_sport: u16,
    pub rflow_dport: u16,
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
            family: libc::ETH_P_IP as i32,
            flow_sip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            flow_dip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            flow_sport: 0,
            flow_dport: 0,
            flow_proto: 0,
            flow_vrf: 0,
            flow_dvrf: 0,
            mir_id: 0,
            sec_mir_id: 0,
            mir_sip: Ipv4Addr::UNSPECIFIED,
            mir_sport: 0,
            pcap_meta_data: vec![],
            mir_vrf: 0,
            ecmp_nh_index: 0,
            src_nh_index: 0,
            flow_nh_id: 0,
            drop_reason: FlowDropReason::Unknown,
            gen_id: 0,
            rflow_sip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            rflow_dip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            rflow_nh_id: 0,
            rflow_sport: 0,
            rflow_dport: 0,
            qos_id: 0,
            ttl: 0,
            extflags: 0,
            flags1: 0,
        }
    }
}
