// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::vr_types_binding::sandesh_info_t;
use std::convert::TryFrom;

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
