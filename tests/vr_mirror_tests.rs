// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_mirror {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_mirror::MirrorRequest;

    #[test]
    fn empty_requset() {
        let mirr: MirrorRequest = MirrorRequest::default();
        let bytes = mirr.write().unwrap();
        let mirr: MirrorRequest = MirrorRequest::read(bytes).unwrap();
        assert_eq!(mirr.op, SandeshOp::Add);
        assert_eq!(mirr.index, 0);
        assert_eq!(mirr.rid, 0);
        assert_eq!(mirr.nhid, 0);
        assert_eq!(mirr.users, 0);
        assert_eq!(mirr.flags, 0);
        assert_eq!(mirr.marker, 0);
        assert_eq!(mirr.vni, 0);
        assert_eq!(mirr.vlan, 0);
    }

    #[test]
    fn complex_request() {
        let mut mirr: MirrorRequest = MirrorRequest::default();
        mirr.op = SandeshOp::Dump;
        mirr.index = 1;
        mirr.rid = 1;
        mirr.nhid = 1;
        mirr.users = 1;
        mirr.flags = 1;
        mirr.marker = 1;
        mirr.vni = 1;
        mirr.vlan = 1;

        let bytes = mirr.write().unwrap();
        let mirr: MirrorRequest = MirrorRequest::read(bytes).unwrap();

        assert_eq!(mirr.op, SandeshOp::Dump);
        assert_eq!(mirr.index, 1);
        assert_eq!(mirr.rid, 1);
        assert_eq!(mirr.nhid, 1);
        assert_eq!(mirr.users, 1);
        assert_eq!(mirr.flags, 1);
        assert_eq!(mirr.marker, 1);
        assert_eq!(mirr.vni, 1);
        assert_eq!(mirr.vlan, 1);
    }
}
