// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::sandesh::SandeshOp;
use crate::utils;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_route_req;
use eui48::MacAddress;
use std::convert::TryInto;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RouteFlag {
    Valid = 0x01,
    LabelVaild = 0x02,
    FloodDhcp = 0x04,
    MacMoved = 0x08,
    L2ControlData = 0x10,
    MacNew = 0x20,
    EvpnControlProcessing = 0x40,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RouteRequest {
    pub op: SandeshOp,
    pub vrf_id: i32,
    pub family: i32,
    pub prefix: Option<IpAddr>,
    pub prefix_len: i32,
    pub rid: i16,
    pub label_flags: i16,
    pub label: i32,
    pub nh_id: i32,
    pub marker: Option<IpAddr>,
    pub marker_prefix_len: i32,
    pub mac: MacAddress,
    pub replace_prefix_len: i32,
    pub index: i32,
}

impl Default for RouteRequest {
    fn default() -> RouteRequest {
        RouteRequest {
            op: SandeshOp::Add,
            vrf_id: 0,
            family: 0,
            prefix: None,
            prefix_len: 0,
            rid: 0,
            label_flags: 0,
            label: 0,
            nh_id: 0,
            marker: None,
            marker_prefix_len: 0,
            mac: MacAddress::nil(),
            replace_prefix_len: 0,
            index: 0,
        }
    }
}

impl RouteRequest {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_route_req = vr_route_req::new();
        encoder.h_op = self.op as u32;
        encoder.rtr_vrf_id = self.vrf_id;
        encoder.rtr_family = self.family;
        encoder.rtr_prefix = Self::write_ip(&self.prefix);
        encoder.rtr_prefix_size = Self::prefix_size(self.prefix);
        encoder.rtr_prefix_len = self.prefix_len;
        encoder.rtr_rid = self.rid;
        encoder.rtr_label_flags = self.label_flags;
        encoder.rtr_label = self.label;
        encoder.rtr_nh_id = self.nh_id;
        encoder.rtr_marker = Self::write_ip(&self.marker);
        encoder.rtr_marker_size = Self::prefix_size(self.marker);
        encoder.rtr_marker_plen = self.marker_prefix_len;
        encoder.rtr_mac = utils::write_mac(self.mac);
        encoder.rtr_mac_size = if self.mac.is_nil() {
            0u32
        } else {
            libc::ETH_ALEN as u32
        };
        encoder.rtr_replace_plen = self.replace_prefix_len;
        encoder.rtr_index = self.index;

        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<RouteRequest, &'a str> {
        let decoder: vr_route_req = vr_route_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut rtr: RouteRequest = RouteRequest::default();
                rtr.op = decoder.h_op.try_into().unwrap();
                rtr.vrf_id = decoder.rtr_vrf_id;
                rtr.family = decoder.rtr_family;
                rtr.prefix = Self::read_ip(
                    decoder.rtr_family,
                    decoder.rtr_prefix,
                    decoder.rtr_prefix_size,
                );
                rtr.prefix_len = decoder.rtr_prefix_len;
                rtr.rid = decoder.rtr_rid;
                rtr.label_flags = decoder.rtr_label_flags;
                rtr.label = decoder.rtr_label;
                rtr.nh_id = decoder.rtr_nh_id;
                rtr.marker = Self::read_ip(
                    decoder.rtr_family,
                    decoder.rtr_marker,
                    decoder.rtr_marker_size,
                );
                rtr.marker_prefix_len = decoder.rtr_marker_plen;
                rtr.mac =
                    utils::read_mac_addr(decoder.rtr_mac, decoder.rtr_mac_size);
                rtr.replace_prefix_len = decoder.rtr_replace_plen;
                rtr.index = decoder.rtr_index;
                Ok(rtr)
            }
        }
    }

    fn prefix_size(ip: Option<IpAddr>) -> u32 {
        match ip {
            Some(IpAddr::V4(_)) => 4,
            Some(IpAddr::V6(_)) => 16,
            None => 0,
        }
    }

    fn write_ip(ip: &Option<IpAddr>) -> *mut i8 {
        let v: Vec<i8> = match ip {
            Some(IpAddr::V4(ip4)) => Self::write_ip4(*ip4),
            Some(IpAddr::V6(ip6)) => Self::write_ip6(*ip6),
            None => vec![],
        };
        utils::into_mut_ptr(&v)
    }

    fn read_ip(family: i32, ptr: *mut i8, size: u32) -> Option<IpAddr> {
        match family {
            libc::AF_INET if size == 4 => Some(IpAddr::V4(Self::read_ip4(ptr))),
            libc::AF_INET6 if size == 16 => {
                Some(IpAddr::V6(Self::read_ip6(ptr)))
            }
            _ => None,
        }
    }

    fn write_ip6(ip6: Ipv6Addr) -> Vec<i8> {
        let v: Vec<i8> = Vec::new();
        ip6.octets()
            .to_vec()
            .iter()
            .fold(v, |mut acc: Vec<i8>, &o| {
                acc.push(o as i8);
                acc
            })
    }

    fn read_ip6(ptr: *mut i8) -> Ipv6Addr {
        let octets = utils::free_buf(ptr, 16);
        Ipv6Addr::from(
            ((octets[0] as u128) << 120)
                | ((octets[1] as u128) << 112)
                | ((octets[2] as u128) << 104)
                | ((octets[3] as u128) << 96)
                | ((octets[4] as u128) << 88)
                | ((octets[5] as u128) << 80)
                | ((octets[6] as u128) << 72)
                | ((octets[7] as u128) << 64)
                | ((octets[8] as u128) << 56)
                | ((octets[9] as u128) << 48)
                | ((octets[10] as u128) << 40)
                | ((octets[11] as u128) << 32)
                | ((octets[12] as u128) << 24)
                | ((octets[13] as u128) << 16)
                | ((octets[14] as u128) << 8)
                | (octets[15] as u128),
        )
    }

    fn write_ip4(ip4: Ipv4Addr) -> Vec<i8> {
        let v: Vec<i8> = Vec::new();
        ip4.octets()
            .to_vec()
            .iter()
            .fold(v, |mut acc: Vec<i8>, &o| {
                acc.push(o as i8);
                acc
            })
    }

    fn read_ip4(ptr: *mut i8) -> Ipv4Addr {
        let octets = utils::free_buf(ptr, 4);
        Ipv4Addr::from(
            ((octets[0] as u32) << 24)
                | ((octets[1] as u32) << 16)
                | ((octets[2] as u32) << 8)
                | (octets[3] as u32),
        )
    }
}
