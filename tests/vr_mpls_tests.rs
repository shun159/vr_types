// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_mpls {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_mpls::MplsRequest;

    #[test]
    fn empty_request() {
        let mr: MplsRequest = MplsRequest::default();
        let bytes = mr.write().unwrap();
        let mr: MplsRequest = MplsRequest::read(bytes).unwrap();
        assert_eq!(mr.op, SandeshOp::Add);
        assert_eq!(mr.rid, 0);
        assert_eq!(mr.label, 0);
        assert_eq!(mr.nhid, 0);
    }

    #[test]
    fn complex_request() {
        let mut mr: MplsRequest = MplsRequest::default();
        mr.op = SandeshOp::Dump;
        mr.rid = 1;
        mr.label = 1;
        mr.nhid = 1;
        let bytes = mr.write().unwrap();
        let mr: MplsRequest = MplsRequest::read(bytes).unwrap();
        assert_eq!(mr.op, SandeshOp::Dump);
        assert_eq!(mr.rid, 1);
        assert_eq!(mr.label, 1);
        assert_eq!(mr.nhid, 1);
    }
}
