// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_mem_stats {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_mem_stats::MemStatsRequest;

    #[test]
    fn complex_request() {
        let mut vms: MemStatsRequest = MemStatsRequest::default();

        vms.op = SandeshOp::Dump;
        vms.rid = 1;
        vms.alloced = 1;
        vms.freed = 1;
        vms.assembler_table_object = 1;
        vms.bridge_mac_object = 1;
        vms.btable_object = 1;
        vms.build_info_object = 1;
        vms.defer_object = 1;
        vms.drop_stats_object = 1;
        vms.drop_stats_req_object = 1;
        vms.flow_queue_object = 1;
        vms.flow_req_object = 1;
        vms.flow_req_path_object = 1;
        vms.flow_hold_stat_object = 1;
        vms.flow_link_local_object = 1;
        vms.flow_metadata_object = 1;
        vms.flow_table_data_object = 1;
        vms.flow_table_info_object = 1;
        vms.fragment_object = 1;
        vms.fragment_queue_object = 1;
        vms.fragment_queue_element_object = 1;
        vms.fragment_scanner_object = 1;
        vms.hpacket_pool_object = 1;
        vms.htable_object = 1;
        vms.interface_object = 1;
        vms.interface_mac_object = 1;
        vms.interface_req_object = 1;
        vms.interface_req_mac_object = 1;
        vms.interface_req_name_object = 1;
        vms.interface_stats_object = 1;
        vms.interface_table_object = 1;
        vms.interface_vrf_table_object = 1;
        vms.itable_object = 1;
        vms.malloc_object = 1;
        vms.message_object = 1;
        vms.message_response_object = 1;
        vms.message_dump_object = 1;
        vms.mem_stats_req_object = 1;
        vms.mirror_object = 1;
        vms.mirror_table_object = 1;
        vms.mirror_meta_object = 1;
        vms.mtrie_object = 1;
        vms.mtrie_bucket_object = 1;
        vms.mtrie_stats_object = 1;
        vms.mtrie_table_object = 1;
        vms.network_address_object = 1;
        vms.nexthop_object = 1;
        vms.nexthop_component_object = 1;
        vms.nexthop_req_list_object = 1;
        vms.nexthop_req_encap_object = 1;
        vms.nexthop_req_object = 1;
        vms.route_table_object = 1;
        vms.route_req_mac_object = 1;
        vms.timer_object = 1;
        vms.usock_object = 1;
        vms.usock_poll_object = 1;
        vms.usock_buf_object = 1;
        vms.usock_iovec_object = 1;
        vms.vrouter_req_object = 1;
        vms.interface_fat_flow_config_object = 1;
        vms.qos_map_object = 1;
        vms.fc_object = 1;
        vms.interface_mirror_meta_object = 1;
        vms.interface_req_mirror_meta_object = 1;
        vms.interface_bridge_lock_object = 1;
        vms.interface_queue_object = 1;
        vms.interface_req_pbb_mac_object = 1;
        vms.nexthop_req_bmac_object = 1;
        vms.interface_req_bridge_id_object = 1;
        vms.interface_fat_flow_ipv4_exclude_list_object = 1;
        vms.interface_fat_flow_ipv6_exclude_list_object = 1;

        let bytes = vms.write().unwrap();
        let vms: MemStatsRequest = MemStatsRequest::read(bytes).unwrap();

        assert_eq!(vms.op, SandeshOp::Dump);
        assert_eq!(vms.rid, 1);
        assert_eq!(vms.alloced, 1);
        assert_eq!(vms.freed, 1);
        assert_eq!(vms.assembler_table_object, 1);
        assert_eq!(vms.bridge_mac_object, 1);
        assert_eq!(vms.btable_object, 1);
        assert_eq!(vms.build_info_object, 1);
        assert_eq!(vms.defer_object, 1);
        assert_eq!(vms.drop_stats_object, 1);
        assert_eq!(vms.drop_stats_req_object, 1);
        assert_eq!(vms.flow_queue_object, 1);
        assert_eq!(vms.flow_req_object, 1);
        assert_eq!(vms.flow_req_path_object, 1);
        assert_eq!(vms.flow_hold_stat_object, 1);
        assert_eq!(vms.flow_link_local_object, 1);
        assert_eq!(vms.flow_metadata_object, 1);
        assert_eq!(vms.flow_table_data_object, 1);
        assert_eq!(vms.flow_table_info_object, 1);
        assert_eq!(vms.fragment_object, 1);
        assert_eq!(vms.fragment_queue_object, 1);
        assert_eq!(vms.fragment_queue_element_object, 1);
        assert_eq!(vms.fragment_scanner_object, 1);
        assert_eq!(vms.hpacket_pool_object, 1);
        assert_eq!(vms.htable_object, 1);
        assert_eq!(vms.interface_object, 1);
        assert_eq!(vms.interface_mac_object, 1);
        assert_eq!(vms.interface_req_object, 1);
        assert_eq!(vms.interface_req_mac_object, 1);
        assert_eq!(vms.interface_req_name_object, 1);
        assert_eq!(vms.interface_stats_object, 1);
        assert_eq!(vms.interface_table_object, 1);
        assert_eq!(vms.interface_vrf_table_object, 1);
        assert_eq!(vms.itable_object, 1);
        assert_eq!(vms.malloc_object, 1);
        assert_eq!(vms.message_object, 1);
        assert_eq!(vms.message_response_object, 1);
        assert_eq!(vms.message_dump_object, 1);
        assert_eq!(vms.mem_stats_req_object, 1);
        assert_eq!(vms.mirror_object, 1);
        assert_eq!(vms.mirror_table_object, 1);
        assert_eq!(vms.mirror_meta_object, 1);
        assert_eq!(vms.mtrie_object, 1);
        assert_eq!(vms.mtrie_bucket_object, 1);
        assert_eq!(vms.mtrie_stats_object, 1);
        assert_eq!(vms.mtrie_table_object, 1);
        assert_eq!(vms.network_address_object, 1);
        assert_eq!(vms.nexthop_object, 1);
        assert_eq!(vms.nexthop_component_object, 1);
        assert_eq!(vms.nexthop_req_list_object, 1);
        assert_eq!(vms.nexthop_req_encap_object, 1);
        assert_eq!(vms.nexthop_req_object, 1);
        assert_eq!(vms.route_table_object, 1);
        assert_eq!(vms.route_req_mac_object, 1);
        assert_eq!(vms.timer_object, 1);
        assert_eq!(vms.usock_object, 1);
        assert_eq!(vms.usock_poll_object, 1);
        assert_eq!(vms.usock_buf_object, 1);
        assert_eq!(vms.usock_iovec_object, 1);
        assert_eq!(vms.vrouter_req_object, 1);
        assert_eq!(vms.interface_fat_flow_config_object, 1);
        assert_eq!(vms.qos_map_object, 1);
        assert_eq!(vms.fc_object, 1);
        assert_eq!(vms.interface_mirror_meta_object, 1);
        assert_eq!(vms.interface_req_mirror_meta_object, 1);
        assert_eq!(vms.interface_bridge_lock_object, 1);
        assert_eq!(vms.interface_queue_object, 1);
        assert_eq!(vms.interface_req_pbb_mac_object, 1);
        assert_eq!(vms.nexthop_req_bmac_object, 1);
        assert_eq!(vms.interface_req_bridge_id_object, 1);
        assert_eq!(vms.interface_fat_flow_ipv4_exclude_list_object, 1);
        assert_eq!(vms.interface_fat_flow_ipv6_exclude_list_object, 1);
    }

    #[test]
    fn empty_requset() {
        let vms: MemStatsRequest = MemStatsRequest::default();
        let bytes = vms.write().unwrap();
        let vms: MemStatsRequest = MemStatsRequest::read(bytes).unwrap();

        assert_eq!(vms.rid, 0);
        assert_eq!(vms.alloced, 0);
        assert_eq!(vms.freed, 0);
        assert_eq!(vms.assembler_table_object, 0);
        assert_eq!(vms.bridge_mac_object, 0);
        assert_eq!(vms.btable_object, 0);
        assert_eq!(vms.build_info_object, 0);
        assert_eq!(vms.defer_object, 0);
        assert_eq!(vms.drop_stats_object, 0);
        assert_eq!(vms.drop_stats_req_object, 0);
        assert_eq!(vms.flow_queue_object, 0);
        assert_eq!(vms.flow_req_object, 0);
        assert_eq!(vms.flow_req_path_object, 0);
        assert_eq!(vms.flow_hold_stat_object, 0);
        assert_eq!(vms.flow_link_local_object, 0);
        assert_eq!(vms.flow_metadata_object, 0);
        assert_eq!(vms.flow_table_data_object, 0);
        assert_eq!(vms.flow_table_info_object, 0);
        assert_eq!(vms.fragment_object, 0);
        assert_eq!(vms.fragment_queue_object, 0);
        assert_eq!(vms.fragment_queue_element_object, 0);
        assert_eq!(vms.fragment_scanner_object, 0);
        assert_eq!(vms.hpacket_pool_object, 0);
        assert_eq!(vms.htable_object, 0);
        assert_eq!(vms.interface_object, 0);
        assert_eq!(vms.interface_mac_object, 0);
        assert_eq!(vms.interface_req_object, 0);
        assert_eq!(vms.interface_req_mac_object, 0);
        assert_eq!(vms.interface_req_name_object, 0);
        assert_eq!(vms.interface_stats_object, 0);
        assert_eq!(vms.interface_table_object, 0);
        assert_eq!(vms.interface_vrf_table_object, 0);
        assert_eq!(vms.itable_object, 0);
        assert_eq!(vms.malloc_object, 0);
        assert_eq!(vms.message_object, 0);
        assert_eq!(vms.message_response_object, 0);
        assert_eq!(vms.message_dump_object, 0);
        assert_eq!(vms.mem_stats_req_object, 0);
        assert_eq!(vms.mirror_object, 0);
        assert_eq!(vms.mirror_table_object, 0);
        assert_eq!(vms.mirror_meta_object, 0);
        assert_eq!(vms.mtrie_object, 0);
        assert_eq!(vms.mtrie_bucket_object, 0);
        assert_eq!(vms.mtrie_stats_object, 0);
        assert_eq!(vms.mtrie_table_object, 0);
        assert_eq!(vms.network_address_object, 0);
        assert_eq!(vms.nexthop_object, 0);
        assert_eq!(vms.nexthop_component_object, 0);
        assert_eq!(vms.nexthop_req_list_object, 0);
        assert_eq!(vms.nexthop_req_encap_object, 0);
        assert_eq!(vms.nexthop_req_object, 0);
        assert_eq!(vms.route_table_object, 0);
        assert_eq!(vms.route_req_mac_object, 0);
        assert_eq!(vms.timer_object, 0);
        assert_eq!(vms.usock_object, 0);
        assert_eq!(vms.usock_poll_object, 0);
        assert_eq!(vms.usock_buf_object, 0);
        assert_eq!(vms.usock_iovec_object, 0);
        assert_eq!(vms.vrouter_req_object, 0);
        assert_eq!(vms.interface_fat_flow_config_object, 0);
        assert_eq!(vms.qos_map_object, 0);
        assert_eq!(vms.fc_object, 0);
        assert_eq!(vms.interface_mirror_meta_object, 0);
        assert_eq!(vms.interface_req_mirror_meta_object, 0);
        assert_eq!(vms.interface_bridge_lock_object, 0);
        assert_eq!(vms.interface_queue_object, 0);
        assert_eq!(vms.interface_req_pbb_mac_object, 0);
        assert_eq!(vms.nexthop_req_bmac_object, 0);
        assert_eq!(vms.interface_req_bridge_id_object, 0);
        assert_eq!(vms.interface_fat_flow_ipv4_exclude_list_object, 0);
        assert_eq!(vms.interface_fat_flow_ipv6_exclude_list_object, 0);
    }
}
