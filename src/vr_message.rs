// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use vr_messages::vr_bridge_table_data::BridgeTableData;
use vr_messages::vr_drop_stats::DropStats;
use vr_messages::vr_fc_map::FcMapRequest;
use vr_messages::vr_flow::FlowRequest;
use vr_messages::vr_flow_response::FlowResponse;
use vr_messages::vr_flow_table_data::FlowTableData;
use vr_messages::vr_hugepage_config::HugepageConfig;
use vr_messages::vr_interface::InterfaceRequest;
use vr_messages::vr_mem_stats::MemStatsRequest;
use vr_messages::vr_mirror::MirrorRequest;
use vr_messages::vr_mpls::MplsRequest;
use vr_messages::vr_nexthop::NexthopRequest;
use vr_messages::vr_pkt_droplog::PktDropLog;
use vr_messages::vr_qos_map::QosMapRequest;
use vr_messages::vr_response::VrResponse;
use vr_messages::vr_route::RouteRequest;
use vr_messages::vr_types_binding::sandesh_info_t;
use vr_messages::vr_vrf::VrfRequest;
use vr_messages::vr_vrf_assign::VrfAssignRequest;
use vr_messages::vr_vrf_stats::VrfStatsRequest;
use vr_messages::vr_vxlan::VxlanRequest;
use vr_messages::vrouter_ops::VrouterOps;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageType {
    BridgeTableData,
    DropStats,
    FcMapRequest,
    FlowRequest,
    FlowResponse,
    FlowTableData,
    HugepageConfig,
    InterfaceRequest,
    MemStatsRequest,
    MirrorRequest,
    MplsRequest,
    NexthopRequest,
    PktDropLog,
    QosMapRequest,
    VrResponse,
    RouteRequest,
    VrfRequest,
    VrfAssignRequest,
    VrfStatsRequest,
    VxlanRequest,
    VrouterOps,
    Unknown,
}

impl TryFrom<Vec<u8>> for MessageType {
    type Error = ();

    fn try_from(buf: Vec<u8>) -> Result<MessageType, Self::Error> {
        match sandesh_info_t::sname_from_bytes(&buf) {
            "vr_bridge_table_data" => Ok(MessageType::BridgeTableData),
            "vr_drop_stats_req" => Ok(MessageType::DropStats),
            "vr_fc_map_req" => Ok(MessageType::FcMapRequest),
            "vr_flow_response" => Ok(MessageType::FlowResponse),
            "vr_flow_req" => Ok(MessageType::FlowRequest),
            "vr_flow_table_data" => Ok(MessageType::FlowTableData),
            "vr_hugepage_config" => Ok(MessageType::HugepageConfig),
            "vr_interface_req" => Ok(MessageType::InterfaceRequest),
            "vr_mem_stats_req" => Ok(MessageType::MemStatsRequest),
            "vr_mirror_req" => Ok(MessageType::MirrorRequest),
            "vr_mpls_req" => Ok(MessageType::MplsRequest),
            "vr_nexthop_req" => Ok(MessageType::NexthopRequest),
            "vr_pkt_drop_log_req" => Ok(MessageType::PktDropLog),
            "vr_qos_map_req" => Ok(MessageType::QosMapRequest),
            "vr_response" => Ok(MessageType::VrResponse),
            "vr_route_req" => Ok(MessageType::RouteRequest),
            "vr_vrf_req" => Ok(MessageType::VrfRequest),
            "vr_vrf_assign_req" => Ok(MessageType::VrfAssignRequest),
            "vr_vrf_stats_req" => Ok(MessageType::VrfStatsRequest),
            "vr_vxlan_req" => Ok(MessageType::VxlanRequest),
            "vrouter_ops" => Ok(MessageType::VrouterOps),
            &_ => Ok(MessageType::Unknown)
        }
    }
}

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
            }
            MessageType::DropStats => match DropStats::read(buf) {
                Ok(req) => Ok(Message::DropStats(req)),
                Err(e) => Err(e),
            }
            MessageType::FcMapRequest => match FcMapRequest::read(buf) {
                Ok(req) => Ok(Message::FcMapRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::FlowResponse => match FlowResponse::read(buf) {
                Ok(req) => Ok(Message::FlowResponse(req)),
                Err(e) => Err(e),
            }
            MessageType::FlowRequest => match FlowRequest::read(buf) {
                Ok(req) => Ok(Message::FlowRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::FlowTableData => match FlowTableData::read(buf) {
                Ok(req) => Ok(Message::FlowTableData(req)),
                Err(e) => Err(e),
            }
            MessageType::HugepageConfig => match HugepageConfig::read(buf) {
                Ok(req) => Ok(Message::HugepageConfig(req)),
                Err(e) => Err(e),
            }
            MessageType::InterfaceRequest => match InterfaceRequest::read(buf) {
                Ok(req) => Ok(Message::InterfaceRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::MemStatsRequest => match MemStatsRequest::read(buf) {
                Ok(req) => Ok(Message::MemStatsRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::MirrorRequest => match MirrorRequest::read(buf) {
                Ok(req) => Ok(Message::MirrorRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::MplsRequest => match MplsRequest::read(buf) {
                Ok(req) => Ok(Message::MplsRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::NexthopRequest => match NexthopRequest::read(buf) {
                Ok(req) => Ok(Message::NexthopRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::PktDropLog => match PktDropLog::read(buf) {
                Ok(req) => Ok(Message::PktDropLog(req)),
                Err(e) => Err(e),
            }
            MessageType::QosMapRequest => match QosMapRequest::read(buf) {
                Ok(req) => Ok(Message::QosMapRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::VrResponse => match VrResponse::read(buf) {
                Ok(req) => Ok(Message::VrResponse(req)),
                Err(e) => Err(e),
            }
            MessageType::RouteRequest => match RouteRequest::read(buf) {
                Ok(req) => Ok(Message::RouteRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::VrfRequest => match VrfRequest::read(buf) {
                Ok(req) => Ok(Message::VrfRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::VrfAssignRequest => match VrfAssignRequest::read(buf) {
                Ok(req) => Ok(Message::VrfAssignRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::VrfStatsRequest => match VrfStatsRequest::read(buf) {
                Ok(req) => Ok(Message::VrfStatsRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::VxlanRequest => match VxlanRequest::read(buf) {
                Ok(req) => Ok(Message::VxlanRequest(req)),
                Err(e) => Err(e),
            }
            MessageType::VrouterOps => match VrouterOps::read(buf) {
                Ok(req) => Ok(Message::VrouterOps(req)),
                Err(e) => Err(e),
            }
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
