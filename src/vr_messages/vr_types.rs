// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::error::CodecError;
use super::vr_flow;
use super::vr_interface;
use super::vr_nexthop;
use super::vr_pkt_droplog;
use super::vr_types_binding::*;
use crate::utils;

use byteorder::{NetworkEndian, ReadBytesExt};
use libc::{time_t, AF_INET6};
use std::io::Cursor;
use std::mem::{size_of, size_of_val};
use std::os::raw::{c_int, c_uint, c_void};

pub trait VrSandesh {
    type Type;

    fn new() -> Self::Type;

    fn obj_len(&self) -> usize {
        4usize * size_of::<Self::Type>()
    }

    fn write(&self) -> Result<Vec<u8>, CodecError> {
        unsafe {
            let mut error = 0;
            let wsandesh = self.as_c_void();
            let buf = utils::alloc_buf(self.obj_len());
            let buf_len = self.obj_len();
            match self.write_binary_fn()(wsandesh, buf, buf_len, &mut error) {
                wxfer if wxfer >= 0 && error == 0 => {
                    Ok(utils::free_buf::<u8>(buf, wxfer as usize))
                }
                _ => Err(CodecError::Write(error)),
            }
        }
    }

    fn read(&self, buf: &Vec<u8>) -> Result<i32, CodecError> {
        unsafe {
            let mut error = 0;
            let buf_ptr = Box::into_raw(buf.clone().into_boxed_slice()) as *mut u8;
            let buf_len = self.obj_len();
            let rsandesh = self.as_c_void();
            match self.read_binary_fn()(rsandesh, buf_ptr, buf_len, &mut error) {
                rxfer if rxfer >= 0 && error == 0 => Ok(rxfer),
                _ => Err(CodecError::Read(error)),
            }
        }
    }

    fn as_c_void(&self) -> *mut c_void;

    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        error: *mut c_int,
    ) -> i32;

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        buf_len: usize,
        error: *mut c_int,
    ) -> i32;
}

impl Default for vr_nexthop_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_nexthop_req {
    type Type = vr_nexthop_req;

    fn new() -> Self {
        vr_nexthop_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_nexthop_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_nexthop_req_read_binary_from_buffer
    }

    // From vr_nexthop_req_get_size on dp-core/vr_nexthop.c
    fn obj_len(&self) -> usize {
        let mut size = 4usize * size_of::<Self>();
        size += self.nhr_encap_size as usize;

        if 0 < self.nhr_nh_list_size {
            size += 4 * self.nhr_nh_list_size as usize
        }
        if 0 < self.nhr_label_list_size {
            size += 4 * self.nhr_label_list_size as usize
        }
        size += self.nhr_pbb_mac_size as usize;

        if (self.nhr_type == vr_nexthop::NhType::Tunnel as i8)
            && (0 != self.nhr_flags & vr_nexthop::NhFlag::TunnelUdp as u32)
            && (self.nhr_family as i32 == AF_INET6)
        {
            size += (vr_flow::VR_IP6_ADDRESS_LEN * 2 * 4) as usize;
        }

        return size;
    }
}

impl Default for vr_interface_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_interface_req {
    type Type = vr_interface_req;

    fn new() -> Self {
        vr_interface_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_interface_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_interface_req_read_binary_from_buffer
    }

    // From vr_interface_req_get_size on dp-core/vr_interface.c
    fn obj_len(&self) -> usize {
        let mut size = 4 * size_of_val(self) as u32;
        size += 2 * vr_interface::VIF_MAX_MIRROR_MD_SIZE;
        if 0 < self.vifr_queue_ierrors_to_lcore_size {
            size += 8 * self.vifr_queue_ierrors_to_lcore_size;
        }

        return size as usize;
    }
}

impl Default for vr_vxlan_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_vxlan_req {
    type Type = vr_vxlan_req;

    fn new() -> Self {
        vr_vxlan_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_vxlan_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_vxlan_req_read_binary_from_buffer
    }
}

impl Default for vr_route_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_route_req {
    type Type = vr_route_req;

    fn new() -> Self {
        vr_route_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_route_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_route_req_read_binary_from_buffer
    }
}

impl Default for vr_mpls_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_mpls_req {
    type Type = vr_mpls_req;

    fn new() -> Self {
        vr_mpls_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_mpls_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_mpls_req_read_binary_from_buffer
    }
}

impl Default for vr_mirror_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_mirror_req {
    type Type = vr_mirror_req;

    fn new() -> Self {
        vr_mirror_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_mirror_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_mirror_req_read_binary_from_buffer
    }
}

impl Default for vr_vrf_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_vrf_req {
    type Type = vr_vrf_req;

    fn new() -> Self {
        vr_vrf_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_vrf_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_vrf_req_read_binary_from_buffer
    }
}

impl Default for vr_flow_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_flow_req {
    type Type = vr_flow_req;

    fn new() -> Self {
        vr_flow_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_flow_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_flow_req_read_binary_from_buffer
    }
}

impl Default for vr_vrf_assign_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_vrf_assign_req {
    type Type = vr_vrf_assign_req;

    fn new() -> Self {
        vr_vrf_assign_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_vrf_assign_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_vrf_assign_req_read_binary_from_buffer
    }
}

impl Default for vr_vrf_stats_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_vrf_stats_req {
    type Type = vr_vrf_stats_req;

    fn new() -> Self {
        vr_vrf_stats_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_vrf_stats_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_vrf_stats_req_read_binary_from_buffer
    }
}

impl Default for vr_response {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_response {
    type Type = vr_response;

    fn new() -> Self {
        vr_response::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_response_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_response_read_binary_from_buffer
    }
}

impl Default for vrouter_ops {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vrouter_ops {
    type Type = vrouter_ops;

    fn new() -> Self {
        vrouter_ops::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vrouter_ops_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vrouter_ops_read_binary_from_buffer
    }
}

impl Default for vr_mem_stats_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_mem_stats_req {
    type Type = vr_mem_stats_req;

    fn new() -> Self {
        vr_mem_stats_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_mem_stats_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_mem_stats_req_read_binary_from_buffer
    }
}

impl Default for vr_pkt_drop_log_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
union PktLogAddr {
    ipv4: libc::in_addr,
    ipv6: libc::in6_addr,
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct vr_drop_loc {
    file: c_int,
    line: c_uint,
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct vr_pkt_drop_log {
    timestamp: time_t,
    vp_type: u8,
    drop_reason: u16,
    vif_idx: u16,
    nh_id: u32,
    src: PktLogAddr,
    dst: PktLogAddr,
    sport: u16,
    dport: u16,
    drop_loc: vr_drop_loc,
    pkt_len: u16,
    pkt_header: [u8; 100],
}

impl VrSandesh for vr_pkt_drop_log_req {
    type Type = vr_pkt_drop_log_req;

    fn new() -> Self {
        let mut req = vr_pkt_drop_log_req::default();
        req.vdl_pkt_droplog_arr_size = 0;
        req
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_pkt_drop_log_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_pkt_drop_log_req_read_binary_from_buffer
    }

    fn obj_len(&self) -> usize {
        let mut size = 4 * size_of::<Self>();
        size += vr_pkt_droplog::VR_PKT_DROP_LOG_MAX as usize;
        size += size_of::<vr_pkt_drop_log>();
        size
    }
}

impl Default for vr_drop_stats_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_drop_stats_req {
    type Type = vr_drop_stats_req;

    fn new() -> Self {
        vr_drop_stats_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_drop_stats_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_drop_stats_req_read_binary_from_buffer
    }
}

impl Default for vr_qos_map_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_qos_map_req {
    type Type = vr_qos_map_req;

    fn new() -> Self {
        vr_qos_map_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_qos_map_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_qos_map_req_read_binary_from_buffer
    }

    fn obj_len(&self) -> usize {
        let mut size = 4 * size_of_val(self) as u32;
        size += 4 * self.qmr_dscp_size;
        size += 4 * self.qmr_dscp_fc_id_size;
        size += 4 * self.qmr_mpls_qos_size;
        size += 4 * self.qmr_mpls_qos_fc_id_size;
        size += 4 * self.qmr_dotonep_size;
        size += 4 * self.qmr_dotonep_fc_id_size;
        size as usize
    }
}

impl Default for vr_fc_map_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_fc_map_req {
    type Type = vr_fc_map_req;

    fn new() -> Self {
        vr_fc_map_req::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_fc_map_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_fc_map_req_read_binary_from_buffer
    }
}

impl Default for vr_flow_response {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_flow_response {
    type Type = vr_flow_response;

    fn new() -> Self {
        vr_flow_response::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_flow_response_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_flow_response_read_binary_from_buffer
    }
}

impl Default for vr_flow_table_data {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_flow_table_data {
    type Type = vr_flow_table_data;

    fn new() -> Self {
        vr_flow_table_data::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_flow_table_data_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_flow_table_data_read_binary_from_buffer
    }
}

impl Default for vr_bridge_table_data {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_bridge_table_data {
    type Type = vr_bridge_table_data;

    fn new() -> Self {
        vr_bridge_table_data::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_bridge_table_data_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_bridge_table_data_read_binary_from_buffer
    }
}

impl Default for vr_hugepage_config {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl VrSandesh for vr_hugepage_config {
    type Type = vr_hugepage_config;

    fn new() -> Self {
        vr_hugepage_config::default()
    }

    fn as_c_void(&self) -> *mut c_void {
        utils::into_raw_ptr(&*self) as *mut c_void
    }

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_hugepage_config_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        err: *mut c_int,
    ) -> i32 {
        vr_hugepage_config_read_binary_from_buffer
    }
}

// sandesh info utils

impl sandesh_info_t {
    pub fn sname_from_bytes<'a>(buf: &'a Vec<u8>) -> &'a str {
        let buf_len = buf.len();
        let mut c = Cursor::new(&buf);
        let sname_len = c.read_u32::<NetworkEndian>().unwrap() as usize;
        let offset = c.position() as usize;
        if sname_len + offset <= buf_len {
            let v = &buf[offset..sname_len + offset];
            std::str::from_utf8(v).unwrap()
        } else {
            ""
        }
    }
}
