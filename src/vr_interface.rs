// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use eui48::MacAddress;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::convert::{TryFrom, TryInto};

pub const VIF_MAX_MIRROR_MD_SIZE: u32 = 0xFF;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IfType {
    Host        = 0,
    Agent       = 1,
    Physical    = 2,
    Virtual     = 3,
    XenLlHost   = 4,
    Gateway     = 5,
    VirtualVlan = 6,
    Stats       = 7,
    Vlan        = 8,
    Monitoring  = 9,
    Max         = 10
}

impl TryFrom<i8> for IfType {
    type Error = ();
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        match v {
            x if x == IfType::Host as i8 =>
                Ok(IfType::Host),
            x if x == IfType::Agent as i8 =>
                Ok(IfType::Agent),
            x if x == IfType::Physical as i8 =>
                Ok(IfType::Physical),
            x if x == IfType::Virtual as i8 =>
                Ok(IfType::Virtual),
            x if x == IfType::XenLlHost as i8 =>
                Ok(IfType::XenLlHost),
            x if x == IfType::Gateway as i8 =>
                Ok(IfType::Gateway),
            x if x == IfType::VirtualVlan as i8 =>
                Ok(IfType::VirtualVlan),
            x if x == IfType::Stats as i8 =>
                Ok(IfType::Stats),
            x if x == IfType::Vlan as i8 =>
                Ok(IfType::Vlan),
            x if x == IfType::Monitoring as i8 =>
                Ok(IfType::Monitoring),
            x if x == IfType::Max as i8 =>
                Ok(IfType::Max),
            _ =>
                Err(())
        }
    }
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
    pub fab_name: String,
    pub fab_drv_name: String,
    pub num_bond_slave: i8,
    pub bond_slave_name: String,
    pub bond_slave_drv_name: String,
    pub vlan_tag: u32,
    pub vlan_name: String,
}

impl Default for IfRequest {
    fn default() -> IfRequest {
        IfRequest {
            op: SandeshOp::Add,
            core: 0,
            _type: IfType::Host,
            flags: 0,
            vrf: 0,
            idx: 0,
            rid: 0,
            os_idx: 0,
            mtu: 0,
            name: String::default(),
            ibytes: 0,
            ipackets: 0,
            ierrors: 0,
            obytes: 0,
            opackets: 0,
            oerrors: 0,
            queue_ipackets: 0,
            queue_ierrors: 0,
            queue_ierrors_to_lcore: vec![],
            queue_opackets: 0,
            queue_oerrors: 0,
            port_ipackets: 0,
            port_ierrors: 0,
            port_isyscalls: 0,
            port_inombufs: 0,
            port_opackets: 0,
            port_oerrors: 0,
            port_osyscalls: 0,
            dev_ibytes: 0,
            dev_ipackets: 0,
            dev_ierrors: 0,
            dev_inombufs: 0,
            dev_obytes: 0,
            dev_opackets: 0,
            dev_oerrors: 0,
            ref_cnt: 0,
            marker: 0,
            mac: MacAddress::nil(),
            ip: Ipv4Addr::UNSPECIFIED,
            ip6: Ipv6Addr::UNSPECIFIED,
            context: 0,
            mir_id: 0,
            speed: 0,
            duplex: 0,
            vlan_id: 0,
            parent_vif_idx: 0,
            nh_id: 0,
            cross_connect_idx: 0,
            src_mac: MacAddress::nil(),
            bridge_idx: vec![],
            ovlan_id: 0,
            transport: vec![],
            fat_flow_protocol_port: vec![],
            qos_map_index: 0,
            in_mirror_md: vec![],
            out_mirror_md: vec![],
            dpackets: 0,
            hw_queues: vec![],
            isid: 0,
            pbb_mac: MacAddress::nil(),
            vhostuser_mode: vec![],
            mcast_vrf: 0,
            if_guid: vec![],
            fat_flow_exclude_ip_list: vec![],
            fat_flow_exclude_ip6_list: vec![],
            fat_flow_exclude_ip6_plen_list: vec![],
            fat_flow_src_prefix: vec![],
            fat_flow_src_prefix_mask: vec![],
            fat_flow_src_aggregate_plen: vec![],
            fat_flow_dst_prefix: vec![],
            fat_flow_dst_prefix_mask: vec![],
            fat_flow_dst_aggregate_plen: vec![],
            intf_status: 0,
            fab_name: String::default(),
            fab_drv_name: String::default(),
            num_bond_slave: 0,
            bond_slave_name: String::default(),
            bond_slave_drv_name: String::default(),
            vlan_tag: 0,
            vlan_name: String::default()
        }
    }
}
