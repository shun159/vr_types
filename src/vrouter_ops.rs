// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vrouter_ops;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
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
