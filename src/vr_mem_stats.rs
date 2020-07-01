// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_mem_stats_req;
use std::convert::TryInto;

pub struct MemStats {
    pub op: SandeshOp,
    pub rid: i16,
    pub alloced: i64,
    pub freed: i64,
    pub assembler_table_object: i64,
    pub bridge_mac_object: i64,
    pub btable_object: i64,
    pub build_info_object: i64,
    pub defer_object: i64,
    pub drop_stats_object: i64,
    pub drop_stats_req_object: i64,
    pub flow_queue_object: i64,
    pub flow_req_object: i64,
    pub flow_req_path_object: i64,
    pub flow_hold_stat_object: i64,
    pub flow_link_local_object: i64,
    pub flow_metadata_object: i64,
    pub flow_table_data_object: i64,
    pub flow_table_info_object: i64,
    pub fragment_object: i64,
    pub fragment_queue_object: i64,
    pub fragment_queue_element_object: i64,
    pub fragment_scanner_object: i64,
    pub hpacket_pool_object: i64,
    pub htable_object: i64,
    pub interface_object: i64,
    pub interface_mac_object: i64,
    pub interface_req_object: i64,
    pub interface_req_mac_object: i64,
    pub interface_req_name_object: i64,
    pub interface_stats_object: i64,
    pub interface_table_object: i64,
    pub interface_vrf_table_object: i64,
    pub itable_object: i64,
    pub malloc_object: i64,
    pub message_object: i64,
    pub message_response_object: i64,
    pub message_dump_object: i64,
    pub mem_stats_req_object: i64,
    pub mirror_object: i64,
    pub mirror_table_object: i64,
    pub mirror_meta_object: i64,
    pub mtrie_object: i64,
    pub mtrie_bucket_object: i64,
    pub mtrie_stats_object: i64,
    pub mtrie_table_object: i64,
    pub network_address_object: i64,
    pub nexthop_object: i64,
    pub nexthop_component_object: i64,
    pub nexthop_req_list_object: i64,
    pub nexthop_req_encap_object: i64,
    pub nexthop_req_object: i64,
    pub route_table_object: i64,
    pub route_req_mac_object: i64,
    pub timer_object: i64,
    pub usock_object: i64,
    pub usock_poll_object: i64,
    pub usock_buf_object: i64,
    pub usock_iovec_object: i64,
    pub vrouter_req_object: i64,
    pub interface_fat_flow_config_object: i64,
    pub qos_map_object: i64,
    pub fc_object: i64,
    pub interface_mirror_meta_object: i64,
    pub interface_req_mirror_meta_object: i64,
    pub interface_bridge_lock_object: i64,
    pub interface_queue_object: i64,
    pub interface_req_pbb_mac_object: i64,
    pub nexthop_req_bmac_object: i64,
    pub interface_req_bridge_id_object: i64,
    pub interface_fat_flow_ipv4_exclude_list_object: i64,
    pub interface_fat_flow_ipv6_exclude_list_object: i64,
}

impl Default for MemStats {
    fn default() -> MemStats {
        MemStats {
            op: SandeshOp::Add,
            rid: 0,
            alloced: 0,
            freed: 0,
            assembler_table_object: 0,
            bridge_mac_object: 0,
            btable_object: 0,
            build_info_object: 0,
            defer_object: 0,
            drop_stats_object: 0,
            drop_stats_req_object: 0,
            flow_queue_object: 0,
            flow_req_object: 0,
            flow_req_path_object: 0,
            flow_hold_stat_object: 0,
            flow_link_local_object: 0,
            flow_metadata_object: 0,
            flow_table_data_object: 0,
            flow_table_info_object: 0,
            fragment_object: 0,
            fragment_queue_object: 0,
            fragment_queue_element_object: 0,
            fragment_scanner_object: 0,
            hpacket_pool_object: 0,
            htable_object: 0,
            interface_object: 0,
            interface_mac_object: 0,
            interface_req_object: 0,
            interface_req_mac_object: 0,
            interface_req_name_object: 0,
            interface_stats_object: 0,
            interface_table_object: 0,
            interface_vrf_table_object: 0,
            itable_object: 0,
            malloc_object: 0,
            message_object: 0,
            message_response_object: 0,
            message_dump_object: 0,
            mem_stats_req_object: 0,
            mirror_object: 0,
            mirror_table_object: 0,
            mirror_meta_object: 0,
            mtrie_object: 0,
            mtrie_bucket_object: 0,
            mtrie_stats_object: 0,
            mtrie_table_object: 0,
            network_address_object: 0,
            nexthop_object: 0,
            nexthop_component_object: 0,
            nexthop_req_list_object: 0,
            nexthop_req_encap_object: 0,
            nexthop_req_object: 0,
            route_table_object: 0,
            route_req_mac_object: 0,
            timer_object: 0,
            usock_object: 0,
            usock_poll_object: 0,
            usock_buf_object: 0,
            usock_iovec_object: 0,
            vrouter_req_object: 0,
            interface_fat_flow_config_object: 0,
            qos_map_object: 0,
            fc_object: 0,
            interface_mirror_meta_object: 0,
            interface_req_mirror_meta_object: 0,
            interface_bridge_lock_object: 0,
            interface_queue_object: 0,
            interface_req_pbb_mac_object: 0,
            nexthop_req_bmac_object: 0,
            interface_req_bridge_id_object: 0,
            interface_fat_flow_ipv4_exclude_list_object: 0,
            interface_fat_flow_ipv6_exclude_list_object: 0,
        }
    }
}

impl MemStats {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_mem_stats_req = vr_mem_stats_req::new();
        encoder.h_op = self.op as u32;
        encoder.vms_rid = self.rid;
        encoder.vms_alloced = self.alloced;
        encoder.vms_freed = self.freed;
        encoder.vms_assembler_table_object = self.assembler_table_object;
        encoder.vms_bridge_mac_object = self.bridge_mac_object;
        encoder.vms_btable_object = self.btable_object;
        encoder.vms_build_info_object = self.build_info_object;
        encoder.vms_defer_object = self.defer_object;
        encoder.vms_drop_stats_object = self.drop_stats_object;
        encoder.vms_drop_stats_req_object = self.drop_stats_req_object;
        encoder.vms_flow_queue_object = self.flow_queue_object;
        encoder.vms_flow_req_object = self.flow_req_object;
        encoder.vms_flow_req_path_object = self.flow_req_path_object;
        encoder.vms_flow_hold_stat_object = self.flow_hold_stat_object;
        encoder.vms_flow_link_local_object = self.flow_link_local_object;
        encoder.vms_flow_metadata_object = self.flow_metadata_object;
        encoder.vms_flow_table_data_object = self.flow_table_data_object;
        encoder.vms_flow_table_info_object = self.flow_table_info_object;
        encoder.vms_fragment_object = self.fragment_object;
        encoder.vms_fragment_queue_object = self.fragment_queue_object;
        encoder.vms_fragment_queue_element_object =
            self.fragment_queue_element_object;
        encoder.vms_fragment_scanner_object = self.fragment_scanner_object;
        encoder.vms_hpacket_pool_object = self.hpacket_pool_object;
        encoder.vms_htable_object = self.htable_object;
        encoder.vms_interface_object = self.interface_object;
        encoder.vms_interface_mac_object = self.interface_mac_object;
        encoder.vms_interface_req_object = self.interface_req_object;
        encoder.vms_interface_req_mac_object = self.interface_req_mac_object;
        encoder.vms_interface_req_name_object = self.interface_req_name_object;
        encoder.vms_interface_stats_object = self.interface_stats_object;
        encoder.vms_interface_table_object = self.interface_table_object;
        encoder.vms_interface_vrf_table_object =
            self.interface_vrf_table_object;
        encoder.vms_itable_object = self.itable_object;
        encoder.vms_malloc_object = self.malloc_object;
        encoder.vms_message_object = self.message_object;
        encoder.vms_message_response_object = self.message_response_object;
        encoder.vms_message_dump_object = self.message_dump_object;
        encoder.vms_mem_stats_req_object = self.mem_stats_req_object;
        encoder.vms_mirror_object = self.mirror_object;
        encoder.vms_mirror_table_object = self.mirror_table_object;
        encoder.vms_mirror_meta_object = self.mirror_meta_object;
        encoder.vms_mtrie_object = self.mtrie_object;
        encoder.vms_mtrie_bucket_object = self.mtrie_bucket_object;
        encoder.vms_mtrie_stats_object = self.mtrie_stats_object;
        encoder.vms_mtrie_table_object = self.mtrie_table_object;
        encoder.vms_network_address_object = self.network_address_object;
        encoder.vms_nexthop_object = self.nexthop_object;
        encoder.vms_nexthop_component_object = self.nexthop_component_object;
        encoder.vms_nexthop_req_list_object = self.nexthop_req_list_object;
        encoder.vms_nexthop_req_encap_object = self.nexthop_req_encap_object;
        encoder.vms_nexthop_req_object = self.nexthop_req_object;
        encoder.vms_route_table_object = self.route_table_object;
        encoder.vms_route_req_mac_object = self.route_req_mac_object;
        encoder.vms_timer_object = self.timer_object;
        encoder.vms_usock_object = self.usock_object;
        encoder.vms_usock_poll_object = self.usock_poll_object;
        encoder.vms_usock_buf_object = self.usock_buf_object;
        encoder.vms_usock_iovec_object = self.usock_iovec_object;
        encoder.vms_vrouter_req_object = self.vrouter_req_object;
        encoder.vms_interface_fat_flow_config_object =
            self.interface_fat_flow_config_object;
        encoder.vms_qos_map_object = self.qos_map_object;
        encoder.vms_fc_object = self.fc_object;
        encoder.vms_interface_mirror_meta_object =
            self.interface_mirror_meta_object;
        encoder.vms_interface_req_mirror_meta_object =
            self.interface_req_mirror_meta_object;
        encoder.vms_interface_bridge_lock_object =
            self.interface_bridge_lock_object;
        encoder.vms_interface_queue_object = self.interface_queue_object;
        encoder.vms_interface_req_pbb_mac_object =
            self.interface_req_pbb_mac_object;
        encoder.vms_nexthop_req_bmac_object = self.nexthop_req_bmac_object;
        encoder.vms_interface_req_bridge_id_object =
            self.interface_req_bridge_id_object;
        encoder.vms_interface_fat_flow_ipv4_exclude_list_object =
            self.interface_fat_flow_ipv4_exclude_list_object;
        encoder.vms_interface_fat_flow_ipv6_exclude_list_object =
            self.interface_fat_flow_ipv6_exclude_list_object;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<MemStats, &'a str> {
        let decoder: vr_mem_stats_req = vr_mem_stats_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut vms: MemStats = MemStats::default();
                vms.op = decoder.h_op.try_into().unwrap();
                vms.rid = decoder.vms_rid;
                vms.alloced = decoder.vms_alloced;
                vms.freed = decoder.vms_freed;
                vms.assembler_table_object = decoder.vms_assembler_table_object;
                vms.bridge_mac_object = decoder.vms_bridge_mac_object;
                vms.btable_object = decoder.vms_btable_object;
                vms.build_info_object = decoder.vms_build_info_object;
                vms.defer_object = decoder.vms_defer_object;
                vms.drop_stats_object = decoder.vms_drop_stats_object;
                vms.drop_stats_req_object = decoder.vms_drop_stats_req_object;
                vms.flow_queue_object = decoder.vms_flow_queue_object;
                vms.flow_req_object = decoder.vms_flow_req_object;
                vms.flow_req_path_object = decoder.vms_flow_req_path_object;
                vms.flow_hold_stat_object = decoder.vms_flow_hold_stat_object;
                vms.flow_link_local_object = decoder.vms_flow_link_local_object;
                vms.flow_metadata_object = decoder.vms_flow_metadata_object;
                vms.flow_table_data_object = decoder.vms_flow_table_data_object;
                vms.flow_table_info_object = decoder.vms_flow_table_info_object;
                vms.fragment_object = decoder.vms_fragment_object;
                vms.fragment_queue_object = decoder.vms_fragment_queue_object;
                vms.fragment_queue_element_object =
                    decoder.vms_fragment_queue_element_object;
                vms.fragment_scanner_object =
                    decoder.vms_fragment_scanner_object;
                vms.hpacket_pool_object = decoder.vms_hpacket_pool_object;
                vms.htable_object = decoder.vms_htable_object;
                vms.interface_object = decoder.vms_interface_object;
                vms.interface_mac_object = decoder.vms_interface_mac_object;
                vms.interface_req_object = decoder.vms_interface_req_object;
                vms.interface_req_mac_object =
                    decoder.vms_interface_req_mac_object;
                vms.interface_req_name_object =
                    decoder.vms_interface_req_name_object;
                vms.interface_stats_object = decoder.vms_interface_stats_object;
                vms.interface_table_object = decoder.vms_interface_table_object;
                vms.interface_vrf_table_object =
                    decoder.vms_interface_vrf_table_object;
                vms.itable_object = decoder.vms_itable_object;
                vms.malloc_object = decoder.vms_malloc_object;
                vms.message_object = decoder.vms_message_object;
                vms.message_response_object =
                    decoder.vms_message_response_object;
                vms.message_dump_object = decoder.vms_message_dump_object;
                vms.mem_stats_req_object = decoder.vms_mem_stats_req_object;
                vms.mirror_object = decoder.vms_mirror_object;
                vms.mirror_table_object = decoder.vms_mirror_table_object;
                vms.mirror_meta_object = decoder.vms_mirror_meta_object;
                vms.mtrie_object = decoder.vms_mtrie_object;
                vms.mtrie_bucket_object = decoder.vms_mtrie_bucket_object;
                vms.mtrie_stats_object = decoder.vms_mtrie_stats_object;
                vms.mtrie_table_object = decoder.vms_mtrie_table_object;
                vms.network_address_object = decoder.vms_network_address_object;
                vms.nexthop_object = decoder.vms_nexthop_object;
                vms.nexthop_component_object =
                    decoder.vms_nexthop_component_object;
                vms.nexthop_req_list_object =
                    decoder.vms_nexthop_req_list_object;
                vms.nexthop_req_encap_object =
                    decoder.vms_nexthop_req_encap_object;
                vms.nexthop_req_object = decoder.vms_nexthop_req_object;
                vms.route_table_object = decoder.vms_route_table_object;
                vms.route_req_mac_object = decoder.vms_route_req_mac_object;
                vms.timer_object = decoder.vms_timer_object;
                vms.usock_object = decoder.vms_usock_object;
                vms.usock_poll_object = decoder.vms_usock_poll_object;
                vms.usock_buf_object = decoder.vms_usock_buf_object;
                vms.usock_iovec_object = decoder.vms_usock_iovec_object;
                vms.vrouter_req_object = decoder.vms_vrouter_req_object;
                vms.interface_fat_flow_config_object =
                    decoder.vms_interface_fat_flow_config_object;
                vms.qos_map_object = decoder.vms_qos_map_object;
                vms.fc_object = decoder.vms_fc_object;
                vms.interface_mirror_meta_object =
                    decoder.vms_interface_mirror_meta_object;
                vms.interface_req_mirror_meta_object =
                    decoder.vms_interface_req_mirror_meta_object;
                vms.interface_bridge_lock_object =
                    decoder.vms_interface_bridge_lock_object;
                vms.interface_queue_object = decoder.vms_interface_queue_object;
                vms.interface_req_pbb_mac_object =
                    decoder.vms_interface_req_pbb_mac_object;
                vms.nexthop_req_bmac_object =
                    decoder.vms_nexthop_req_bmac_object;
                vms.interface_req_bridge_id_object =
                    decoder.vms_interface_req_bridge_id_object;
                vms.interface_fat_flow_ipv4_exclude_list_object =
                    decoder.vms_interface_fat_flow_ipv4_exclude_list_object;
                vms.interface_fat_flow_ipv6_exclude_list_object =
                    decoder.vms_interface_fat_flow_ipv6_exclude_list_object;
                Ok(vms)
            }
        }
    }
}

#[cfg(test)]
mod test_vr_mem_stats {
    use crate::sandesh::SandeshOp;
    use crate::vr_mem_stats::MemStats;

    #[test]
    fn complex_request() {
        let mut vms: MemStats = MemStats::default();

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
        let vms: MemStats = MemStats::read(bytes).unwrap();

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
        let vms: MemStats = MemStats::default();
        let bytes = vms.write().unwrap();
        let vms: MemStats = MemStats::read(bytes).unwrap();

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
