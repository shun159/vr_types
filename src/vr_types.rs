use crate::utils;
use crate::vr_flow;
use crate::vr_interface;
use crate::vr_nexthop;
use crate::vr_pkt_droplog;
use crate::vr_types_binding::*;

use libc::{time_t, AF_INET6};
use std::mem::{size_of, size_of_val};
use std::os::raw::{c_int, c_uint, c_void};

pub trait VrSandesh {
    type Type;

    fn new() -> Self::Type;

    fn obj_len(&self) -> usize {
        4usize * size_of::<Self::Type>()
    }

    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        wsandesh: *mut c_void,
        buf: *mut u8,
        len: usize,
        error: *mut c_int) -> i32;

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(
        rsandesh: *mut c_void,
        buf: *mut u8,
        buf_len: usize,
        error: *mut c_int) -> i32;
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_nexthop_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

        if (self.nhr_type == vr_nexthop::NH_TUNNEL)
            && (0 != self.nhr_flags & vr_nexthop::NH_FLAG_TUNNEL_UDP)
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_interface_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_vxlan_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_route_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_mpls_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_mirror_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_vrf_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_flow_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_vrf_assign_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_vrf_stats_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_response_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vrouter_ops_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_mem_stats_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_pkt_drop_log_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_pkt_drop_log_req_read_binary_from_buffer
    }

    fn obj_len(&self) -> usize {
        4usize * size_of::<Self>()
            + (vr_pkt_droplog::VR_PKT_DROP_LOG_MAX as usize + size_of::<vr_pkt_drop_log>())
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_drop_stats_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_qos_map_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_fc_map_req_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_flow_response_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_flow_table_data_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_bridge_table_data_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
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

    // write_binary_to_buffer function
    fn write_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(wsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_hugepage_config_write_binary_to_buffer
    }

    fn read_binary_fn(
        &self,
    ) -> unsafe extern "C" fn(rsandesh: *mut c_void, buf: *mut u8, len: usize, err: *mut c_int) -> i32
    {
        vr_hugepage_config_read_binary_from_buffer
    }
}

// sandesh info utils

impl sandesh_info_t {
    pub fn to_binary<T: VrSandesh>(req: T) -> Result<Vec<u8>, i32> {
        unsafe {
            let mut error = 0;
            let req_ptr = utils::into_raw_ptr(&req) as *mut c_void;
            let buf = utils::alloc_buf(req.obj_len());
            let buf_len = req.obj_len();
            match req.write_binary_fn()(req_ptr, buf, buf_len, &mut error) {
                wxfer if wxfer >= 0 && error == 0 => Ok(utils::free_buf(buf, wxfer as usize)),
                _ => Err(error),
            }
        }
    }

    pub fn read<T>(_buf: &Vec<u8>) -> Result<impl VrSandesh, i32> {
        Ok(vr_nexthop_req::new())
    }
}

// tests

#[cfg(test)]
mod test_encode_types {
    use crate::vr_types::VrSandesh;
    use crate::vr_types_binding::*;

    #[test]
    fn vr_nexthop_req() {
        let req = vr_nexthop_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(214, res.len())
    }

    #[test]
    fn vr_interface_req() {
        let req = vr_interface_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(724, res.len())
    }

    #[test]
    fn vr_vxlan_req() {
        let req = vr_vxlan_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(43, res.len())
    }

    #[test]
    fn vr_route_req() {
        let req = vr_route_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(114, res.len())
    }

    #[test]
    fn vr_mpls_req() {
        let req = vr_mpls_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(49, res.len())
    }

    #[test]
    fn vr_mirror_req() {
        let req = vr_mirror_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(75, res.len())
    }

    #[test]
    fn vr_vrf_req() {
        let req = vr_vrf_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(62, res.len())
    }

    #[test]
    fn vr_flow_req() {
        let req = vr_flow_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(272, res.len())
    }

    #[test]
    fn vr_vrf_assign_req() {
        let req = vr_vrf_assign_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(63, res.len())
    }

    #[test]
    fn vr_vrf_stats_req() {
        let req = vr_vrf_stats_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(352, res.len())
    }

    #[test]
    fn vr_response() {
        let req = vr_response::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(30, res.len())
    }

    #[test]
    fn vr_mem_stats_req() {
        let req = vr_mem_stats_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(803, res.len())
    }

    #[test]
    fn vr_pkt_drop_log_req() {
        let req = vr_pkt_drop_log_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(74, res.len())
    }

    #[test]
    fn vr_drop_stats_req() {
        let req = vr_drop_stats_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(593, res.len())
    }

    #[test]
    fn vr_qos_map_req() {
        let req = vr_qos_map_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(89, res.len())
    }

    #[test]
    fn vr_fc_map_req() {
        let req = vr_fc_map_req::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(75, res.len())
    }

    #[test]
    fn vr_flow_response() {
        let req = vr_flow_response::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(70, res.len())
    }

    #[test]
    fn vr_flow_table_data() {
        let req = vr_flow_table_data::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(163, res.len())
    }

    #[test]
    fn vr_bridge_table_data() {
        let req = vr_bridge_table_data::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(56, res.len())
    }

    #[test]
    fn vr_hugepage_config() {
        let req = vr_hugepage_config::new();
        let res = sandesh_info_t::to_binary(req).unwrap();
        assert_eq!(53, res.len())
    }
}
