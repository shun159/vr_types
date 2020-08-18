// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::nix::net::if_::if_nametoindex;
use crate::vr_type::utils::{create_vhost, add_vhost_ip};
use crate::vr_type::vr_messages::*;
use crate::vr_type::genetlink::MessageHandleError;
use crate::mac_address::*;
use crate::eui48::MacAddress as MacAddr;
use crate::pnet::datalink::*;
use std::net::{IpAddr, Ipv4Addr};

const VM1_IFNAME: &'static str = "veth0";
const VM2_IFNAME: &'static str = "veth2";
const XC_IFNAME: &'static str = "veth4";
const VHOST_IFNAME: &'static str = "vhost0";

#[derive(Default)]
struct VifState(i32);

impl VifState {
    fn alloc(&mut self) -> i32 {
        let cur = self.0;
        self.0 += 1;
        cur
    }
}

pub fn init_ifaces() -> Result<Vec<Message>, MessageHandleError> {
    let mut state = VifState::default();
    let _ = create_vhost();
    let _ = add_vhost_ip("10.10.10.10/24".to_string());
    add_xc_iface(state.alloc())?;
    add_veth_iface(VM1_IFNAME, state.alloc())?;
    add_veth_iface(VM2_IFNAME, state.alloc())?;
    add_vhost_iface(state.alloc())
}

pub fn get_iface(id: i32) -> Result<Vec<Message>, MessageHandleError> {
    let mut vifr: InterfaceRequest = InterfaceRequest::default();
    vifr.op = SandeshOp::Get;
    vifr.idx = id;
    let request = Message::InterfaceRequest(vifr);
    request.send_nl()
}

pub fn add_veth_iface(name: &str, idx: i32) -> Result<Vec<Message>, MessageHandleError> {
    let mut vifr: InterfaceRequest = InterfaceRequest::default();
    vifr.op = SandeshOp::Add;
    vifr._type = IfType::Virtual;
    vifr.nh_id = 1;
    vifr.name = name.to_string();
    vifr.idx = idx;
    vifr.os_idx = find_ifindex(name).unwrap();
    vifr.mac = find_macaddr(name).unwrap();
    vifr.transport = VIF_TRANSPORT_ETH;
    vifr.flags = VIF_FLAG_L2_ENABLED | VIF_FLAG_POLICY_ENABLED | VIF_FLAG_DHCP_ENABLED;
    vifr.mtu = 1514;
    let request = Message::InterfaceRequest(vifr);
    request.send_nl()
}

pub fn add_xc_iface(idx: i32) -> Result<Vec<Message>, MessageHandleError> {
    let mut vifr: InterfaceRequest = InterfaceRequest::default();
    vifr.op = SandeshOp::Add;
    vifr._type = IfType::Virtual;
    vifr.name = XC_IFNAME.to_string();
    vifr.idx = idx;
    vifr.os_idx = find_ifindex(XC_IFNAME).unwrap();
    vifr.mac = find_macaddr(XC_IFNAME).unwrap();
    vifr.transport = VIF_TRANSPORT_ETH;
    vifr.flags = VIF_FLAG_XCONNECT | VIF_FLAG_VHOST_PHYS;
    vifr.vrf = 0xffff;
    vifr.mcast_vrf = 0xffff;
    vifr.mtu = 9000;
    let request = Message::InterfaceRequest(vifr);
    request.send_nl()
}

pub fn add_vhost_iface(idx: i32) -> Result<Vec<Message>, MessageHandleError> {
    let mut vifr: InterfaceRequest = InterfaceRequest::default();
    vifr.op = SandeshOp::Add;
    vifr._type = IfType::Host;
    vifr.name = VHOST_IFNAME.to_string();
    vifr.idx = idx;//find_ifindex(VHOST_IFNAME).unwrap();
    vifr.os_idx = find_ifindex(VHOST_IFNAME).unwrap();
    vifr.cross_connect_idx = find_ifindex(XC_IFNAME).unwrap();
    vifr.ip = find_ipaddr(VHOST_IFNAME).unwrap();
    vifr.mac = find_macaddr(VHOST_IFNAME).unwrap();
    vifr.flags = VIF_FLAG_L3_ENABLED | VIF_FLAG_DHCP_ENABLED;
    vifr.transport = VIF_TRANSPORT_PMD;
    vifr.mcast_vrf = 0xffff;
    vifr.mtu = 9000;
    let request = Message::InterfaceRequest(vifr);
    request.send_nl()
}

pub fn find_ifindex(name: &str) -> Result<i32, ()> {
    if let Ok(index) = if_nametoindex(name) {
         Ok(index as i32)
    } else {
        Err(())
    }
}

pub fn find_macaddr(name: &str) -> Result<MacAddr, ()> {
    if let Ok(Some(macaddr)) = mac_address_by_name(&name.to_string()) {
        Ok(MacAddr::new(macaddr.bytes()))
    } else {
        Err(())
    }
}

pub fn find_ipaddr(name: &str) -> Result<Ipv4Addr, ()> {
    if let Some(iface) = interfaces()
        .iter()
        .find(|iface: &&NetworkInterface| {
            iface.name == name.to_string()
        }) {
            match iface.ips[0].ip() {
                IpAddr::V4(ip) => Ok(ip),
                _ => Err(())
            }
        } else {
            Err(())
        }
}
