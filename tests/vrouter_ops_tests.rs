// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vrouter_ops {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vrouter_ops::VrouterOps;

    #[test]
    fn empty_requset() {
        let vo: VrouterOps = VrouterOps::default();
        let bytes = vo.write().unwrap();
        let vo: VrouterOps = VrouterOps::read(bytes).unwrap();
        assert_eq!(vo.op, SandeshOp::Add);
        assert_eq!(vo.rid, 0);
        assert_eq!(vo.mpls_labels, 0);
        assert_eq!(vo.nexthops, 0);
        assert_eq!(vo.bridge_entries, 0);
        assert_eq!(vo.overflow_flow_bridge_entries, 0);
        assert_eq!(vo.flow_entries, 0);
        assert_eq!(vo.overflow_flow_entries, 0);
        assert_eq!(vo.interfaces, 0);
        assert_eq!(vo.mirror_entries, 0);
        assert_eq!(vo.vrfs, 0);
        assert_eq!(vo.build_info, "".to_string());
        assert_eq!(vo.log_level, 0);
        assert_eq!(vo.log_type_enable, vec![]);
        assert_eq!(vo.log_type_disable, vec![]);
        assert_eq!(vo.perfr, 0);
        assert_eq!(vo.perfs, 0);
        assert_eq!(vo.from_vm_mss_adj, 0);
        assert_eq!(vo.to_vm_mss_adj, 0);
        assert_eq!(vo.perfr1, 0);
        assert_eq!(vo.perfr2, 0);
        assert_eq!(vo.perfr3, 0);
        assert_eq!(vo.perfp, 0);
        assert_eq!(vo.perfq1, 0);
        assert_eq!(vo.perfq2, 0);
        assert_eq!(vo.perfq3, 0);
        assert_eq!(vo.udp_coff, 0);
        assert_eq!(vo.flow_hold_limit, 0);
        assert_eq!(vo.mudp, 0);
        assert_eq!(vo.flow_used_entries, 0);
        assert_eq!(vo.flow_used_overflow_entries, 0);
        assert_eq!(vo.bridge_used_entries, 0);
        assert_eq!(vo.bridge_used_overflow_entries, 0);
        assert_eq!(vo.burst_tokens, 0);
        assert_eq!(vo.burst_interval, 0);
        assert_eq!(vo.burst_step, 0);
        assert_eq!(vo.memory_alloc_checks, 0);
        assert_eq!(vo.priority_tagging, 0);
        assert_eq!(vo.vif_bridge_entries, 0);
        assert_eq!(vo.vif_overflow_flow_bridge_entries, 0);
        assert_eq!(vo.packet_dump, 0);
        assert_eq!(vo.pkt_droplog_bufsz, 0);
        assert_eq!(vo.pkt_droplog_buf_en, 0);
        assert_eq!(vo.pkt_droplog_en, 0);
        assert_eq!(vo.pkt_droplog_min_en, 0);
    }

    #[test]
    fn complex_requset() {
        let mut vo: VrouterOps = VrouterOps::default();
        vo.op = SandeshOp::Dump;
        vo.rid = 1;
        vo.mpls_labels = 1;
        vo.nexthops = 1;
        vo.bridge_entries = 1;
        vo.overflow_flow_bridge_entries = 1;
        vo.flow_entries = 1;
        vo.overflow_flow_entries = 1;
        vo.interfaces = 1;
        vo.mirror_entries = 1;
        vo.vrfs = 1;
        vo.build_info = "Linux shun159.localhost".to_string();
        vo.log_level = 1;
        vo.log_type_enable = vec![1, 2, 3, 4, 5];
        vo.log_type_disable = vec![1, 2, 3, 4, 5];
        vo.perfr = 1;
        vo.perfs = 1;
        vo.from_vm_mss_adj = 1;
        vo.to_vm_mss_adj = 1;
        vo.perfr1 = 1;
        vo.perfr2 = 1;
        vo.perfr3 = 1;
        vo.perfp = 1;
        vo.perfq1 = 1;
        vo.perfq2 = 1;
        vo.perfq3 = 1;
        vo.udp_coff = 1;
        vo.flow_hold_limit = 1;
        vo.mudp = 1;
        vo.flow_used_entries = 1;
        vo.flow_used_overflow_entries = 1;
        vo.bridge_used_entries = 1;
        vo.bridge_used_overflow_entries = 1;
        vo.burst_tokens = 1;
        vo.burst_interval = 1;
        vo.burst_step = 1;
        vo.memory_alloc_checks = 1;
        vo.priority_tagging = 1;
        vo.vif_bridge_entries = 1;
        vo.vif_overflow_flow_bridge_entries = 1;
        vo.packet_dump = 1;
        vo.pkt_droplog_bufsz = 1;
        vo.pkt_droplog_buf_en = 1;
        vo.pkt_droplog_en = 1;
        vo.pkt_droplog_min_en = 1;

        let bytes = vo.write().unwrap();
        let vo: VrouterOps = VrouterOps::read(bytes).unwrap();

        assert_eq!(vo.op, SandeshOp::Dump);
        assert_eq!(vo.rid, 1);
        assert_eq!(vo.mpls_labels, 1);
        assert_eq!(vo.nexthops, 1);
        assert_eq!(vo.bridge_entries, 1);
        assert_eq!(vo.overflow_flow_bridge_entries, 1);
        assert_eq!(vo.flow_entries, 1);
        assert_eq!(vo.overflow_flow_entries, 1);
        assert_eq!(vo.interfaces, 1);
        assert_eq!(vo.mirror_entries, 1);
        assert_eq!(vo.vrfs, 1);
        assert_eq!(vo.build_info, "Linux shun159.localhost".to_string());
        assert_eq!(vo.log_level, 1);
        assert_eq!(vo.log_type_enable, vec![1, 2, 3, 4, 5]);
        assert_eq!(vo.log_type_disable, vec![1, 2, 3, 4, 5]);
        assert_eq!(vo.perfr, 1);
        assert_eq!(vo.perfs, 1);
        assert_eq!(vo.from_vm_mss_adj, 1);
        assert_eq!(vo.to_vm_mss_adj, 1);
        assert_eq!(vo.perfr1, 1);
        assert_eq!(vo.perfr2, 1);
        assert_eq!(vo.perfr3, 1);
        assert_eq!(vo.perfp, 1);
        assert_eq!(vo.perfq1, 1);
        assert_eq!(vo.perfq2, 1);
        assert_eq!(vo.perfq3, 1);
        assert_eq!(vo.udp_coff, 1);
        assert_eq!(vo.flow_hold_limit, 1);
        assert_eq!(vo.mudp, 1);
        assert_eq!(vo.flow_used_entries, 1);
        assert_eq!(vo.flow_used_overflow_entries, 1);
        assert_eq!(vo.bridge_used_entries, 1);
        assert_eq!(vo.bridge_used_overflow_entries, 1);
        assert_eq!(vo.burst_tokens, 1);
        assert_eq!(vo.burst_interval, 1);
        assert_eq!(vo.burst_step, 1);
        assert_eq!(vo.memory_alloc_checks, 1);
        assert_eq!(vo.priority_tagging, 1);
        assert_eq!(vo.vif_bridge_entries, 1);
        assert_eq!(vo.vif_overflow_flow_bridge_entries, 1);
        assert_eq!(vo.packet_dump, 1);
        assert_eq!(vo.pkt_droplog_bufsz, 1);
        assert_eq!(vo.pkt_droplog_buf_en, 1);
        assert_eq!(vo.pkt_droplog_en, 1);
        assert_eq!(vo.pkt_droplog_min_en, 1);
    }
}
