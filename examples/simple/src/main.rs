// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

extern crate nix;
extern crate vr_type;
extern crate mac_address;
extern crate eui48;

use crate::nix::ifaddrs::*;
use crate::vr_type::vr_messages::*;
use crate::vr_type::genetlink::MessageHandleError;
use crate::mac_address::*;
use crate::eui48::MacAddress as MacAddr;

const VM1_IFNAME: &'static str = "veth0";
const VM2_IFNAME: &'static str = "veth2";
const AGENT_IFNAME: &'static str = "agent0";

fn main() {
    // prepare interfaces
    init_ifaces().unwrap();
}

// private functions

#[derive(Debug, Default)]
struct VifState(i32);

fn init_ifaces() -> Result<Vec<Message>, MessageHandleError> {
    // prepare interfaces
    let mut vif_state = VifState(0);
    add_agent_iface(vif_state.0)?;
    vif_state.0 += 1;
    add_veth_iface(VM1_IFNAME, vif_state.0)?;
    vif_state.0 += 1;
    add_veth_iface(VM2_IFNAME, vif_state.0)
}

fn add_veth_iface(name: &str, idx: i32) -> Result<Vec<Message>, MessageHandleError> {
    let mut vifr: InterfaceRequest = InterfaceRequest::default();
    vifr._type = IfType::Virtual;
    vifr.name = name.to_string();
    vifr.idx = idx;
    vifr.mac = find_macaddr(name).unwrap();
    vifr.nh_id = -1;
    vifr.transport = VIF_TRANSPORT_VIRTUAL;
    vifr.mtu = 1514;
    vifr.flags = VIF_FLAG_POLICY_ENABLED | VIF_FLAG_DHCP_ENABLED;
    let request = Message::InterfaceRequest(vifr);
    request.send_nl()
}

fn add_agent_iface(idx: i32) -> Result<Vec<Message>, MessageHandleError> {
    let mut vifr: InterfaceRequest = InterfaceRequest::default();
    vifr._type = IfType::Agent;
    vifr.name = AGENT_IFNAME.to_string();
    vifr.idx = idx;
    vifr.mac =  find_macaddr(AGENT_IFNAME).unwrap();
    vifr.nh_id = -1;
    vifr.transport = VIF_TRANSPORT_SOCKET;
    vifr.mtu = 1514;
    vifr.flags = VIF_FLAG_L3_ENABLED | VIF_FLAG_DHCP_ENABLED;
    let request = Message::InterfaceRequest(vifr);
    request.send_nl()
}

fn find_macaddr(name: &str) -> Result<MacAddr, ()> {
    if let Ok(Some(macaddr)) = mac_address_by_name(&name.to_string()) {
        Ok(MacAddr::new(macaddr.bytes()))
    } else {
        Err(())
    }
}
