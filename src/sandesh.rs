use crate::vr_types_binding;
use std::ffi::CString;
use std::os::raw::{c_char};

pub fn find_fn_by_name(name: String) -> Option<*mut vr_types_binding::sandesh_info_t> {
    unsafe {
        let sname = string_to_c_char(&name);
        let sinfo = vr_types_binding::vr_find_sandesh_info(sname);
        if !(*sinfo).name.is_null() { Some(sinfo) } else { None }
    }
}

impl Default for vr_types_binding::_vr_nexthop_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_interface_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_vxlan_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_route_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_mpls_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_mirror_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_vrf_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_flow_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_vrf_assign_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_vrf_stats_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_response {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vrouter_ops {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_mem_stats_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_pkt_drop_log_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_drop_stats_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_qos_map_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_fc_map_req {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_flow_response {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_flow_table_data {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_bridge_table_data {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for vr_types_binding::_vr_hugepage_config {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

// private functions

fn string_to_c_char(s: &String) -> *const c_char {
    let c_str = CString::new(&*s.to_string()).unwrap();
    let c_string: *const c_char = c_str.as_ptr() as *const c_char;
    return c_string
}
