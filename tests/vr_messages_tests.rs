// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_messages {
    use vr_type::vr_messages::vr_bridge_table_data::BridgeTableData;
    use vr_type::vr_messages::vr_drop_stats::DropStats;
    use vr_type::vr_messages::vr_fc_map::FcMapRequest;
    use vr_type::vr_messages::vr_flow::FlowRequest;
    use vr_type::vr_messages::vr_flow_response::FlowResponse;
    use vr_type::vr_messages::vr_flow_table_data::FlowTableData;
    use vr_type::vr_messages::vr_hugepage_config::HugepageConfig;
    use vr_type::vr_messages::vr_interface::InterfaceRequest;
    use vr_type::vr_messages::vr_mem_stats::MemStatsRequest;
    use vr_type::vr_messages::vr_mirror::MirrorRequest;
    use vr_type::vr_messages::vr_mpls::MplsRequest;
    use vr_type::vr_messages::vr_nexthop::NexthopRequest;
    use vr_type::vr_messages::vr_pkt_droplog::PktDropLog;
    use vr_type::vr_messages::vr_qos_map::QosMapRequest;
    use vr_type::vr_messages::vr_response::VrResponse;
    use vr_type::vr_messages::vr_route::RouteRequest;
    use vr_type::vr_messages::vr_vrf::VrfRequest;
    use vr_type::vr_messages::vr_vrf_assign::VrfAssignRequest;
    use vr_type::vr_messages::vr_vrf_stats::VrfStatsRequest;
    use vr_type::vr_messages::vr_vxlan::VxlanRequest;
    use vr_type::vr_messages::vrouter_ops::VrouterOps;
    use vr_type::vr_messages::Message;

    #[test]
    fn bridge_table_data() {
        use netlink_packet_core::NetlinkMessage;
        let payload = Message::BridgeTableData(BridgeTableData::default());
        let mut packet = NetlinkMessage::from(payload);
        packet.finalize();
        println!("{:?}", packet);

        let req = Message::BridgeTableData(BridgeTableData::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::BridgeTableData(BridgeTableData::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn drop_stats() {
        let req = Message::DropStats(DropStats::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::DropStats(DropStats::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn fc_map_req() {
        let req = Message::FcMapRequest(FcMapRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::FcMapRequest(FcMapRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn flow_req() {
        let req = Message::FlowRequest(FlowRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::FlowRequest(FlowRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn flow_response() {
        let req = Message::FlowResponse(FlowResponse::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::FlowResponse(FlowResponse::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn flow_table_data() {
        let req = Message::FlowTableData(FlowTableData::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::FlowTableData(FlowTableData::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn hugepage_config() {
        let req = Message::HugepageConfig(HugepageConfig::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::HugepageConfig(HugepageConfig::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn interface_req() {
        let req = Message::InterfaceRequest(InterfaceRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::InterfaceRequest(InterfaceRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn mem_stats_req() {
        let req = Message::MemStatsRequest(MemStatsRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::MemStatsRequest(MemStatsRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn mirror_req() {
        let req = Message::MirrorRequest(MirrorRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::MirrorRequest(MirrorRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn mpls_req() {
        let req = Message::MplsRequest(MplsRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::MplsRequest(MplsRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn nexthop_req() {
        let req = Message::NexthopRequest(NexthopRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::NexthopRequest(NexthopRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn pkt_droplog() {
        let req = Message::PktDropLog(PktDropLog::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::PktDropLog(PktDropLog::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn qos_map_req() {
        let req = Message::QosMapRequest(QosMapRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::QosMapRequest(QosMapRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vr_response() {
        let req = Message::VrResponse(VrResponse::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VrResponse(VrResponse::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn route_req() {
        let req = Message::RouteRequest(RouteRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::RouteRequest(RouteRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vrf_req() {
        let req = Message::VrfRequest(VrfRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VrfRequest(VrfRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vrf_assign_req() {
        let req = Message::VrfAssignRequest(VrfAssignRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VrfAssignRequest(VrfAssignRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vrf_stats_req() {
        let req = Message::VrfStatsRequest(VrfStatsRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VrfStatsRequest(VrfStatsRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vxlan_req() {
        let req = Message::VxlanRequest(VxlanRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VxlanRequest(VxlanRequest::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vrouter_ops() {
        let req = Message::VrouterOps(VrouterOps::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VrouterOps(VrouterOps::default()),
            Message::from_bytes(bytes).unwrap()
        );
    }
}

#[cfg(test)]
mod test_netlink_header {
    use vr_type::vr_messages::vr_bridge_table_data::BridgeTableData;
    use vr_type::vr_messages::Message;
    use vr_type::vr_messages::sandesh::SandeshOp::Add;
    use netlink_packet_core::{NetlinkMessage, NetlinkPayload};

    #[test]
    fn bridge_table_data() {
        let payload = Message::BridgeTableData(BridgeTableData::default());
        let mut packet = NetlinkMessage::from(payload);
        packet.finalize();
        assert_eq!(packet.header.length, 72);
        assert_eq!(packet.header.message_type, 1);
        assert_eq!(packet.header.flags, 0);
        assert_eq!(packet.header.sequence_number, 0);
        assert_eq!(packet.header.port_number, 0);
        assert_eq!(
            packet.payload,
            NetlinkPayload::InnerMessage(
                Message::BridgeTableData(
                    BridgeTableData {
                        op: Add,
                        rid: 0,
                        size: 0,
                        dev: 0,
                        file_path: "".to_string()
                    }
                )
            )
        )
    }
}
