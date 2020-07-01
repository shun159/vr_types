// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_drop_stats_req;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct DropStats {
    pub op: SandeshOp,
    pub rid: i16,
    pub core: i16,
    pub discard: i64,
    pub pcpu_stats_failure_status: i8,
    pub pull: i64,
    pub invalid_if: i64,
    pub invalid_arp: i64,
    pub trap_no_if: i64,
    pub nowhere_to_go: i64,
    pub flow_queue_limit_exceeded: i64,
    pub flow_no_memory: i64,
    pub flow_invalid_protocol: i64,
    pub flow_nat_no_rflow: i64,
    pub flow_action_drop: i64,
    pub flow_action_invalid: i64,
    pub flow_unusable: i64,
    pub flow_table_full: i64,
    pub interface_tx_discard: i64,
    pub interface_drop: i64,
    pub duplicated: i64,
    pub push: i64,
    pub ttl_exceeded: i64,
    pub invalid_nh: i64,
    pub invalid_label: i64,
    pub invalid_protocol: i64,
    pub interface_rx_discard: i64,
    pub invalid_mcast_source: i64,
    pub head_alloc_fail: i64,
    pub pcow_fail: i64,
    pub mcast_df_bit: i64,
    pub mcast_clone_fail: i64,
    pub no_memory: i64,
    pub rewrite_fail: i64,
    pub misc: i64,
    pub invalid_packet: i64,
    pub cksum_err: i64,
    pub no_fmd: i64,
    pub cloned_original: i64,
    pub invalid_vnid: i64,
    pub frag_err: i64,
    pub invalid_source: i64,
    pub l2_no_route: i64,
    pub fragment_queue_fail: i64,
    pub vlan_fwd_tx: i64,
    pub vlan_fwd_enq: i64,
    pub drop_new_flow: i64,
    pub flow_evict: i64,
    pub trap_original: i64,
    pub leaf_to_leaf: i64,
    pub bmac_isid_mismatch: i64,
    pub pkt_loop: i64,
    pub no_crypt_path: i64,
    pub invalid_hbs_pkt: i64,
}

impl Default for DropStats {
    fn default() -> DropStats {
        DropStats {
            op: SandeshOp::Add,
            rid: 0,
            core: 0,
            discard: 0,
            pcpu_stats_failure_status: 0,
            pull: 0,
            invalid_if: 0,
            invalid_arp: 0,
            trap_no_if: 0,
            nowhere_to_go: 0,
            flow_queue_limit_exceeded: 0,
            flow_no_memory: 0,
            flow_invalid_protocol: 0,
            flow_nat_no_rflow: 0,
            flow_action_drop: 0,
            flow_action_invalid: 0,
            flow_unusable: 0,
            flow_table_full: 0,
            interface_tx_discard: 0,
            interface_drop: 0,
            duplicated: 0,
            push: 0,
            ttl_exceeded: 0,
            invalid_nh: 0,
            invalid_label: 0,
            invalid_protocol: 0,
            interface_rx_discard: 0,
            invalid_mcast_source: 0,
            head_alloc_fail: 0,
            pcow_fail: 0,
            mcast_df_bit: 0,
            mcast_clone_fail: 0,
            no_memory: 0,
            rewrite_fail: 0,
            misc: 0,
            invalid_packet: 0,
            cksum_err: 0,
            no_fmd: 0,
            cloned_original: 0,
            invalid_vnid: 0,
            frag_err: 0,
            invalid_source: 0,
            l2_no_route: 0,
            fragment_queue_fail: 0,
            vlan_fwd_tx: 0,
            vlan_fwd_enq: 0,
            drop_new_flow: 0,
            flow_evict: 0,
            trap_original: 0,
            leaf_to_leaf: 0,
            bmac_isid_mismatch: 0,
            pkt_loop: 0,
            no_crypt_path: 0,
            invalid_hbs_pkt: 0,
        }
    }
}

impl DropStats {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_drop_stats_req = vr_drop_stats_req::new();
        encoder.h_op = self.op as u32;
        encoder.vds_rid = self.rid;
        encoder.vds_core = self.core;
        encoder.vds_discard = self.discard;
        encoder.vds_pcpu_stats_failure_status = self.pcpu_stats_failure_status;
        encoder.vds_pull = self.pull;
        encoder.vds_invalid_if = self.invalid_if;
        encoder.vds_invalid_arp = self.invalid_arp;
        encoder.vds_trap_no_if = self.trap_no_if;
        encoder.vds_nowhere_to_go = self.nowhere_to_go;
        encoder.vds_flow_queue_limit_exceeded = self.flow_queue_limit_exceeded;
        encoder.vds_flow_no_memory = self.flow_no_memory;
        encoder.vds_flow_invalid_protocol = self.flow_invalid_protocol;
        encoder.vds_flow_nat_no_rflow = self.flow_nat_no_rflow;
        encoder.vds_flow_action_drop = self.flow_action_drop;
        encoder.vds_flow_action_invalid = self.flow_action_invalid;
        encoder.vds_flow_unusable = self.flow_unusable;
        encoder.vds_flow_table_full = self.flow_table_full;
        encoder.vds_interface_tx_discard = self.interface_tx_discard;
        encoder.vds_interface_drop = self.interface_drop;
        encoder.vds_duplicated = self.duplicated;
        encoder.vds_push = self.push;
        encoder.vds_ttl_exceeded = self.ttl_exceeded;
        encoder.vds_invalid_nh = self.invalid_nh;
        encoder.vds_invalid_label = self.invalid_label;
        encoder.vds_invalid_protocol = self.invalid_protocol;
        encoder.vds_interface_rx_discard = self.interface_rx_discard;
        encoder.vds_invalid_mcast_source = self.invalid_mcast_source;
        encoder.vds_head_alloc_fail = self.head_alloc_fail;
        encoder.vds_pcow_fail = self.pcow_fail;
        encoder.vds_mcast_df_bit = self.mcast_df_bit;
        encoder.vds_mcast_clone_fail = self.mcast_clone_fail;
        encoder.vds_no_memory = self.no_memory;
        encoder.vds_rewrite_fail = self.rewrite_fail;
        encoder.vds_misc = self.misc;
        encoder.vds_invalid_packet = self.invalid_packet;
        encoder.vds_cksum_err = self.cksum_err;
        encoder.vds_no_fmd = self.no_fmd;
        encoder.vds_cloned_original = self.cloned_original;
        encoder.vds_invalid_vnid = self.invalid_vnid;
        encoder.vds_frag_err = self.frag_err;
        encoder.vds_invalid_source = self.invalid_source;
        encoder.vds_l2_no_route = self.l2_no_route;
        encoder.vds_fragment_queue_fail = self.fragment_queue_fail;
        encoder.vds_vlan_fwd_tx = self.vlan_fwd_tx;
        encoder.vds_vlan_fwd_enq = self.vlan_fwd_enq;
        encoder.vds_drop_new_flow = self.drop_new_flow;
        encoder.vds_flow_evict = self.flow_evict;
        encoder.vds_trap_original = self.trap_original;
        encoder.vds_leaf_to_leaf = self.leaf_to_leaf;
        encoder.vds_bmac_isid_mismatch = self.bmac_isid_mismatch;
        encoder.vds_pkt_loop = self.pkt_loop;
        encoder.vds_no_crypt_path = self.no_crypt_path;
        encoder.vds_invalid_hbs_pkt = self.invalid_hbs_pkt;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<DropStats, &'a str> {
        let decoder: vr_drop_stats_req = vr_drop_stats_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut vds: DropStats = DropStats::default();
                vds.op = decoder.h_op.try_into().unwrap();
                vds.rid = decoder.vds_rid;
                vds.core = decoder.vds_core;
                vds.discard = decoder.vds_discard;
                vds.pcpu_stats_failure_status =
                    decoder.vds_pcpu_stats_failure_status;
                vds.pull = decoder.vds_pull;
                vds.invalid_if = decoder.vds_invalid_if;
                vds.invalid_arp = decoder.vds_invalid_arp;
                vds.trap_no_if = decoder.vds_trap_no_if;
                vds.nowhere_to_go = decoder.vds_nowhere_to_go;
                vds.flow_queue_limit_exceeded =
                    decoder.vds_flow_queue_limit_exceeded;
                vds.flow_no_memory = decoder.vds_flow_no_memory;
                vds.flow_invalid_protocol = decoder.vds_flow_invalid_protocol;
                vds.flow_nat_no_rflow = decoder.vds_flow_nat_no_rflow;
                vds.flow_action_drop = decoder.vds_flow_action_drop;
                vds.flow_action_invalid = decoder.vds_flow_action_invalid;
                vds.flow_unusable = decoder.vds_flow_unusable;
                vds.flow_table_full = decoder.vds_flow_table_full;
                vds.interface_tx_discard = decoder.vds_interface_tx_discard;
                vds.interface_drop = decoder.vds_interface_drop;
                vds.duplicated = decoder.vds_duplicated;
                vds.push = decoder.vds_push;
                vds.ttl_exceeded = decoder.vds_ttl_exceeded;
                vds.invalid_nh = decoder.vds_invalid_nh;
                vds.invalid_label = decoder.vds_invalid_label;
                vds.invalid_protocol = decoder.vds_invalid_protocol;
                vds.interface_rx_discard = decoder.vds_interface_rx_discard;
                vds.invalid_mcast_source = decoder.vds_invalid_mcast_source;
                vds.head_alloc_fail = decoder.vds_head_alloc_fail;
                vds.pcow_fail = decoder.vds_pcow_fail;
                vds.mcast_df_bit = decoder.vds_mcast_df_bit;
                vds.mcast_clone_fail = decoder.vds_mcast_clone_fail;
                vds.no_memory = decoder.vds_no_memory;
                vds.rewrite_fail = decoder.vds_rewrite_fail;
                vds.misc = decoder.vds_misc;
                vds.invalid_packet = decoder.vds_invalid_packet;
                vds.cksum_err = decoder.vds_cksum_err;
                vds.no_fmd = decoder.vds_no_fmd;
                vds.cloned_original = decoder.vds_cloned_original;
                vds.invalid_vnid = decoder.vds_invalid_vnid;
                vds.frag_err = decoder.vds_frag_err;
                vds.invalid_source = decoder.vds_invalid_source;
                vds.l2_no_route = decoder.vds_l2_no_route;
                vds.fragment_queue_fail = decoder.vds_fragment_queue_fail;
                vds.vlan_fwd_tx = decoder.vds_vlan_fwd_tx;
                vds.vlan_fwd_enq = decoder.vds_vlan_fwd_enq;
                vds.drop_new_flow = decoder.vds_drop_new_flow;
                vds.flow_evict = decoder.vds_flow_evict;
                vds.trap_original = decoder.vds_trap_original;
                vds.leaf_to_leaf = decoder.vds_leaf_to_leaf;
                vds.bmac_isid_mismatch = decoder.vds_bmac_isid_mismatch;
                vds.pkt_loop = decoder.vds_pkt_loop;
                vds.no_crypt_path = decoder.vds_no_crypt_path;
                vds.invalid_hbs_pkt = decoder.vds_invalid_hbs_pkt;
                Ok(vds)
            }
        }
    }
}

#[cfg(test)]
mod test_vr_drop_stats {
    use crate::sandesh::SandeshOp;
    use crate::vr_drop_stats::DropStats;

    #[test]
    fn complex_request() {
        let mut vds: DropStats = DropStats::default();

        vds.rid = 0;
        vds.core = 1;
        vds.discard = 2;
        vds.pcpu_stats_failure_status = 3;
        vds.pull = 4;
        vds.invalid_if = 5;
        vds.invalid_arp = 6;
        vds.trap_no_if = 7;
        vds.nowhere_to_go = 8;
        vds.flow_queue_limit_exceeded = 9;
        vds.flow_no_memory = 10;
        vds.flow_invalid_protocol = 11;
        vds.flow_nat_no_rflow = 12;
        vds.flow_action_drop = 13;
        vds.flow_action_invalid = 14;
        vds.flow_unusable = 15;
        vds.flow_table_full = 16;
        vds.interface_tx_discard = 17;
        vds.interface_drop = 18;
        vds.duplicated = 19;
        vds.push = 20;
        vds.ttl_exceeded = 21;
        vds.invalid_nh = 22;
        vds.invalid_label = 23;
        vds.invalid_protocol = 24;
        vds.interface_rx_discard = 25;
        vds.invalid_mcast_source = 26;
        vds.head_alloc_fail = 27;
        vds.pcow_fail = 28;
        vds.mcast_df_bit = 29;
        vds.mcast_clone_fail = 30;
        vds.no_memory = 31;
        vds.rewrite_fail = 32;
        vds.misc = 33;
        vds.invalid_packet = 34;
        vds.cksum_err = 35;
        vds.no_fmd = 36;
        vds.cloned_original = 37;
        vds.invalid_vnid = 38;
        vds.frag_err = 39;
        vds.invalid_source = 40;
        vds.l2_no_route = 41;
        vds.fragment_queue_fail = 42;
        vds.vlan_fwd_tx = 43;
        vds.vlan_fwd_enq = 44;
        vds.drop_new_flow = 45;
        vds.flow_evict = 46;
        vds.trap_original = 47;
        vds.leaf_to_leaf = 48;
        vds.bmac_isid_mismatch = 49;
        vds.pkt_loop = 50;
        vds.no_crypt_path = 51;
        vds.invalid_hbs_pkt = 52;

        let bytes = vds.write().unwrap();
        let vds: DropStats = DropStats::read(bytes).unwrap();

        assert_eq!(vds.rid, 0);
        assert_eq!(vds.core, 1);
        assert_eq!(vds.discard, 2);
        assert_eq!(vds.pcpu_stats_failure_status, 3);
        assert_eq!(vds.pull, 4);
        assert_eq!(vds.invalid_if, 5);
        assert_eq!(vds.invalid_arp, 6);
        assert_eq!(vds.trap_no_if, 7);
        assert_eq!(vds.nowhere_to_go, 8);
        assert_eq!(vds.flow_queue_limit_exceeded, 9);
        assert_eq!(vds.flow_no_memory, 10);
        assert_eq!(vds.flow_invalid_protocol, 11);
        assert_eq!(vds.flow_nat_no_rflow, 12);
        assert_eq!(vds.flow_action_drop, 13);
        assert_eq!(vds.flow_action_invalid, 14);
        assert_eq!(vds.flow_unusable, 15);
        assert_eq!(vds.flow_table_full, 16);
        assert_eq!(vds.interface_tx_discard, 17);
        assert_eq!(vds.interface_drop, 18);
        assert_eq!(vds.duplicated, 19);
        assert_eq!(vds.push, 20);
        assert_eq!(vds.ttl_exceeded, 21);
        assert_eq!(vds.invalid_nh, 22);
        assert_eq!(vds.invalid_label, 23);
        assert_eq!(vds.invalid_protocol, 24);
        assert_eq!(vds.interface_rx_discard, 25);
        assert_eq!(vds.invalid_mcast_source, 26);
        assert_eq!(vds.head_alloc_fail, 27);
        assert_eq!(vds.pcow_fail, 28);
        assert_eq!(vds.mcast_df_bit, 29);
        assert_eq!(vds.mcast_clone_fail, 30);
        assert_eq!(vds.no_memory, 31);
        assert_eq!(vds.rewrite_fail, 32);
        assert_eq!(vds.misc, 33);
        assert_eq!(vds.invalid_packet, 34);
        assert_eq!(vds.cksum_err, 35);
        assert_eq!(vds.no_fmd, 36);
        assert_eq!(vds.cloned_original, 37);
        assert_eq!(vds.invalid_vnid, 38);
        assert_eq!(vds.frag_err, 39);
        assert_eq!(vds.invalid_source, 40);
        assert_eq!(vds.l2_no_route, 41);
        assert_eq!(vds.fragment_queue_fail, 42);
        assert_eq!(vds.vlan_fwd_tx, 43);
        assert_eq!(vds.vlan_fwd_enq, 44);
        assert_eq!(vds.drop_new_flow, 45);
        assert_eq!(vds.flow_evict, 46);
        assert_eq!(vds.trap_original, 47);
        assert_eq!(vds.leaf_to_leaf, 48);
        assert_eq!(vds.bmac_isid_mismatch, 49);
        assert_eq!(vds.pkt_loop, 50);
        assert_eq!(vds.no_crypt_path, 51);
        assert_eq!(vds.invalid_hbs_pkt, 52);
    }

    #[test]
    fn empty_request() {
        let vds: DropStats = DropStats::default();
        let bytes = vds.write().unwrap();
        let vds: DropStats = DropStats::read(bytes).unwrap();
        assert_eq!(vds.op, SandeshOp::Add);
        assert_eq!(vds.rid, 0);
        assert_eq!(vds.core, 0);
        assert_eq!(vds.discard, 0);
        assert_eq!(vds.pcpu_stats_failure_status, 0);
        assert_eq!(vds.pull, 0);
        assert_eq!(vds.invalid_if, 0);
        assert_eq!(vds.invalid_arp, 0);
        assert_eq!(vds.trap_no_if, 0);
        assert_eq!(vds.nowhere_to_go, 0);
        assert_eq!(vds.flow_queue_limit_exceeded, 0);
        assert_eq!(vds.flow_no_memory, 0);
        assert_eq!(vds.flow_invalid_protocol, 0);
        assert_eq!(vds.flow_nat_no_rflow, 0);
        assert_eq!(vds.flow_action_drop, 0);
        assert_eq!(vds.flow_action_invalid, 0);
        assert_eq!(vds.flow_unusable, 0);
        assert_eq!(vds.flow_table_full, 0);
        assert_eq!(vds.interface_tx_discard, 0);
        assert_eq!(vds.interface_drop, 0);
        assert_eq!(vds.duplicated, 0);
        assert_eq!(vds.push, 0);
        assert_eq!(vds.ttl_exceeded, 0);
        assert_eq!(vds.invalid_nh, 0);
        assert_eq!(vds.invalid_label, 0);
        assert_eq!(vds.invalid_protocol, 0);
        assert_eq!(vds.interface_rx_discard, 0);
        assert_eq!(vds.invalid_mcast_source, 0);
        assert_eq!(vds.head_alloc_fail, 0);
        assert_eq!(vds.pcow_fail, 0);
        assert_eq!(vds.mcast_df_bit, 0);
        assert_eq!(vds.mcast_clone_fail, 0);
        assert_eq!(vds.no_memory, 0);
        assert_eq!(vds.rewrite_fail, 0);
        assert_eq!(vds.misc, 0);
        assert_eq!(vds.invalid_packet, 0);
        assert_eq!(vds.cksum_err, 0);
        assert_eq!(vds.no_fmd, 0);
        assert_eq!(vds.cloned_original, 0);
        assert_eq!(vds.invalid_vnid, 0);
        assert_eq!(vds.frag_err, 0);
        assert_eq!(vds.invalid_source, 0);
        assert_eq!(vds.l2_no_route, 0);
        assert_eq!(vds.fragment_queue_fail, 0);
        assert_eq!(vds.vlan_fwd_tx, 0);
        assert_eq!(vds.vlan_fwd_enq, 0);
        assert_eq!(vds.drop_new_flow, 0);
        assert_eq!(vds.flow_evict, 0);
        assert_eq!(vds.trap_original, 0);
        assert_eq!(vds.leaf_to_leaf, 0);
        assert_eq!(vds.bmac_isid_mismatch, 0);
        assert_eq!(vds.pkt_loop, 0);
        assert_eq!(vds.no_crypt_path, 0);
        assert_eq!(vds.invalid_hbs_pkt, 0);
    }
}
