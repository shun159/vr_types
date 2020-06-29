// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::vr_types_binding::flow_op;
use std::convert::TryFrom;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub const VR_IP6_ADDRESS_LEN: u32 = 16;

#[derive(Debug, PartialEq)]
pub enum FlowOp {
    Set,
    List,
    Get
}

impl TryFrom<flow_op> for FlowOp {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == FlowOp::Set as u32 => Ok(FlowOp::Set),
            x if x == FlowOp::List as u32 => Ok(FlowOp::List),
            x if x == FlowOp::Get as u32 => Ok(FlowOp::Get),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FlowAction {
    Drop,
    Hold,
    Forward,
    Nat
}

impl TryFrom<i16> for FlowAction {
    type Error = ();

    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            x if x == FlowAction::Drop as i16 => Ok(FlowAction::Drop),
            x if x == FlowAction::Hold as i16 => Ok(FlowAction::Hold),
            x if x == FlowAction::Forward as i16 => Ok(FlowAction::Forward),
            x if x == FlowAction::Nat as i16 => Ok(FlowAction::Nat),
            _ => Err(())
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
    pub drop_reason: u16,
    pub gen_id: i8,
    pub rflow_sip: IpAddr,
    pub rflow_dip: IpAddr,
    pub rflow_nh_id: u32,
    pub rflow_sport: u16,
    pub rflow_dport: u16,
    pub qos_id: u16,
    pub ttl: i8,
    pub extflags: i16,
    pub flags1: i16
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
            drop_reason: 0,
            gen_id: 0,
            rflow_sip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            rflow_dip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            rflow_nh_id: 0,
            rflow_sport: 0,
            rflow_dport: 0,
            qos_id: 0,
            ttl: 0,
            extflags: 0,
            flags1: 0
        }
    }
}
