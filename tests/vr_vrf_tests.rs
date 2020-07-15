// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_vrf {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_vrf::VrfRequest;

    #[test]
    fn empty_request() {
        let vrf: VrfRequest = VrfRequest::default();
        let bytes = vrf.write().unwrap();
        let vrf: VrfRequest = VrfRequest::read(bytes).unwrap();
        assert_eq!(vrf.op, SandeshOp::Add);
        assert_eq!(vrf.rid, 0);
        assert_eq!(vrf.idx, 0);
        assert_eq!(vrf.flags, 0);
        assert_eq!(vrf.hbfl_vif_idx, 0);
        assert_eq!(vrf.hbfr_vif_idx, 0);
        assert_eq!(vrf.marker, 0);
    }
    #[test]
    fn complex_request() {
        let mut vrf: VrfRequest = VrfRequest::default();
        vrf.op = SandeshOp::Dump;
        vrf.rid = 1;
        vrf.idx = 1;
        vrf.flags = 1;
        vrf.hbfl_vif_idx = 1;
        vrf.hbfr_vif_idx = 2;
        vrf.marker = 1;

        let bytes = vrf.write().unwrap();
        let vrf: VrfRequest = VrfRequest::read(bytes).unwrap();

        assert_eq!(vrf.op, SandeshOp::Dump);
        assert_eq!(vrf.rid, 1);
        assert_eq!(vrf.idx, 1);
        assert_eq!(vrf.flags, 1);
        assert_eq!(vrf.hbfl_vif_idx, 1);
        assert_eq!(vrf.hbfr_vif_idx, 2);
        assert_eq!(vrf.marker, 1);
    }
}
