// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

pub mod message_type;
pub mod sandesh;
pub mod vr_bridge_table_data;
pub mod vr_drop_stats;
pub mod vr_fc_map;
pub mod vr_flow;
pub mod vr_flow_response;
pub mod vr_flow_table_data;
pub mod vr_hugepage_config;
pub mod vr_interface;
pub mod vr_mem_stats;
pub mod vr_mirror;
pub mod vr_mpls;
pub mod vr_nexthop;
pub mod vr_pkt_droplog;
pub mod vr_qos_map;
pub mod vr_response;
pub mod vr_route;
pub mod vr_types;
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(improper_ctypes)]
pub mod vr_types_binding;
pub mod vr_vrf;
pub mod vr_vrf_assign;
pub mod vr_vrf_stats;
pub mod vr_vxlan;
pub mod vrouter_ops;

use message_type::MessageType;
use std::convert::TryInto;
use vr_bridge_table_data::BridgeTableData;
use vr_drop_stats::DropStats;
use vr_fc_map::FcMapRequest;
use vr_flow::FlowRequest;
use vr_flow_response::FlowResponse;
use vr_flow_table_data::FlowTableData;
use vr_hugepage_config::HugepageConfig;
use vr_interface::InterfaceRequest;
use vr_mem_stats::MemStatsRequest;
use vr_mirror::MirrorRequest;
use vr_mpls::MplsRequest;
use vr_nexthop::NexthopRequest;
use vr_pkt_droplog::PktDropLog;
use vr_qos_map::QosMapRequest;
use vr_response::VrResponse;
use vr_route::RouteRequest;
use vr_vrf::VrfRequest;
use vr_vrf_assign::VrfAssignRequest;
use vr_vrf_stats::VrfStatsRequest;
use vr_vxlan::VxlanRequest;
use vrouter_ops::VrouterOps;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    BridgeTableData(BridgeTableData),
    DropStats(DropStats),
    FcMapRequest(FcMapRequest),
    FlowRequest(FlowRequest),
    FlowResponse(FlowResponse),
    FlowTableData(FlowTableData),
    HugepageConfig(HugepageConfig),
    InterfaceRequest(InterfaceRequest),
    MemStatsRequest(MemStatsRequest),
    MirrorRequest(MirrorRequest),
    MplsRequest(MplsRequest),
    NexthopRequest(NexthopRequest),
    PktDropLog(PktDropLog),
    QosMapRequest(QosMapRequest),
    VrResponse(VrResponse),
    RouteRequest(RouteRequest),
    VrfRequest(VrfRequest),
    VrfAssignRequest(VrfAssignRequest),
    VrfStatsRequest(VrfStatsRequest),
    VxlanRequest(VxlanRequest),
    VrouterOps(VrouterOps),
}

impl Message {
    pub fn from_bytes<'a>(buf: Vec<u8>) -> Result<Message, &'a str> {
        match buf.clone().try_into().unwrap() {
            MessageType::BridgeTableData => match BridgeTableData::read(buf) {
                Ok(req) => Ok(Message::BridgeTableData(req)),
                Err(e) => Err(e),
            },
            MessageType::DropStats => match DropStats::read(buf) {
                Ok(req) => Ok(Message::DropStats(req)),
                Err(e) => Err(e),
            },
            MessageType::FcMapRequest => match FcMapRequest::read(buf) {
                Ok(req) => Ok(Message::FcMapRequest(req)),
                Err(e) => Err(e),
            },
            MessageType::FlowResponse => match FlowResponse::read(buf) {
                Ok(req) => Ok(Message::FlowResponse(req)),
                Err(e) => Err(e),
            },
            MessageType::FlowRequest => match FlowRequest::read(buf) {
                Ok(req) => Ok(Message::FlowRequest(req)),
                Err(e) => Err(e),
            },
            MessageType::FlowTableData => match FlowTableData::read(buf) {
                Ok(req) => Ok(Message::FlowTableData(req)),
                Err(e) => Err(e),
            },
            MessageType::HugepageConfig => match HugepageConfig::read(buf) {
                Ok(req) => Ok(Message::HugepageConfig(req)),
                Err(e) => Err(e),
            },
            MessageType::InterfaceRequest => {
                match InterfaceRequest::read(buf) {
                    Ok(req) => Ok(Message::InterfaceRequest(req)),
                    Err(e) => Err(e),
                }
            }
            MessageType::MemStatsRequest => match MemStatsRequest::read(buf) {
                Ok(req) => Ok(Message::MemStatsRequest(req)),
                Err(e) => Err(e),
            },
            MessageType::MirrorRequest => match MirrorRequest::read(buf) {
                Ok(req) => Ok(Message::MirrorRequest(req)),
                Err(e) => Err(e),
            },
            MessageType::MplsRequest => match MplsRequest::read(buf) {
                Ok(req) => Ok(Message::MplsRequest(req)),
                Err(e) => Err(e),
            },
            MessageType::NexthopRequest => match NexthopRequest::read(buf) {
                Ok(req) => Ok(Message::NexthopRequest(req)),
                Err(e) => Err(e),
            },
            MessageType::PktDropLog => match PktDropLog::read(buf) {
                Ok(req) => Ok(Message::PktDropLog(req)),
                Err(e) => Err(e),
            },
            MessageType::QosMapRequest => match QosMapRequest::read(buf) {
                Ok(req) => Ok(Message::QosMapRequest(req)),
                Err(e) => Err(e),
            },
            MessageType::VrResponse => match VrResponse::read(buf) {
                Ok(req) => Ok(Message::VrResponse(req)),
                Err(e) => Err(e),
            },
            MessageType::RouteRequest => match RouteRequest::read(buf) {
                Ok(req) => Ok(Message::RouteRequest(req)),
                Err(e) => Err(e),
            },
            MessageType::VrfRequest => match VrfRequest::read(buf) {
                Ok(req) => Ok(Message::VrfRequest(req)),
                Err(e) => Err(e),
            },
            MessageType::VrfAssignRequest => {
                match VrfAssignRequest::read(buf) {
                    Ok(req) => Ok(Message::VrfAssignRequest(req)),
                    Err(e) => Err(e),
                }
            }
            MessageType::VrfStatsRequest => match VrfStatsRequest::read(buf) {
                Ok(req) => Ok(Message::VrfStatsRequest(req)),
                Err(e) => Err(e),
            },
            MessageType::VxlanRequest => match VxlanRequest::read(buf) {
                Ok(req) => Ok(Message::VxlanRequest(req)),
                Err(e) => Err(e),
            },
            MessageType::VrouterOps => match VrouterOps::read(buf) {
                Ok(req) => Ok(Message::VrouterOps(req)),
                Err(e) => Err(e),
            },
            MessageType::Unknown => {
                Err("Failed to decode sandesh name from buffer")
            }
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        match self {
            Message::BridgeTableData(btable) => btable.write(),
            Message::DropStats(vds) => vds.write(),
            Message::FcMapRequest(fmr) => fmr.write(),
            Message::FlowRequest(fr) => fr.write(),
            Message::FlowResponse(fresp) => fresp.write(),
            Message::FlowTableData(ftable) => ftable.write(),
            Message::HugepageConfig(vhp) => vhp.write(),
            Message::InterfaceRequest(ifreq) => ifreq.write(),
            Message::MemStatsRequest(vms) => vms.write(),
            Message::MirrorRequest(mirr) => mirr.write(),
            Message::MplsRequest(mr) => mr.write(),
            Message::NexthopRequest(nhreq) => nhreq.write(),
            Message::PktDropLog(vdl) => vdl.write(),
            Message::QosMapRequest(qmr) => qmr.write(),
            Message::VrResponse(resp) => resp.write(),
            Message::RouteRequest(rtr) => rtr.write(),
            Message::VrfRequest(vrf) => vrf.write(),
            Message::VrfAssignRequest(var) => var.write(),
            Message::VrfStatsRequest(vsr) => vsr.write(),
            Message::VxlanRequest(vxlanr) => vxlanr.write(),
            Message::VrouterOps(vo) => vo.write(),
        }
    }
}
