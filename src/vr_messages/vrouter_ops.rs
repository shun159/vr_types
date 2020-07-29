// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::error::CodecError;
use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vrouter_ops;
use crate::utils;
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct VrouterOps {
    pub op: SandeshOp,
    pub read_length: usize,
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
    // Close flow on TCP rst
    pub close_flow_on_tcp_rst: i8,
}

impl VrouterOps {
    pub fn write(&self) -> Result<Vec<u8>, CodecError> {
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
        encoder.vo_vif_oflow_bridge_entries = self.vif_overflow_flow_bridge_entries;
        encoder.vo_packet_dump = self.packet_dump;
        encoder.vo_pkt_droplog_bufsz = self.pkt_droplog_bufsz;
        encoder.vo_pkt_droplog_buf_en = self.pkt_droplog_buf_en;
        encoder.vo_pkt_droplog_en = self.pkt_droplog_en;
        encoder.vo_pkt_droplog_min_en = self.pkt_droplog_min_en;
        encoder.vo_close_flow_on_tcp_rst = self.close_flow_on_tcp_rst;
        encoder.write()
    }

    pub fn read(buf: Vec<u8>) -> Result<VrouterOps, CodecError> {
        let decoder: vrouter_ops = vrouter_ops::new();
        let rxfer = decoder.read(&buf)?;
        let mut vo: VrouterOps = VrouterOps::default();
        vo.read_length = rxfer as usize;
        vo.op = decoder.h_op.try_into().unwrap();
        vo.rid = decoder.vo_rid;
        vo.mpls_labels = decoder.vo_mpls_labels;
        vo.nexthops = decoder.vo_nexthops;
        vo.bridge_entries = decoder.vo_bridge_entries;
        vo.overflow_flow_bridge_entries = decoder.vo_oflow_bridge_entries;
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
        vo.bridge_used_overflow_entries = decoder.vo_bridge_used_oentries;
        vo.burst_tokens = decoder.vo_burst_tokens;
        vo.burst_interval = decoder.vo_burst_interval;
        vo.burst_step = decoder.vo_burst_step;
        vo.memory_alloc_checks = decoder.vo_memory_alloc_checks;
        vo.priority_tagging = decoder.vo_priority_tagging;
        vo.vif_bridge_entries = decoder.vo_vif_bridge_entries;
        vo.vif_overflow_flow_bridge_entries = decoder.vo_vif_oflow_bridge_entries;
        vo.packet_dump = decoder.vo_packet_dump;
        vo.pkt_droplog_bufsz = decoder.vo_pkt_droplog_bufsz;
        vo.pkt_droplog_buf_en = decoder.vo_pkt_droplog_buf_en;
        vo.pkt_droplog_en = decoder.vo_pkt_droplog_en;
        vo.pkt_droplog_min_en = decoder.vo_pkt_droplog_min_en;
        vo.close_flow_on_tcp_rst = decoder.vo_close_flow_on_tcp_rst;
        Ok(vo)
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
