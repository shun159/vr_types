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
        let mut expected = BridgeTableData::default();
        expected.read_length = 56;
        let req = Message::BridgeTableData(BridgeTableData::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::BridgeTableData(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn drop_stats() {
        let mut expected = DropStats::default();
        expected.read_length = 626;
        let req = Message::DropStats(DropStats::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::DropStats(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn fc_map_req() {
        let mut expected = FcMapRequest::default();
        expected.read_length = 75;
        let req = Message::FcMapRequest(FcMapRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::FcMapRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn flow_req() {
        let mut expected = FlowRequest::default();
        expected.read_length = 272;
        let req = Message::FlowRequest(FlowRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::FlowRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn flow_response() {
        let mut expected = FlowResponse::default();
        expected.read_length = 70;
        let req = Message::FlowResponse(FlowResponse::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::FlowResponse(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn flow_table_data() {
        let mut expected = FlowTableData::default();
        expected.read_length = 163;
        let req = Message::FlowTableData(FlowTableData::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::FlowTableData(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn hugepage_config() {
        let mut expected = HugepageConfig::default();
        expected.read_length = 77;
        let req = Message::HugepageConfig(HugepageConfig::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::HugepageConfig(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn interface_req() {
        let mut expected = InterfaceRequest::default();
        expected.read_length = 724;
        let req = Message::InterfaceRequest(InterfaceRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::InterfaceRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn mem_stats_req() {
        let mut expected = MemStatsRequest::default();
        expected.read_length = 803;
        let req = Message::MemStatsRequest(MemStatsRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::MemStatsRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn mirror_req() {
        let mut expected = MirrorRequest::default();
        expected.read_length = 75;
        let req = Message::MirrorRequest(MirrorRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::MirrorRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn mpls_req() {
        let mut expected = MplsRequest::default();
        expected.read_length = 49;
        let req = Message::MplsRequest(MplsRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::MplsRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn nexthop_req() {
        let mut expected = NexthopRequest::default();
        expected.read_length = 214;
        let req = Message::NexthopRequest(NexthopRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::NexthopRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn pkt_droplog() {
        let mut expected = PktDropLog::default();
        expected.read_length = 74;
        let req = Message::PktDropLog(PktDropLog::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::PktDropLog(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn qos_map_req() {
        let mut expected = QosMapRequest::default();
        expected.read_length = 89;
        let req = Message::QosMapRequest(QosMapRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::QosMapRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vr_response() {
        let mut expected = VrResponse::default();
        expected.read_length = 30;
        let req = Message::VrResponse(VrResponse::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VrResponse(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn route_req() {
        let mut expected = RouteRequest::default();
        expected.read_length = 114;
        let req = Message::RouteRequest(RouteRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::RouteRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vrf_req() {
        let mut expected = VrfRequest::default();
        expected.read_length = 62;
        let req = Message::VrfRequest(VrfRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VrfRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vrf_assign_req() {
        let mut expected = VrfAssignRequest::default();
        expected.read_length = 63;
        let req = Message::VrfAssignRequest(VrfAssignRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VrfAssignRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vrf_stats_req() {
        let mut expected = VrfStatsRequest::default();
        expected.read_length = 352;
        let req = Message::VrfStatsRequest(VrfStatsRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VrfStatsRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vxlan_req() {
        let mut expected = VxlanRequest::default();
        expected.read_length = 43;
        let req = Message::VxlanRequest(VxlanRequest::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VxlanRequest(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }

    #[test]
    fn vrouter_ops() {
        let mut expected = VrouterOps::default();
        expected.read_length = 328;
        let req = Message::VrouterOps(VrouterOps::default());
        let bytes = req.to_bytes().unwrap();
        assert_eq!(
            Message::VrouterOps(expected),
            Message::from_bytes(bytes).unwrap()
        );
    }
}
