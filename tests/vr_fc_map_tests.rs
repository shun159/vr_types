// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_fc_map {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_fc_map::FcMapRequest;

    #[test]
    fn empty_request() {
        let fmr: FcMapRequest = FcMapRequest::default();
        let byte = fmr.write().unwrap();
        let fmr: FcMapRequest = FcMapRequest::read(byte).unwrap();
        assert_eq!(fmr.op, SandeshOp::Add);
        assert_eq!(fmr.rid, 0);
        assert_eq!(fmr.id, vec![]);
        assert_eq!(fmr.dscp, vec![]);
        assert_eq!(fmr.mpls_qos, vec![]);
        assert_eq!(fmr.dotonep, vec![]);
        assert_eq!(fmr.queue_id, vec![]);
        assert_eq!(fmr.marker, 0);
    }

    #[test]
    fn complex_request() {
        let mut fmr: FcMapRequest = FcMapRequest::default();

        fmr.op = SandeshOp::Dump;
        fmr.rid = 1;
        fmr.id = vec![1, 2, 3, 4, 5];
        fmr.dscp = vec![1, 2, 3, 4, 5];
        fmr.mpls_qos = vec![1, 2, 3, 4, 5];
        fmr.dotonep = vec![1, 2, 3, 4, 5];
        fmr.queue_id = vec![1, 2, 3, 4, 5];
        fmr.marker = 3;

        let byte = fmr.write().unwrap();
        let fmr: FcMapRequest = FcMapRequest::read(byte).unwrap();

        assert_eq!(fmr.op, SandeshOp::Dump);
        assert_eq!(fmr.rid, 1);
        assert_eq!(fmr.id, vec![1, 2, 3, 4, 5]);
        assert_eq!(fmr.dscp, vec![1, 2, 3, 4, 5]);
        assert_eq!(fmr.mpls_qos, vec![1, 2, 3, 4, 5]);
        assert_eq!(fmr.dotonep, vec![1, 2, 3, 4, 5]);
        assert_eq!(fmr.queue_id, vec![1, 2, 3, 4, 5]);
        assert_eq!(fmr.marker, 3);
    }
}
