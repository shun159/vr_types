// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_vxlan {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_vxlan::VxlanRequest;

    #[test]
    fn empty_request() {
        let vxlanr: VxlanRequest = VxlanRequest::default();
        let bytes = vxlanr.write().unwrap();
        let vxlanr: VxlanRequest = VxlanRequest::read(bytes).unwrap();
        assert_eq!(vxlanr.op, SandeshOp::Add);
        assert_eq!(vxlanr.rid, 0);
        assert_eq!(vxlanr.vnid, 0);
        assert_eq!(vxlanr.nhid, 0);
    }

    #[test]
    fn complex_request() {
        let mut vxlanr: VxlanRequest = VxlanRequest::default();
        vxlanr.op = SandeshOp::Dump;
        vxlanr.rid = 1;
        vxlanr.vnid = 1;
        vxlanr.nhid = 1;
        let bytes = vxlanr.write().unwrap();
        let vxlanr: VxlanRequest = VxlanRequest::read(bytes).unwrap();
        assert_eq!(vxlanr.op, SandeshOp::Dump);
        assert_eq!(vxlanr.rid, 1);
        assert_eq!(vxlanr.vnid, 1);
        assert_eq!(vxlanr.nhid, 1);
    }
}
