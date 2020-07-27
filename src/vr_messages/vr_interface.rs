// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::error::CodecError;
use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::*;
use crate::utils;
use eui48::MacAddress;
use std::convert::{TryFrom, TryInto};
use std::ffi::CString;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::os::raw::c_char;
use std::slice;

pub const VIF_MAX_MIRROR_MD_SIZE: u32 = 0xFF;
pub const IPV6_UPPER_MASK: u128 = 0xffffffffffffffff_0000000000000000;
pub const IPV6_LOWER_MASK: u128 = 0x0000000000000000_ffffffffffffffff;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IfType {
    Host = 0,
    Agent = 1,
    Physical = 2,
    Virtual = 3,
    XenLlHost = 4,
    Gateway = 5,
    VirtualVlan = 6,
    Stats = 7,
    Vlan = 8,
    Monitoring = 9,
    Max = 10,
}

impl Default for IfType {
    fn default() -> IfType {
        IfType::Host
    }
}

impl TryFrom<i32> for IfType {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == IfType::Host as i32 => Ok(IfType::Host),
            x if x == IfType::Agent as i32 => Ok(IfType::Agent),
            x if x == IfType::Physical as i32 => Ok(IfType::Physical),
            x if x == IfType::Virtual as i32 => Ok(IfType::Virtual),
            x if x == IfType::XenLlHost as i32 => Ok(IfType::XenLlHost),
            x if x == IfType::Gateway as i32 => Ok(IfType::Gateway),
            x if x == IfType::VirtualVlan as i32 => Ok(IfType::VirtualVlan),
            x if x == IfType::Stats as i32 => Ok(IfType::Stats),
            x if x == IfType::Vlan as i32 => Ok(IfType::Vlan),
            x if x == IfType::Monitoring as i32 => Ok(IfType::Monitoring),
            x if x == IfType::Max as i32 => Ok(IfType::Max),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IfFlag {
    PolicyEnabled = 0x0000001,
    Xconnect = 0x0000002,
    ServiceIf = 0x0000004,
    MirrorRx = 0x0000008,
    MirrorTx = 0x0000010,
    TxCsumOffload = 0x0000020,
    L3Enabled = 0x0000040,
    L2Enabled = 0x0000080,
    DhcpEnabled = 0x0000100,
    // The physical interface corresponds to a vhost interface
    VhostPhys = 0x0000200,
    Promiscous = 0x0000400,
    // untagged packet should be treated as packets with tag 0.
    NativeVlanTag = 0x0000800,
    NoArpProxy = 0x0001000,
    Pmd = 0x0002000,
    // The physical interface supports hardware filtering.
    FilteringOffload = 0x0004000,
    /*
     * The interface is being monitored,
     * so we copy all the packets to another interface.
     */
    Monitored = 0x0008000,
    UnknownUcFlood = 0x0010000,
    VlanOffload = 0x0020000,
    /*
     * The interface is marked to drop new incoming flows
     * marked by vrouter agent to enforce flow-limit.
     */
    DropNewFlows = 0x0040000,
    MacLearn = 0x0080000,
    MacProxy = 0x0100000,
    EtreeRoot = 0x0200000,
    GroNeeded = 0x0400000,
    MrgRxBuf = 0x0800000,
    MirrorNotag = 0x1000000,
    IgmpEnabled = 0x2000000,
    MockDevice = 0x4000000,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InterfaceRequest {
    pub op: SandeshOp,
    pub read_length: usize,
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
    pub mirror_id: i16,
    pub speed: i32,
    pub duplex: i32,
    pub vlan_id: i16,
    pub parent_vif_idx: i32,
    pub nh_id: i32,
    pub cross_connect_idx: i32,
    pub src_mac: MacAddress,
    pub bridge_idx: Vec<i32>,
    pub ovlan_id: i16,
    pub transport: i8,
    pub fat_flow_protocol_port: Vec<i32>,
    pub qos_map_index: i16,
    pub in_mirror_md: Vec<i8>,
    pub out_mirror_md: Vec<i8>,
    pub dpackets: u64,
    pub hw_queues: Vec<i16>,
    pub isid: u32,
    pub pbb_mac: MacAddress,
    pub vhostuser_mode: i8,
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

impl Default for InterfaceRequest {
    fn default() -> InterfaceRequest {
        InterfaceRequest {
            op: SandeshOp::Add,
            read_length: 0,
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
            mirror_id: 0,
            speed: 0,
            duplex: 0,
            vlan_id: 0,
            parent_vif_idx: 0,
            nh_id: 0,
            cross_connect_idx: 0,
            src_mac: MacAddress::nil(),
            bridge_idx: vec![],
            ovlan_id: 0,
            transport: 0,
            fat_flow_protocol_port: vec![],
            qos_map_index: 0,
            in_mirror_md: vec![],
            out_mirror_md: vec![],
            dpackets: 0,
            hw_queues: vec![],
            isid: 0,
            pbb_mac: MacAddress::nil(),
            vhostuser_mode: 0,
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
            vlan_name: String::default(),
        }
    }
}

impl InterfaceRequest {
    pub fn write(&self) -> Result<Vec<u8>, CodecError> {
        let mut encoder: vr_interface_req = vr_interface_req::new();
        encoder.h_op = self.op as u32;
        encoder.vifr_core = self.core;
        encoder.vifr_type = self._type as i32;
        encoder.vifr_flags = self.flags;
        encoder.vifr_vrf = self.vrf;
        encoder.vifr_idx = self.idx;
        encoder.vifr_rid = self.rid;
        encoder.vifr_os_idx = self.os_idx;
        encoder.vifr_mtu = self.mtu;
        encoder.vifr_name = Self::write_cstring(&self.name);
        encoder.vifr_ibytes = self.ibytes;
        encoder.vifr_ipackets = self.ipackets;
        encoder.vifr_ierrors = self.ierrors;
        encoder.vifr_obytes = self.obytes;
        encoder.vifr_opackets = self.opackets;
        encoder.vifr_oerrors = self.oerrors;
        encoder.vifr_queue_ipackets = self.queue_ipackets;
        encoder.vifr_queue_ierrors = self.queue_ierrors;
        encoder.vifr_queue_ierrors_to_lcore =
            utils::into_mut_ptr(&self.queue_ierrors_to_lcore);
        encoder.vifr_queue_ierrors_to_lcore_size =
            self.queue_ierrors_to_lcore.len() as u32;
        encoder.vifr_queue_opackets = self.queue_opackets;
        encoder.vifr_queue_oerrors = self.queue_oerrors;
        encoder.vifr_port_ipackets = self.port_ipackets;
        encoder.vifr_port_ierrors = self.port_ierrors;
        encoder.vifr_port_isyscalls = self.port_isyscalls;
        encoder.vifr_port_inombufs = self.port_inombufs;
        encoder.vifr_port_opackets = self.port_opackets;
        encoder.vifr_port_oerrors = self.port_oerrors;
        encoder.vifr_port_osyscalls = self.port_osyscalls;
        encoder.vifr_dev_ibytes = self.dev_ibytes;
        encoder.vifr_dev_ipackets = self.dev_ipackets;
        encoder.vifr_dev_ierrors = self.dev_ierrors;
        encoder.vifr_dev_inombufs = self.dev_inombufs;
        encoder.vifr_dev_obytes = self.dev_obytes;
        encoder.vifr_dev_opackets = self.dev_opackets;
        encoder.vifr_dev_oerrors = self.dev_oerrors;
        encoder.vifr_ref_cnt = self.ref_cnt;
        encoder.vifr_marker = self.marker;
        encoder.vifr_mac = utils::write_mac(self.mac);
        encoder.vifr_mac_size = if self.mac.is_nil() {
            0u32
        } else {
            libc::ETH_ALEN as u32
        };
        encoder.vifr_ip = utils::write_ip4(self.ip);
        let vifr_ip6 = utils::write_ip6(self.ip6);
        encoder.vifr_ip6_u = ((vifr_ip6 & IPV6_UPPER_MASK) >> 64) as u64;
        encoder.vifr_ip6_l = (vifr_ip6 & IPV6_LOWER_MASK) as u64;
        encoder.vifr_context = self.context;
        encoder.vifr_mir_id = self.mirror_id;
        encoder.vifr_speed = self.speed;
        encoder.vifr_duplex = self.duplex;
        encoder.vifr_vlan_id = self.vlan_id;
        encoder.vifr_parent_vif_idx = self.parent_vif_idx;
        encoder.vifr_nh_id = self.nh_id;
        encoder.vifr_cross_connect_idx = self.cross_connect_idx;
        encoder.vifr_src_mac = utils::write_mac(self.src_mac);
        encoder.vifr_src_mac_size = if self.src_mac.is_nil() {
            0u32
        } else {
            libc::ETH_ALEN as u32
        };
        encoder.vifr_bridge_idx = utils::into_mut_ptr(&self.bridge_idx);
        encoder.vifr_bridge_idx_size = self.bridge_idx.len() as u32;
        encoder.vifr_ovlan_id = self.ovlan_id;
        encoder.vifr_transport = self.transport;
        encoder.vifr_fat_flow_protocol_port =
            utils::into_mut_ptr(&self.fat_flow_protocol_port);
        encoder.vifr_fat_flow_protocol_port_size =
            self.fat_flow_protocol_port.len() as u32;
        encoder.vifr_qos_map_index = self.qos_map_index;
        encoder.vifr_in_mirror_md = utils::into_mut_ptr(&self.in_mirror_md);
        encoder.vifr_in_mirror_md_size = self.in_mirror_md.len() as u32;
        encoder.vifr_out_mirror_md = utils::into_mut_ptr(&self.out_mirror_md);
        encoder.vifr_out_mirror_md_size = self.out_mirror_md.len() as u32;
        encoder.vifr_dpackets = self.dpackets;
        encoder.vifr_hw_queues = utils::into_mut_ptr(&self.hw_queues);
        encoder.vifr_hw_queues_size = self.hw_queues.len() as u32;
        encoder.vifr_isid = self.isid;
        encoder.vifr_pbb_mac = utils::write_mac(self.pbb_mac);
        encoder.vifr_pbb_mac_size = if self.pbb_mac.is_nil() {
            0u32
        } else {
            libc::ETH_ALEN as u32
        };
        encoder.vifr_vhostuser_mode = self.vhostuser_mode;
        encoder.vifr_mcast_vrf = self.mcast_vrf;
        encoder.vifr_if_guid = utils::into_mut_ptr(&self.if_guid);
        encoder.vifr_if_guid_size = self.if_guid.len() as u32;
        encoder.vifr_fat_flow_exclude_ip_list =
            Self::write_ip_list(&self.fat_flow_exclude_ip_list);
        encoder.vifr_fat_flow_exclude_ip_list_size =
            self.fat_flow_exclude_ip_list.len() as u32;
        let ip6_mut_ptrs =
            Self::write_ip6_list(&self.fat_flow_exclude_ip6_list);
        encoder.vifr_fat_flow_exclude_ip6_l_list = ip6_mut_ptrs.0;
        encoder.vifr_fat_flow_exclude_ip6_l_list_size =
            self.fat_flow_exclude_ip6_list.len() as u32;
        encoder.vifr_fat_flow_exclude_ip6_u_list = ip6_mut_ptrs.1;
        encoder.vifr_fat_flow_exclude_ip6_u_list_size =
            self.fat_flow_exclude_ip6_list.len() as u32;
        encoder.vifr_fat_flow_exclude_ip6_plen_list =
            utils::into_mut_ptr(&self.fat_flow_exclude_ip6_plen_list);
        encoder.vifr_fat_flow_exclude_ip6_plen_list_size =
            self.fat_flow_exclude_ip6_plen_list.len() as u32;
        let src_prefixes =
            Self::write_u128_into_u64_ptr(&self.fat_flow_src_prefix);
        encoder.vifr_fat_flow_src_prefix_l = src_prefixes.0;
        encoder.vifr_fat_flow_src_prefix_l_size =
            self.fat_flow_src_prefix.len() as u32;
        encoder.vifr_fat_flow_src_prefix_h = src_prefixes.1;
        encoder.vifr_fat_flow_src_prefix_h_size =
            self.fat_flow_src_prefix.len() as u32;
        encoder.vifr_fat_flow_src_prefix_mask =
            utils::into_mut_ptr(&self.fat_flow_src_prefix_mask);
        encoder.vifr_fat_flow_src_prefix_mask_size =
            self.fat_flow_src_prefix_mask.len() as u32;
        encoder.vifr_fat_flow_src_aggregate_plen =
            utils::into_mut_ptr(&self.fat_flow_src_aggregate_plen);
        encoder.vifr_fat_flow_src_aggregate_plen_size =
            self.fat_flow_src_aggregate_plen.len() as u32;
        let dst_prefixes =
            Self::write_u128_into_u64_ptr(&self.fat_flow_dst_prefix);
        encoder.vifr_fat_flow_dst_prefix_l = dst_prefixes.0;
        encoder.vifr_fat_flow_dst_prefix_l_size =
            self.fat_flow_dst_prefix.len() as u32;
        encoder.vifr_fat_flow_dst_prefix_h = dst_prefixes.1;
        encoder.vifr_fat_flow_dst_prefix_h_size =
            self.fat_flow_dst_prefix.len() as u32;
        encoder.vifr_fat_flow_dst_prefix_mask =
            utils::into_mut_ptr(&self.fat_flow_dst_prefix_mask);
        encoder.vifr_fat_flow_dst_prefix_mask_size =
            self.fat_flow_dst_prefix_mask.len() as u32;
        encoder.vifr_fat_flow_dst_aggregate_plen =
            utils::into_mut_ptr(&self.fat_flow_dst_aggregate_plen);
        encoder.vifr_fat_flow_dst_aggregate_plen_size =
            self.fat_flow_dst_aggregate_plen.len() as u32;
        encoder.vifr_intf_status = self.intf_status;
        encoder.vifr_fab_name = Self::write_string(&self.fab_name);
        encoder.vifr_fab_name_size = self.fab_name.len() as u32;
        encoder.vifr_fab_drv_name = Self::write_string(&self.fab_drv_name);
        encoder.vifr_fab_drv_name_size = self.fab_drv_name.len() as u32;
        encoder.vifr_num_bond_slave = self.num_bond_slave;
        encoder.vifr_bond_slave_name =
            Self::write_string(&self.bond_slave_name);
        encoder.vifr_bond_slave_name_size = self.bond_slave_name.len() as u32;
        encoder.vifr_bond_slave_drv_name =
            Self::write_string(&self.bond_slave_drv_name);
        encoder.vifr_bond_slave_drv_name_size =
            self.bond_slave_drv_name.len() as u32;
        encoder.vifr_vlan_tag = self.vlan_tag;
        encoder.vifr_vlan_name = Self::write_string(&self.vlan_name);
        encoder.vifr_vlan_name_size = self.vlan_name.len() as u32;

        match encoder.write() {
            Err(e) => Err(e),
            Ok(v) => Ok(v),
        }
    }

    pub fn read(buf: Vec<u8>) -> Result<InterfaceRequest, CodecError> {
        let decoder: vr_interface_req = vr_interface_req::new();
        match decoder.read(&buf) {
            Err(e) => Err(e),
            Ok(rxfer) => {
                let mut vifr = InterfaceRequest::default();
                vifr.read_length = rxfer as usize;
                vifr.op = decoder.h_op.try_into().unwrap();
                vifr.core = decoder.vifr_core;
                vifr._type = decoder.vifr_type.try_into().unwrap();
                vifr.flags = decoder.vifr_flags;
                vifr.vrf = decoder.vifr_vrf;
                vifr.idx = decoder.vifr_idx;
                vifr.rid = decoder.vifr_rid;
                vifr.os_idx = decoder.vifr_os_idx;
                vifr.mtu = decoder.vifr_mtu;
                vifr.name = Self::read_cstring(decoder.vifr_name);
                vifr.ibytes = decoder.vifr_ibytes;
                vifr.ipackets = decoder.vifr_ipackets;
                vifr.ierrors = decoder.vifr_ierrors;
                vifr.obytes = decoder.vifr_obytes;
                vifr.opackets = decoder.vifr_opackets;
                vifr.oerrors = decoder.vifr_oerrors;
                vifr.queue_ipackets = decoder.vifr_queue_ipackets;
                vifr.queue_ierrors = decoder.vifr_queue_ierrors;
                vifr.queue_ierrors_to_lcore = utils::free_buf::<i64>(
                    decoder.vifr_queue_ierrors_to_lcore as *mut i64,
                    decoder.vifr_queue_ierrors_to_lcore_size as usize,
                );
                vifr.queue_opackets = decoder.vifr_queue_opackets;
                vifr.queue_oerrors = decoder.vifr_queue_oerrors;
                vifr.port_ipackets = decoder.vifr_port_ipackets;
                vifr.port_ierrors = decoder.vifr_port_ierrors;
                vifr.port_isyscalls = decoder.vifr_port_isyscalls;
                vifr.port_inombufs = decoder.vifr_port_inombufs;
                vifr.port_opackets = decoder.vifr_port_opackets;
                vifr.port_oerrors = decoder.vifr_port_oerrors;
                vifr.port_osyscalls = decoder.vifr_port_osyscalls;
                vifr.dev_ibytes = decoder.vifr_dev_ibytes;
                vifr.dev_ipackets = decoder.vifr_dev_ipackets;
                vifr.dev_ierrors = decoder.vifr_dev_ierrors;
                vifr.dev_inombufs = decoder.vifr_dev_inombufs;
                vifr.dev_obytes = decoder.vifr_dev_obytes;
                vifr.dev_opackets = decoder.vifr_dev_opackets;
                vifr.dev_oerrors = decoder.vifr_dev_oerrors;
                vifr.ref_cnt = decoder.vifr_ref_cnt;
                vifr.marker = decoder.vifr_marker;
                vifr.mac = utils::read_mac_addr(
                    decoder.vifr_mac,
                    decoder.vifr_mac_size,
                );
                vifr.ip = Ipv4Addr::from(decoder.vifr_ip);
                vifr.ip6 = Ipv6Addr::from(
                    ((decoder.vifr_ip6_u as u128) << 64)
                        | (decoder.vifr_ip6_l as u128),
                );
                vifr.context = decoder.vifr_context;
                vifr.mirror_id = decoder.vifr_mir_id;
                vifr.speed = decoder.vifr_speed;
                vifr.duplex = decoder.vifr_duplex;
                vifr.vlan_id = decoder.vifr_vlan_id;
                vifr.parent_vif_idx = decoder.vifr_parent_vif_idx;
                vifr.nh_id = decoder.vifr_nh_id;
                vifr.cross_connect_idx = decoder.vifr_cross_connect_idx;
                vifr.src_mac = utils::read_mac_addr(
                    decoder.vifr_src_mac,
                    decoder.vifr_src_mac_size,
                );
                vifr.bridge_idx = utils::free_buf(
                    decoder.vifr_bridge_idx as *mut i32,
                    decoder.vifr_bridge_idx_size as usize,
                );
                vifr.ovlan_id = decoder.vifr_ovlan_id;
                vifr.transport = decoder.vifr_transport;
                vifr.fat_flow_protocol_port = utils::free_buf(
                    decoder.vifr_fat_flow_protocol_port as *mut i32,
                    decoder.vifr_fat_flow_protocol_port_size as usize,
                );
                vifr.qos_map_index = decoder.vifr_qos_map_index;
                vifr.in_mirror_md = utils::free_buf(
                    decoder.vifr_in_mirror_md,
                    decoder.vifr_in_mirror_md_size as usize,
                );
                vifr.out_mirror_md = utils::free_buf(
                    decoder.vifr_out_mirror_md,
                    decoder.vifr_out_mirror_md_size as usize,
                );
                vifr.dpackets = decoder.vifr_dpackets;
                vifr.hw_queues = utils::free_buf(
                    decoder.vifr_hw_queues,
                    decoder.vifr_hw_queues_size as usize,
                );
                vifr.isid = decoder.vifr_isid;
                vifr.pbb_mac = utils::read_mac_addr(
                    decoder.vifr_pbb_mac,
                    decoder.vifr_pbb_mac_size,
                );
                vifr.vhostuser_mode = decoder.vifr_vhostuser_mode;
                vifr.mcast_vrf = decoder.vifr_mcast_vrf;
                vifr.if_guid = utils::free_buf(
                    decoder.vifr_if_guid,
                    decoder.vifr_if_guid_size as usize,
                );
                vifr.fat_flow_exclude_ip_list = Self::read_ip_list(
                    decoder.vifr_fat_flow_exclude_ip_list,
                    decoder.vifr_fat_flow_exclude_ip_list_size,
                );
                vifr.fat_flow_exclude_ip6_list = Self::read_ip6_list(
                    decoder.vifr_fat_flow_exclude_ip6_u_list,
                    decoder.vifr_fat_flow_exclude_ip6_l_list,
                    decoder.vifr_fat_flow_exclude_ip6_u_list_size,
                    decoder.vifr_fat_flow_exclude_ip6_l_list_size,
                );
                vifr.fat_flow_exclude_ip6_plen_list = utils::free_buf(
                    decoder.vifr_fat_flow_exclude_ip6_plen_list,
                    decoder.vifr_fat_flow_exclude_ip6_plen_list_size as usize,
                );
                vifr.fat_flow_src_prefix = Self::read_splitted_u128_vec(
                    decoder.vifr_fat_flow_src_prefix_h,
                    decoder.vifr_fat_flow_src_prefix_l,
                    decoder.vifr_fat_flow_src_prefix_h_size,
                    decoder.vifr_fat_flow_src_prefix_l_size,
                );
                vifr.fat_flow_src_prefix_mask = utils::free_buf(
                    decoder.vifr_fat_flow_src_prefix_mask,
                    decoder.vifr_fat_flow_src_prefix_mask_size as usize,
                );
                vifr.fat_flow_src_aggregate_plen = utils::free_buf(
                    decoder.vifr_fat_flow_src_aggregate_plen,
                    decoder.vifr_fat_flow_src_aggregate_plen_size as usize,
                );
                vifr.fat_flow_dst_prefix = Self::read_splitted_u128_vec(
                    decoder.vifr_fat_flow_dst_prefix_h,
                    decoder.vifr_fat_flow_dst_prefix_l,
                    decoder.vifr_fat_flow_dst_prefix_h_size,
                    decoder.vifr_fat_flow_dst_prefix_l_size,
                );
                vifr.fat_flow_dst_prefix_mask = utils::free_buf(
                    decoder.vifr_fat_flow_dst_prefix_mask,
                    decoder.vifr_fat_flow_dst_prefix_mask_size as usize,
                );
                vifr.fat_flow_dst_aggregate_plen = utils::free_buf(
                    decoder.vifr_fat_flow_dst_aggregate_plen,
                    decoder.vifr_fat_flow_dst_aggregate_plen_size as usize,
                );
                vifr.intf_status = decoder.vifr_intf_status;
                vifr.fab_name = Self::read_bytes_as_string(
                    decoder.vifr_fab_name,
                    decoder.vifr_fab_name_size,
                );
                vifr.fab_drv_name = Self::read_bytes_as_string(
                    decoder.vifr_fab_drv_name,
                    decoder.vifr_fab_drv_name_size,
                );
                vifr.num_bond_slave = decoder.vifr_num_bond_slave;
                vifr.bond_slave_name = Self::read_bytes_as_string(
                    decoder.vifr_bond_slave_name,
                    decoder.vifr_bond_slave_name_size,
                );
                vifr.bond_slave_drv_name = Self::read_bytes_as_string(
                    decoder.vifr_bond_slave_drv_name,
                    decoder.vifr_bond_slave_drv_name_size,
                );
                vifr.vlan_tag = decoder.vifr_vlan_tag;
                vifr.vlan_name = Self::read_bytes_as_string(
                    decoder.vifr_vlan_name,
                    decoder.vifr_vlan_name_size,
                );
                Ok(vifr)
            }
        }
    }

    // private functions

    fn read_bytes_as_string(ptr: *mut i8, size: u32) -> String {
        unsafe {
            String::from(
                std::str::from_utf8(slice::from_raw_parts(
                    ptr as *const u8,
                    size as usize,
                ))
                .unwrap(),
            )
        }
    }

    fn write_ip6_list(ip6_list: &Vec<Ipv6Addr>) -> (*mut u64, *mut u64) {
        let mut ip6_u: Vec<u64> = Vec::new();
        let mut ip6_l: Vec<u64> = Vec::new();
        ip6_list.iter().for_each(|&ip6| {
            let ip6 = utils::write_ip6(ip6);
            ip6_l.push((ip6 & IPV6_LOWER_MASK) as u64);
            ip6_u.push(((ip6 & IPV6_UPPER_MASK) >> 64) as u64);
        });

        (utils::into_mut_ptr(&ip6_l), utils::into_mut_ptr(&ip6_u))
    }

    fn read_ip6_list(
        ptr_u: *mut u64,
        ptr_l: *mut u64,
        size_u: u32,
        size_l: u32,
    ) -> Vec<Ipv6Addr> {
        let ip6_list: Vec<Ipv6Addr> = Vec::new();
        let ip6_i_v =
            Self::read_splitted_u128_vec(ptr_u, ptr_l, size_u, size_l);

        ip6_i_v.iter().fold(ip6_list, |mut acc, &ip6_i| {
            let ip6 = Ipv6Addr::from(ip6_i);
            acc.push(ip6);
            acc
        })
    }

    fn write_u128_into_u64_ptr(v: &Vec<u128>) -> (*mut u64, *mut u64) {
        let mut upper: Vec<u64> = Vec::new();
        let mut lower: Vec<u64> = Vec::new();
        v.iter().for_each(|x| {
            lower.push((x & IPV6_LOWER_MASK) as u64);
            upper.push((x & IPV6_UPPER_MASK) as u64);
        });
        (utils::into_mut_ptr(&lower), utils::into_mut_ptr(&upper))
    }

    fn read_splitted_u128_vec(
        ptr_u: *mut u64,
        ptr_l: *mut u64,
        size_u: u32,
        size_l: u32,
    ) -> Vec<u128> {
        let u128_list: Vec<u128> = Vec::new();
        let u128_u_v: Vec<u64> = utils::free_buf(ptr_u, size_u as usize);
        let u128_l_v: Vec<u64> = utils::free_buf(ptr_l, size_l as usize);
        u128_l_v.iter().enumerate().fold(
            u128_list,
            |mut acc, (idx, &u128_l)| {
                let u128_i = ((u128_u_v[idx] as u128) << 64) | u128_l as u128;
                acc.push(u128_i);
                acc
            },
        )
    }

    fn write_ip_list(ip_list: &Vec<Ipv4Addr>) -> *mut u64 {
        let mut v: Vec<u64> = Vec::new();
        ip_list.iter().fold(&mut v, |acc, &ip4| {
            let ip4_int = utils::write_ip4(ip4) as u64;
            acc.push(ip4_int);
            acc
        });
        utils::into_mut_ptr(&v)
    }

    fn read_ip_list(ptr: *mut u64, size: u32) -> Vec<Ipv4Addr> {
        let mut ip_list: Vec<Ipv4Addr> = Vec::new();
        let v: Vec<u64> = utils::free_buf(ptr, size as usize);
        v.iter().fold(&mut ip_list, |acc, &ip_u64| {
            let ip = Ipv4Addr::from(ip_u64 as u32);
            acc.push(ip);
            acc
        });
        return ip_list;
    }

    fn write_string(s: &String) -> *mut i8 {
        Self::write_cstring(s) as *mut i8
    }

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
