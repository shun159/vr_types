// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::utils;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vrouter_ops;
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VrouterOps {
    pub op: SandeshOp,
    pub rid: i32,
    // MPLS Lables limit
    pub mpls_labels: i32,
    // NextHops limit
    pub nexthops: i32,
    // Bridge Table limit
    pub bridge_entries: i32,
    // Bridge Table Overflow limit
    pub overflow_flow_bridge_entries: i32,
    // Flow table limit
    pub flow_entries: i32,
    // Flow table overflow limit
    pub overflow_flow_entries: i32,
    // Interface limit
    pub interfaces: i32,
    // Mirror entries limit
    pub mirror_entries: i32,
    // VRF tables limit
    pub vrfs: i32,
    pub build_info: String,
    pub log_level: u32,
    pub log_type_enable: Vec<i32>,
    pub log_type_disable: Vec<i32>,
    // Performance tweaks: GRO. Known as perfr
    pub perfr: i32,
    // Performance tweaks: GSO. Known as perfs
    pub perfs: i32,
    // TCP MSS on packets from VM
    pub from_vm_mss_adj: i32,
    // TCP MSS on packets sent to VM
    pub to_vm_mss_adj: i32,
    // RPS after pulling inner hdr (perfr1)
    pub perfr1: i32,
    // RPS after GRO on pkt1 (perfr2)
    pub perfr2: i32,
    // RPS from phys rx handler (perfr3)
    pub perfr3: i32,
    // Pull inner header (faster version)
    pub perfp: i32,
    // CPU to send pkts to, if perfr1 set.
    pub perfq1: i32,
    // CPU to send pkts to, if perfr2 set.
    pub perfq2: i32,
    // CPU to send pkts to, if perfr3 set.
    pub perfq3: i32,
    // NIC cksum offload for outer UDP hdr
    pub udp_coff: i32,
    // Flow hold limit
    pub flow_hold_limit: i32,
    // MPLS over UDP globally
    pub mudp: i32,
    // Used Flow entries
    pub flow_used_entries: u32,
    // Used Over Flow entries
    pub flow_used_overflow_entries: u32,
    // Used Bridge entries
    pub bridge_used_entries: u32,
    // Used Over Flow bridge entries
    pub bridge_used_overflow_entries: u32,
    // Burst Total Tokens
    pub burst_tokens: i32,
    // Burst Interval
    pub burst_interval: i32,
    // Burst Step
    pub burst_step: i32,
    // Memory allocation checks
    pub memory_alloc_checks: i32,
    // NIC Priority Tagging
    pub priority_tagging: u32,
    // Vif Bridge Table limit
    pub vif_bridge_entries: i32,
    // Vif Bridge table Overflow limit
    pub vif_overflow_flow_bridge_entries: i32,
    // Packet dump
    pub packet_dump: i32,
    // Vrouter pkt drop log buf size
    pub pkt_droplog_bufsz: i32,
    // Enable/Disable pkt drop debug log infra
    pub pkt_droplog_buf_en: i8,
    // Vrouter packet drop log enable
    pub pkt_droplog_en: i8,
    // Vrouter Packet drop log minimum enable
    pub pkt_droplog_min_en: i8,
}

impl Default for VrouterOps {
    fn default() -> VrouterOps {
        VrouterOps {
            op: SandeshOp::Add,
            rid: 0,
            mpls_labels: 0,
            nexthops: 0,
            bridge_entries: 0,
            overflow_flow_bridge_entries: 0,
            flow_entries: 0,
            overflow_flow_entries: 0,
            interfaces: 0,
            mirror_entries: 0,
            vrfs: 0,
            build_info: "".to_string(),
            log_level: 0,
            log_type_enable: vec![],
            log_type_disable: vec![],
            perfr: 0,
            perfs: 0,
            from_vm_mss_adj: 0,
            to_vm_mss_adj: 0,
            perfr1: 0,
            perfr2: 0,
            perfr3: 0,
            perfp: 0,
            perfq1: 0,
            perfq2: 0,
            perfq3: 0,
            udp_coff: 0,
            flow_hold_limit: 0,
            mudp: 0,
            flow_used_entries: 0,
            flow_used_overflow_entries: 0,
            bridge_used_entries: 0,
            bridge_used_overflow_entries: 0,
            burst_tokens: 0,
            burst_interval: 0,
            burst_step: 0,
            memory_alloc_checks: 0,
            priority_tagging: 0,
            vif_bridge_entries: 0,
            vif_overflow_flow_bridge_entries: 0,
            packet_dump: 0,
            pkt_droplog_bufsz: 0,
            pkt_droplog_buf_en: 0,
            pkt_droplog_en: 0,
            pkt_droplog_min_en: 0,
        }
    }
}

impl VrouterOps {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vrouter_ops = vrouter_ops::new();
        encoder.h_op = self.op as u32;
        encoder.vo_rid = self.rid;
        encoder.vo_mpls_labels = self.mpls_labels;
        encoder.vo_nexthops = self.nexthops;
        encoder.vo_bridge_entries = self.bridge_entries;
        encoder.vo_oflow_bridge_entries = self.overflow_flow_bridge_entries;
        encoder.vo_flow_entries = self.flow_entries;
        encoder.vo_oflow_entries = self.overflow_flow_entries;
        encoder.vo_interfaces = self.interfaces;
        encoder.vo_mirror_entries = self.mirror_entries;
        encoder.vo_vrfs = self.vrfs;
        encoder.vo_build_info = Self::write_cstring(&self.build_info);
        encoder.vo_log_level = self.log_level;
        encoder.vo_log_type_enable =
            utils::into_mut_ptr(&self.log_type_enable) as *mut i32;
        encoder.vo_log_type_enable_size = self.log_type_enable.len() as u32;
        encoder.vo_log_type_disable =
            utils::into_mut_ptr(&self.log_type_disable) as *mut i32;
        encoder.vo_log_type_disable_size = self.log_type_disable.len() as u32;
        encoder.vo_perfr = self.perfr;
        encoder.vo_perfs = self.perfs;
        encoder.vo_from_vm_mss_adj = self.from_vm_mss_adj;
        encoder.vo_to_vm_mss_adj = self.to_vm_mss_adj;
        encoder.vo_perfr1 = self.perfr1;
        encoder.vo_perfr2 = self.perfr2;
        encoder.vo_perfr3 = self.perfr3;
        encoder.vo_perfp = self.perfp;
        encoder.vo_perfq1 = self.perfq1;
        encoder.vo_perfq2 = self.perfq2;
        encoder.vo_perfq3 = self.perfq3;
        encoder.vo_udp_coff = self.udp_coff;
        encoder.vo_flow_hold_limit = self.flow_hold_limit;
        encoder.vo_mudp = self.mudp;
        encoder.vo_flow_used_entries = self.flow_used_entries;
        encoder.vo_flow_used_oentries = self.flow_used_overflow_entries;
        encoder.vo_bridge_used_entries = self.bridge_used_entries;
        encoder.vo_bridge_used_oentries = self.bridge_used_overflow_entries;
        encoder.vo_burst_tokens = self.burst_tokens;
        encoder.vo_burst_interval = self.burst_interval;
        encoder.vo_burst_step = self.burst_step;
        encoder.vo_memory_alloc_checks = self.memory_alloc_checks;
        encoder.vo_priority_tagging = self.priority_tagging;
        encoder.vo_vif_bridge_entries = self.vif_bridge_entries;
        encoder.vo_vif_oflow_bridge_entries =
            self.vif_overflow_flow_bridge_entries;
        encoder.vo_packet_dump = self.packet_dump;
        encoder.vo_pkt_droplog_bufsz = self.pkt_droplog_bufsz;
        encoder.vo_pkt_droplog_buf_en = self.pkt_droplog_buf_en;
        encoder.vo_pkt_droplog_en = self.pkt_droplog_en;
        encoder.vo_pkt_droplog_min_en = self.pkt_droplog_min_en;

        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<VrouterOps, &'a str> {
        let decoder: vrouter_ops = vrouter_ops::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut vo: VrouterOps = VrouterOps::default();
                vo.op = decoder.h_op.try_into().unwrap();
                vo.rid = decoder.vo_rid;
                vo.mpls_labels = decoder.vo_mpls_labels;
                vo.nexthops = decoder.vo_nexthops;
                vo.bridge_entries = decoder.vo_bridge_entries;
                vo.overflow_flow_bridge_entries =
                    decoder.vo_oflow_bridge_entries;
                vo.flow_entries = decoder.vo_flow_entries;
                vo.overflow_flow_entries = decoder.vo_oflow_entries;
                vo.interfaces = decoder.vo_interfaces;
                vo.mirror_entries = decoder.vo_mirror_entries;
                vo.vrfs = decoder.vo_vrfs;
                vo.build_info = Self::read_cstring(decoder.vo_build_info);
                vo.log_level = decoder.vo_log_level;
                vo.log_type_enable = utils::free_buf(
                    decoder.vo_log_type_enable,
                    decoder.vo_log_type_enable_size as usize,
                );
                vo.log_type_disable = utils::free_buf(
                    decoder.vo_log_type_disable,
                    decoder.vo_log_type_disable_size as usize,
                );
                vo.perfr = decoder.vo_perfr;
                vo.perfs = decoder.vo_perfs;
                vo.from_vm_mss_adj = decoder.vo_from_vm_mss_adj;
                vo.to_vm_mss_adj = decoder.vo_to_vm_mss_adj;
                vo.perfr1 = decoder.vo_perfr1;
                vo.perfr2 = decoder.vo_perfr2;
                vo.perfr3 = decoder.vo_perfr3;
                vo.perfp = decoder.vo_perfp;
                vo.perfq1 = decoder.vo_perfq1;
                vo.perfq2 = decoder.vo_perfq2;
                vo.perfq3 = decoder.vo_perfq3;
                vo.udp_coff = decoder.vo_udp_coff;
                vo.flow_hold_limit = decoder.vo_flow_hold_limit;
                vo.mudp = decoder.vo_mudp;
                vo.flow_used_entries = decoder.vo_flow_used_entries;
                vo.flow_used_overflow_entries = decoder.vo_flow_used_oentries;
                vo.bridge_used_entries = decoder.vo_bridge_used_entries;
                vo.bridge_used_overflow_entries =
                    decoder.vo_bridge_used_oentries;
                vo.burst_tokens = decoder.vo_burst_tokens;
                vo.burst_interval = decoder.vo_burst_interval;
                vo.burst_step = decoder.vo_burst_step;
                vo.memory_alloc_checks = decoder.vo_memory_alloc_checks;
                vo.priority_tagging = decoder.vo_priority_tagging;
                vo.vif_bridge_entries = decoder.vo_vif_bridge_entries;
                vo.vif_overflow_flow_bridge_entries =
                    decoder.vo_vif_oflow_bridge_entries;
                vo.packet_dump = decoder.vo_packet_dump;
                vo.pkt_droplog_bufsz = decoder.vo_pkt_droplog_bufsz;
                vo.pkt_droplog_buf_en = decoder.vo_pkt_droplog_buf_en;
                vo.pkt_droplog_en = decoder.vo_pkt_droplog_en;
                vo.pkt_droplog_min_en = decoder.vo_pkt_droplog_min_en;
                Ok(vo)
            }
        }
    }

    // private functions

    fn write_cstring(s: &String) -> *mut c_char {
        let cs = CString::new(s.as_str()).unwrap();
        cs.into_raw()
    }

    fn read_cstring(ptr: *mut c_char) -> String {
        unsafe {
            if ptr.is_null() {
                return String::from("");
            }
            let cs = CString::from_raw(ptr);
            cs.to_string_lossy().into_owned()
        }
    }
}

#[cfg(test)]
mod test_vrouter_ops {
    use crate::sandesh::SandeshOp;
    use crate::vrouter_ops::VrouterOps;

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
