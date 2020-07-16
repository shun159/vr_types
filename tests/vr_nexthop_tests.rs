// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_nexthop {
    use eui48::MacAddress;
    use std::net::{Ipv4Addr, Ipv6Addr};
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_nexthop::{NexthopRequest, NhType};

    #[test]
    fn empty_request() {
        let nhreq: NexthopRequest = NexthopRequest::default();
        let bytes = nhreq.write().unwrap();
        let nhreq: NexthopRequest = NexthopRequest::read(bytes).unwrap();
        assert_eq!(nhreq.op, SandeshOp::Add);
        assert_eq!(nhreq._type, NhType::Dead);
        assert_eq!(nhreq.family, 0);
        assert_eq!(nhreq.id, 0);
        assert_eq!(nhreq.rid, 0);
        assert_eq!(nhreq.encap_oif_id, 0);
        assert_eq!(nhreq.encap_len, 0);
        assert_eq!(nhreq.encap_family, 0);
        assert_eq!(nhreq.vrf, 0);
        assert_eq!(nhreq.tun_sip, Ipv4Addr::UNSPECIFIED);
        assert_eq!(nhreq.tun_dip, Ipv4Addr::UNSPECIFIED);
        assert_eq!(nhreq.tun_sport, 0);
        assert_eq!(nhreq.tun_dport, 0);
        assert_eq!(nhreq.ref_cnt, 0);
        assert_eq!(nhreq.marker, 0);
        assert_eq!(nhreq.flags, 0);
        assert_eq!(nhreq.encap, vec![]);
        assert_eq!(nhreq.nh_list, vec![]);
        assert_eq!(nhreq.label_list, vec![]);
        assert_eq!(nhreq.tun_sip6, Ipv6Addr::UNSPECIFIED);
        assert_eq!(nhreq.tun_dip6, Ipv6Addr::UNSPECIFIED);
        assert_eq!(nhreq.ecmp_config_hash, 0);
        assert_eq!(nhreq.pbb_mac, MacAddress::nil());
        assert_eq!(nhreq.encap_crypt_oif_id, 0);
        assert_eq!(nhreq.crypt_path_available, 0);
        assert_eq!(nhreq.crypt_traffic, 0);
        assert_eq!(nhreq.rw_dst_mac, MacAddress::nil());
        assert_eq!(nhreq.transport_label, 0);
    }

    #[test]
    fn complex_request() {
        let mut nhreq: NexthopRequest = NexthopRequest::default();
        nhreq.op = SandeshOp::Get;
        nhreq._type = NhType::Composite;
        nhreq.family = 7; // AF_BRIDGE
        nhreq.id = 1;
        nhreq.rid = 1;
        nhreq.encap_oif_id = 1;
        nhreq.encap_len = 5;
        nhreq.encap_family = 1;
        nhreq.vrf = 1;
        nhreq.tun_sip = Ipv4Addr::LOCALHOST;
        nhreq.tun_dip = Ipv4Addr::LOCALHOST;
        nhreq.tun_sport = 1;
        nhreq.tun_dport = 1;
        nhreq.ref_cnt = 1;
        nhreq.marker = 1;
        nhreq.flags = 1;
        nhreq.encap = vec![1, 2, 3, 4, 5];
        nhreq.nh_list = vec![1, 2, 3, 4, 5];
        nhreq.label_list = vec![1, 2, 3, 4, 5];
        nhreq.nh_count = 5;
        nhreq.tun_sip6 = Ipv6Addr::LOCALHOST;
        nhreq.tun_dip6 = Ipv6Addr::LOCALHOST;
        nhreq.ecmp_config_hash = 1;
        nhreq.pbb_mac = MacAddress::broadcast();
        nhreq.encap_crypt_oif_id = 1;
        nhreq.crypt_traffic = 1;
        nhreq.crypt_path_available = 1;
        nhreq.rw_dst_mac = MacAddress::broadcast();
        nhreq.transport_label = 1;

        let bytes = nhreq.write().unwrap();
        let nhreq: NexthopRequest = NexthopRequest::read(bytes).unwrap();

        assert_eq!(nhreq.op, SandeshOp::Get);
        assert_eq!(nhreq._type, NhType::Composite);
        assert_eq!(nhreq.family, 7);
        assert_eq!(nhreq.id, 1);
        assert_eq!(nhreq.rid, 1);
        assert_eq!(nhreq.encap_oif_id, 1);
        assert_eq!(nhreq.encap_len, 5);
        assert_eq!(nhreq.encap_family, 1);
        assert_eq!(nhreq.vrf, 1);
        assert_eq!(nhreq.tun_sip, Ipv4Addr::LOCALHOST);
        assert_eq!(nhreq.tun_dip, Ipv4Addr::LOCALHOST);
        assert_eq!(nhreq.tun_sport, 1);
        assert_eq!(nhreq.tun_dport, 1);
        assert_eq!(nhreq.ref_cnt, 1);
        assert_eq!(nhreq.marker, 1);
        assert_eq!(nhreq.flags, 1);
        assert_eq!(nhreq.encap, vec![1, 2, 3, 4, 5]);
        assert_eq!(nhreq.nh_list, vec![1, 2, 3, 4, 5]);
        assert_eq!(nhreq.label_list, vec![1, 2, 3, 4, 5]);
        assert_eq!(nhreq.tun_sip6, Ipv6Addr::LOCALHOST);
        assert_eq!(nhreq.tun_dip6, Ipv6Addr::LOCALHOST);
        assert_eq!(nhreq.ecmp_config_hash, 1);
        assert_eq!(nhreq.pbb_mac, MacAddress::broadcast());
        assert_eq!(nhreq.encap_crypt_oif_id, 1);
        assert_eq!(nhreq.crypt_path_available, 1);
        assert_eq!(nhreq.crypt_traffic, 1);
        assert_eq!(nhreq.rw_dst_mac, MacAddress::broadcast());
        assert_eq!(nhreq.transport_label, 1);
    }
}
