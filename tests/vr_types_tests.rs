// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

mod test_encode_types {
    use vr_type::vr_messages::vr_types::VrSandesh;
    use vr_type::vr_messages::vr_types_binding::*;

    #[test]
    fn vr_nexthop_req() {
        let req = vr_nexthop_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_nexthop_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(214, res.len())
    }

    #[test]
    fn vr_interface_req() {
        let req = vr_interface_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_interface_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(724, res.len())
    }

    #[test]
    fn vr_vxlan_req() {
        let req = vr_vxlan_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_vxlan_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(43, res.len())
    }

    #[test]
    fn vr_route_req() {
        let req = vr_route_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_route_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(114, res.len())
    }

    #[test]
    fn vr_mpls_req() {
        let req = vr_mpls_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_mpls_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(49, res.len())
    }

    #[test]
    fn vr_mirror_req() {
        let req = vr_mirror_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_mirror_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(75, res.len())
    }

    #[test]
    fn vr_vrf_req() {
        let req = vr_vrf_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_vrf_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(62, res.len())
    }

    #[test]
    fn vr_flow_req() {
        let req = vr_flow_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_flow_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(272, res.len())
    }

    #[test]
    fn vr_vrf_assign_req() {
        let req = vr_vrf_assign_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_vrf_assign_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(63, res.len())
    }

    #[test]
    fn vr_vrf_stats_req() {
        let req = vr_vrf_stats_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_vrf_stats_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(352, res.len())
    }

    #[test]
    fn vr_response() {
        let req = vr_response::new();
        let res = req.write().unwrap();
        assert_eq!("vr_response", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(30, res.len())
    }

    #[test]
    fn vr_mem_stats_req() {
        let req = vr_mem_stats_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_mem_stats_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(803, res.len())
    }

    #[test]
    fn vr_pkt_drop_log_req() {
        let req = vr_pkt_drop_log_req::new();
        let res = req.write().unwrap();
        assert_eq!(
            "vr_pkt_drop_log_req",
            sandesh_info_t::sname_from_bytes(&res)
        );
        assert_eq!(74, res.len())
    }

    #[test]
    fn vr_drop_stats_req() {
        let req = vr_drop_stats_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_drop_stats_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(593, res.len())
    }

    #[test]
    fn vr_qos_map_req() {
        let req = vr_qos_map_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_qos_map_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(89, res.len())
    }

    #[test]
    fn vr_fc_map_req() {
        let req = vr_fc_map_req::new();
        let res = req.write().unwrap();
        assert_eq!("vr_fc_map_req", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(75, res.len())
    }

    #[test]
    fn vr_flow_response() {
        let req = vr_flow_response::new();
        let res = req.write().unwrap();
        assert_eq!("vr_flow_response", sandesh_info_t::sname_from_bytes(&res));
        assert_eq!(70, res.len())
    }

    #[test]
    fn vr_flow_table_data() {
        let req = vr_flow_table_data::new();
        let res = req.write().unwrap();
        assert_eq!(
            "vr_flow_table_data",
            sandesh_info_t::sname_from_bytes(&res)
        );
        assert_eq!(163, res.len())
    }

    #[test]
    fn vr_bridge_table_data() {
        let req = vr_bridge_table_data::new();
        let res = req.write().unwrap();
        assert_eq!(
            "vr_bridge_table_data",
            sandesh_info_t::sname_from_bytes(&res)
        );
        assert_eq!(56, res.len())
    }

    #[test]
    fn vr_hugepage_config() {
        let req = vr_hugepage_config::new();
        let res = req.write().unwrap();
        assert_eq!(
            "vr_hugepage_config",
            sandesh_info_t::sname_from_bytes(&res)
        );
        assert_eq!(53, res.len())
    }

    #[test]
    fn vrouter_ops() {
        let req = vrouter_ops::new();
        let res = req.write().unwrap();
        assert_eq!(
            "vrouter_ops",
            sandesh_info_t::sname_from_bytes(&res)
        );
        assert_eq!(324, res.len())
    }
}

#[cfg(test)]
mod test_decode_types {
    use vr_type::vr_messages::vr_types::VrSandesh;
    use vr_type::vr_messages::vr_types_binding::*;

    #[test]
    fn vr_nexthop_req() {
        let mut req = vr_nexthop_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_interface_req() {
        let mut req = vr_interface_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_vxlan_req() {
        let mut req = vr_vxlan_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_route_req() {
        let mut req = vr_route_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_mpls_req() {
        let mut req = vr_mpls_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_mirror_req() {
        let mut req = vr_mirror_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_vrf_req() {
        let mut req = vr_vrf_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_flow_req() {
        let mut req = vr_flow_req::new();
        req.fr_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.fr_op)
    }

    #[test]
    fn vr_vrf_assign_req() {
        let mut req = vr_vrf_assign_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_vrf_stats_req() {
        let mut req = vr_vrf_stats_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_response() {
        let mut req = vr_response::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_mem_stats_req() {
        let mut req = vr_mem_stats_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_pkt_drop_log_req() {
        let mut req = vr_pkt_drop_log_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_drop_stats_req() {
        let mut req = vr_drop_stats_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_qos_map_req() {
        let mut req = vr_qos_map_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_fc_map_req() {
        let mut req = vr_fc_map_req::new();
        req.h_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.h_op)
    }

    #[test]
    fn vr_flow_response() {
        let mut req = vr_flow_response::new();
        req.fresp_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.fresp_op)
    }

    #[test]
    fn vr_flow_table_data() {
        let mut req = vr_flow_table_data::new();
        req.ftable_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.ftable_op)
    }

    #[test]
    fn vr_bridge_table_data() {
        let mut req = vr_bridge_table_data::new();
        req.btable_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.btable_op)
    }

    #[test]
    fn vr_hugepage_config() {
        let mut req = vr_hugepage_config::new();
        req.vhp_op = 1;
        let bytes = req.write().unwrap();
        let _ = req.read(bytes);
        assert_eq!(1, req.vhp_op)
    }
}
