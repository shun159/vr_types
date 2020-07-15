// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_route {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_route::RouteRequest;
    use eui48::MacAddress;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    #[test]
    fn empty_request() {
        let rtr: RouteRequest = RouteRequest::default();
        let bytes = rtr.write().unwrap();
        let rtr: RouteRequest = RouteRequest::read(bytes).unwrap();
        assert_eq!(rtr.op, SandeshOp::Add);
        assert_eq!(rtr.vrf_id, 0);
        assert_eq!(rtr.family, 0);
        assert_eq!(rtr.prefix, None);
        assert_eq!(rtr.prefix_len, 0);
        assert_eq!(rtr.rid, 0);
        assert_eq!(rtr.label_flags, 0);
        assert_eq!(rtr.label, 0);
        assert_eq!(rtr.nh_id, 0);
        assert_eq!(rtr.marker, None);
        assert_eq!(rtr.marker_prefix_len, 0);
        assert_eq!(rtr.mac, MacAddress::nil());
        assert_eq!(rtr.replace_prefix_len, 0);
        assert_eq!(rtr.index, 0);
    }

    #[test]
    fn complex_request_1() {
        let mut rtr: RouteRequest = RouteRequest::default();
        rtr.op = SandeshOp::Dump;
        rtr.vrf_id = 1;
        rtr.family = libc::AF_INET as i32;
        rtr.prefix = Some(IpAddr::V4(Ipv4Addr::LOCALHOST));
        rtr.prefix_len = 32;
        rtr.rid = 1;
        rtr.label_flags = 1;
        rtr.label = 1;
        rtr.nh_id = 1;
        rtr.marker = Some(IpAddr::V4(Ipv4Addr::LOCALHOST));
        rtr.marker_prefix_len = 32;
        rtr.mac = MacAddress::broadcast();
        rtr.replace_prefix_len = 32;
        rtr.index = 1;

        let bytes = rtr.write().unwrap();
        let rtr: RouteRequest = RouteRequest::read(bytes).unwrap();

        assert_eq!(rtr.op, SandeshOp::Dump);
        assert_eq!(rtr.vrf_id, 1);
        assert_eq!(rtr.family, libc::AF_INET as i32);
        assert_eq!(rtr.prefix, Some(IpAddr::V4(Ipv4Addr::LOCALHOST)));
        assert_eq!(rtr.prefix_len, 32);
        assert_eq!(rtr.rid, 1);
        assert_eq!(rtr.label_flags, 1);
        assert_eq!(rtr.label, 1);
        assert_eq!(rtr.nh_id, 1);
        assert_eq!(rtr.marker, Some(IpAddr::V4(Ipv4Addr::LOCALHOST)));
        assert_eq!(rtr.marker_prefix_len, 32);
        assert_eq!(rtr.mac, MacAddress::broadcast());
        assert_eq!(rtr.replace_prefix_len, 32);
        assert_eq!(rtr.index, 1);
    }

    #[test]
    fn complex_request_2() {
        let mut rtr: RouteRequest = RouteRequest::default();
        rtr.op = SandeshOp::Dump;
        rtr.vrf_id = 1;
        rtr.family = libc::AF_INET6 as i32;
        rtr.prefix = Some(IpAddr::V6(Ipv6Addr::LOCALHOST));
        rtr.prefix_len = 128;
        rtr.rid = 1;
        rtr.label_flags = 1;
        rtr.label = 1;
        rtr.nh_id = 1;
        rtr.marker = Some(IpAddr::V6(Ipv6Addr::LOCALHOST));
        rtr.marker_prefix_len = 128;
        rtr.mac = MacAddress::broadcast();
        rtr.replace_prefix_len = 128;
        rtr.index = 1;

        let bytes = rtr.write().unwrap();
        let rtr: RouteRequest = RouteRequest::read(bytes).unwrap();

        assert_eq!(rtr.op, SandeshOp::Dump);
        assert_eq!(rtr.vrf_id, 1);
        assert_eq!(rtr.family, libc::AF_INET6 as i32);
        assert_eq!(rtr.prefix, Some(IpAddr::V6(Ipv6Addr::LOCALHOST)));
        assert_eq!(rtr.prefix_len, 128);
        assert_eq!(rtr.rid, 1);
        assert_eq!(rtr.label_flags, 1);
        assert_eq!(rtr.label, 1);
        assert_eq!(rtr.nh_id, 1);
        assert_eq!(rtr.marker, Some(IpAddr::V6(Ipv6Addr::LOCALHOST)));
        assert_eq!(rtr.marker_prefix_len, 128);
        assert_eq!(rtr.mac, MacAddress::broadcast());
        assert_eq!(rtr.replace_prefix_len, 128);
        assert_eq!(rtr.index, 1);
    }
}
