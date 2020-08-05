// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

pub mod error;
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

use crate::genetlink::{send_sandesh_msg, MessageHandleError};
pub use error::*;
pub use message_type::MessageType;
pub use sandesh::*;
pub use std::convert::TryInto;
pub use vr_bridge_table_data::BridgeTableData;
pub use vr_drop_stats::DropStats;
pub use vr_fc_map::FcMapRequest;
pub use vr_flow::*;
pub use vr_flow_response::FlowResponse;
pub use vr_flow_table_data::FlowTableData;
pub use vr_hugepage_config::HugepageConfig;
pub use vr_interface::*;
pub use vr_mem_stats::MemStatsRequest;
pub use vr_mirror::MirrorRequest;
pub use vr_mpls::MplsRequest;
pub use vr_nexthop::*;
pub use vr_pkt_droplog::PktDropLog;
pub use vr_qos_map::QosMapRequest;
pub use vr_response::VrResponse;
pub use vr_route::RouteRequest;
pub use vr_vrf::VrfRequest;
pub use vr_vrf_assign::VrfAssignRequest;
pub use vr_vrf_stats::VrfStatsRequest;
pub use vr_vxlan::VxlanRequest;
pub use vrouter_ops::VrouterOps;

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
    pub fn from_bytes(buf: Vec<u8>) -> Result<Message, CodecError> {
        match buf.clone().try_into().unwrap() {
            MessageType::BridgeTableData => {
                let req = BridgeTableData::read(buf)?;
                Ok(Message::BridgeTableData(req))
            }
            MessageType::DropStats => {
                let req = DropStats::read(buf)?;
                Ok(Message::DropStats(req))
            }
            MessageType::FcMapRequest => {
                let req = FcMapRequest::read(buf)?;
                Ok(Message::FcMapRequest(req))
            }
            MessageType::FlowResponse => {
                let req = FlowResponse::read(buf)?;
                Ok(Message::FlowResponse(req))
            }
            MessageType::FlowRequest => {
                let req = FlowRequest::read(buf)?;
                Ok(Message::FlowRequest(req))
            }
            MessageType::FlowTableData => {
                let req = FlowTableData::read(buf)?;
                Ok(Message::FlowTableData(req))
            }
            MessageType::HugepageConfig => {
                let req = HugepageConfig::read(buf)?;
                Ok(Message::HugepageConfig(req))
            }
            MessageType::InterfaceRequest => {
                let req = InterfaceRequest::read(buf)?;
                Ok(Message::InterfaceRequest(req))
            }
            MessageType::MemStatsRequest => {
                let req = MemStatsRequest::read(buf)?;
                Ok(Message::MemStatsRequest(req))
            }
            MessageType::MirrorRequest => {
                let req = MirrorRequest::read(buf)?;
                Ok(Message::MirrorRequest(req))
            }
            MessageType::MplsRequest => {
                let req = MplsRequest::read(buf)?;
                Ok(Message::MplsRequest(req))
            }
            MessageType::NexthopRequest => {
                let req = NexthopRequest::read(buf)?;
                Ok(Message::NexthopRequest(req))
            }
            MessageType::PktDropLog => {
                let req = PktDropLog::read(buf)?;
                Ok(Message::PktDropLog(req))
            }
            MessageType::QosMapRequest => {
                let req = QosMapRequest::read(buf)?;
                Ok(Message::QosMapRequest(req))
            }
            MessageType::VrResponse => {
                let req = VrResponse::read(buf)?;
                Ok(Message::VrResponse(req))
            }
            MessageType::RouteRequest => {
                let req = RouteRequest::read(buf)?;
                Ok(Message::RouteRequest(req))
            }
            MessageType::VrfRequest => {
                let req = VrfRequest::read(buf)?;
                Ok(Message::VrfRequest(req))
            }
            MessageType::VrfAssignRequest => {
                let req = VrfAssignRequest::read(buf)?;
                Ok(Message::VrfAssignRequest(req))
            }
            MessageType::VrfStatsRequest => {
                let req = VrfStatsRequest::read(buf)?;
                Ok(Message::VrfStatsRequest(req))
            }
            MessageType::VxlanRequest => {
                let req = VxlanRequest::read(buf)?;
                Ok(Message::VxlanRequest(req))
            }
            MessageType::VrouterOps => {
                let req = VrouterOps::read(buf)?;
                Ok(Message::VrouterOps(req))
            }
            MessageType::Unknown => Err(CodecError::UnknownMessageType),
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
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

    pub fn read_length(&self) -> usize {
        match self {
            Message::BridgeTableData(btable) => btable.read_length,
            Message::DropStats(vds) => vds.read_length,
            Message::FcMapRequest(fmr) => fmr.read_length,
            Message::FlowRequest(fr) => fr.read_length,
            Message::FlowResponse(fresp) => fresp.read_length,
            Message::FlowTableData(ftable) => ftable.read_length,
            Message::HugepageConfig(vhp) => vhp.read_length,
            Message::InterfaceRequest(ifreq) => ifreq.read_length,
            Message::MemStatsRequest(vms) => vms.read_length,
            Message::MirrorRequest(mirr) => mirr.read_length,
            Message::MplsRequest(mr) => mr.read_length,
            Message::NexthopRequest(nhreq) => nhreq.read_length,
            Message::PktDropLog(vdl) => vdl.read_length,
            Message::QosMapRequest(qmr) => qmr.read_length,
            Message::VrResponse(resp) => resp.read_length,
            Message::RouteRequest(rtr) => rtr.read_length,
            Message::VrfRequest(vrf) => vrf.read_length,
            Message::VrfAssignRequest(var) => var.read_length,
            Message::VrfStatsRequest(vsr) => vsr.read_length,
            Message::VxlanRequest(vxlanr) => vxlanr.read_length,
            Message::VrouterOps(vo) => vo.read_length,
        }
    }

    pub fn send_nl(&self) -> Result<Vec<Message>, MessageHandleError> {
        send_sandesh_msg(self)
    }
}
