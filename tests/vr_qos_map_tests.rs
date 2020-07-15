// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_qos_map {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_qos_map::QosMapRequest;

    #[test]
    fn empty_request() {
        let qmr: QosMapRequest = QosMapRequest::default();
        let byte = qmr.write().unwrap();
        let qmr: QosMapRequest = QosMapRequest::read(byte).unwrap();
        assert_eq!(qmr.op, SandeshOp::Add);
        assert_eq!(qmr.rid, 0);
        assert_eq!(qmr.id, 0);
        assert_eq!(qmr.dscp, vec![]);
        assert_eq!(qmr.dscp_fc_id, vec![]);
        assert_eq!(qmr.mpls_qos, vec![]);
        assert_eq!(qmr.mpls_qos_fc_id, vec![]);
        assert_eq!(qmr.dotonep, vec![]);
        assert_eq!(qmr.dotonep_fc_id, vec![]);
        assert_eq!(qmr.marker, 0);
    }

    #[test]
    fn complex_request() {
        let mut qmr: QosMapRequest = QosMapRequest::default();

        qmr.op = SandeshOp::Dump;
        qmr.rid = 1;
        qmr.id = 2;
        qmr.dscp = vec![1, 2, 3, 4, 5];
        qmr.dscp_fc_id = vec![1, 2, 3, 4, 5];
        qmr.mpls_qos = vec![1, 2, 3, 4, 5];
        qmr.mpls_qos_fc_id = vec![1, 2, 3, 4, 5];
        qmr.dotonep = vec![1, 2, 3, 4, 5];
        qmr.dotonep_fc_id = vec![1, 2, 3, 4, 5];
        qmr.marker = 3;

        let byte = qmr.write().unwrap();
        let qmr: QosMapRequest = QosMapRequest::read(byte).unwrap();

        assert_eq!(qmr.op, SandeshOp::Dump);
        assert_eq!(qmr.rid, 1);
        assert_eq!(qmr.id, 2);
        assert_eq!(qmr.dscp, vec![1, 2, 3, 4, 5]);
        assert_eq!(qmr.dscp_fc_id, vec![1, 2, 3, 4, 5]);
        assert_eq!(qmr.mpls_qos, vec![1, 2, 3, 4, 5]);
        assert_eq!(qmr.mpls_qos_fc_id, vec![1, 2, 3, 4, 5]);
        assert_eq!(qmr.dotonep, vec![1, 2, 3, 4, 5]);
        assert_eq!(qmr.dotonep_fc_id, vec![1, 2, 3, 4, 5]);
        assert_eq!(qmr.marker, 3);
    }
}
