// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use eui48::MacAddress;
use std::net::{Ipv4Addr, Ipv6Addr};

pub const VIF_MAX_MIRROR_MD_SIZE: u32 = 0xFF;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IfType {
    Host       = 0,
    Agent      = 1,
    Physical   = 2,
    Virtual    = 3,
    XenLlHost  = 4,
    Gateway    = 5,
    VirualVlan = 6,
    Stats      = 7,
    Vlan       = 8,
    Monitoring = 9,
    Max        = 10
}

#[derive(Debug, PartialEq)]
pub enum IfFlag {
    PolicyEnabled    = 0x0000001,
    Xconnect         = 0x0000002,
    ServiceIf        = 0x0000004,
    MirrorRx         = 0x0000008,
    MirrorTx         = 0x0000010,
    TxCsumOffload    = 0x0000020,
    L3Enabled        = 0x0000040,
    L2Enabled        = 0x0000080,
    DhcpEnabled      = 0x0000100,
    // The physical interface corresponds to a vhost interface
    VhostPhys        = 0x0000200,
    Promiscous       = 0x0000400,
    // untagged packet should be treated as packets with tag 0.
    NativeVlanTag    = 0x0000800,
    NoArpProxy       = 0x0001000,
    Pmd              = 0x0002000,
    // The physical interface supports hardware filtering.
    FilteringOffload = 0x0004000,
    /*
     * The interface is being monitored,
     * so we copy all the packets to another interface.
     */
    Monitored        = 0x0008000,
    UnknownUcFlood   = 0x0010000,
    VlanOffload      = 0x0020000,
    /*
     * The interface is marked to drop new incoming flows
     * marked by vrouter agent to enforce flow-limit.
     */
    DropNewFlows     = 0x0040000,
    MacLearn         = 0x0080000,
    MacProxy         = 0x0100000,
    EtreeRoot        = 0x0200000,
    GroNeeded        = 0x0400000,
    MrgRxBuf         = 0x0800000,
    MirrorNotag      = 0x1000000,
    IgmpEnabled      = 0x2000000,
    MockDevice       = 0x4000000,
}

#[derive(Debug, PartialEq)]
pub struct IfRequest {
    pub op: SandeshOp,
    pub core: u32,
    pub _type: IfType,
    pub flags: i32,
    pub vrf: i32,
    pub idx: i32,
    pub rid: i32,
    pub os_idx: i32,
    pub mtu: i32,
    pub name: String,
    pub ibytes: i64,
    pub ipackets: i64,
    pub ierrors: i64,
    pub obytes: i64,
    pub opackets: i64,
    pub oerrors: i64,
    pub queue_ipackets: i64,
    pub queue_ierrors: i64,
    pub queue_ierrors_to_lcore: Vec<i64>,
    pub queue_opackets: i64,
    pub queue_oerrors: i64,
    pub port_ipackets: i64,
    pub port_ierrors: i64,
    pub port_isyscalls: i64,
    pub port_inombufs: i64,
    pub port_opackets: i64,
    pub port_oerrors: i64,
    pub port_osyscalls: i64,
    pub dev_ibytes: i64,
    pub dev_ipackets: i64,
    pub dev_ierrors: i64,
    pub dev_inombufs: i64,
    pub dev_obytes: i64,
    pub dev_opackets: i64,
    pub dev_oerrors: i64,
    pub ref_cnt: i32,
    pub marker: i32,
    pub mac: MacAddress,
    pub ip: Ipv4Addr,
    pub ip6: Ipv6Addr,
    pub context: i32,
    pub mir_id: i16,
    pub speed: i32,
    pub duplex: i32,
    pub vlan_id: i16,
    pub parent_vif_idx: i32,
    pub nh_id: i32,
    pub cross_connect_idx: i32,
    pub src_mac: MacAddress,
    pub bridge_idx: Vec<i32>,
    pub ovlan_id: i16,
    pub transport: Vec<i8>,
    pub fat_flow_protocol_port: Vec<i32>,
    pub qos_map_index: i16,
    pub in_mirror_md: Vec<i8>,
    pub out_mirror_md: Vec<i8>,
    pub dpackets: u64,
    pub hw_queues: Vec<i16>,
    pub isid: u32,
    pub pbb_mac: MacAddress,
    pub vhostuser_mode: Vec<i8>,
    pub mcast_vrf: i32,
    pub if_guid: Vec<i8>,
    pub fat_flow_exclude_ip_list: Vec<Ipv4Addr>,
    pub fat_flow_exclude_ip6_list: Vec<Ipv6Addr>,
    pub fat_flow_exclude_ip6_plen_list: Vec<u16>,
    pub fat_flow_src_prefix: Vec<u128>,
    pub fat_flow_src_prefix_mask: Vec<i8>,
    pub fat_flow_src_aggregate_plen: Vec<i8>,
    pub fat_flow_dst_prefix: Vec<u128>,
    pub fat_flow_dst_prefix_mask: Vec<i8>,
    pub fat_flow_dst_aggregate_plen: Vec<i8>,
    pub intf_status: i8,
    pub fab_name: Vec<i8>,
    pub fab_drv_name: Vec<i8>,
    pub num_bond_slave: i8,
    pub bond_slave_name: Vec<i8>,
    pub bond_slave_drv_name: Vec<i8>,
    pub vlan_tag: u32,
    pub vifr_vlan_name: Vec<i8>,
}
