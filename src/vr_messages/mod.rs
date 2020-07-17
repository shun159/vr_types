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

pub use sandesh::*;
pub use message_type::MessageType;
pub use std::convert::TryInto;
pub use vr_bridge_table_data::BridgeTableData;
pub use vr_drop_stats::DropStats;
pub use vr_fc_map::FcMapRequest;
pub use vr_flow::*;
pub use vr_flow_response::FlowResponse;
pub use vr_flow_table_data::FlowTableData;
pub use vr_hugepage_config::HugepageConfig;
pub use vr_interface::InterfaceRequest;
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

use netlink_packet_core::{
    NetlinkDeserializable, NetlinkHeader, NetlinkPayload, NetlinkSerializable,
};
use std::error::Error;
use std::fmt;

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

// Netlink Codec

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VnswNlAttrs {
    Unspec = 0,
    VrMessageProtocol = 1,
    Max = 3,
}

// A custom error type for when deserialization fails. This is
// required because `NetlinkDeserializable::Error` must implement
// `std::error::Error`, so a simple `String` won't cut it.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeserializeError(&'static str);

impl Error for DeserializeError {
    fn description(&self) -> &str {
        self.0
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// NetlinkDeserializable implementation
impl NetlinkDeserializable<Message> for Message {
    type Error = DeserializeError;

    fn deserialize(
        header: &NetlinkHeader,
        payload: &[u8],
    ) -> Result<Self, Self::Error> {
        match header.message_type {
            msg_type if msg_type == VnswNlAttrs::VrMessageProtocol as u16 => {
                Ok(Message::from_bytes(payload.to_vec()).unwrap())
            }
            _ => {
                Err(DeserializeError("invalid message: unhanded message type"))
            }
        }
    }
}

impl NetlinkSerializable<Message> for Message {
    fn message_type(&self) -> u16 {
        VnswNlAttrs::VrMessageProtocol as u16
    }

    fn buffer_len(&self) -> usize {
        self.to_bytes().unwrap().len()
    }

    fn serialize(&self, buf: &mut [u8]) {
        let bytes = &self.to_bytes().unwrap();
        buf.copy_from_slice(&bytes[..])
    }
}

// It can be convenient to be able to create a NetlinkMessage directly
// from a Message. Since NetlinkMessage<T> already implements
// From<NetlinkPayload<T>>, we just need to implement
// From<NetlinkPayload<Message>> for this to work.
impl From<Message> for NetlinkPayload<Message> {
    fn from(message: Message) -> Self {
        NetlinkPayload::InnerMessage(message)
    }
}
