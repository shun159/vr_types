// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_pkt_droplog {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_pkt_droplog::PktDropLog;

    #[test]
    fn empty_request() {
        let vdl: PktDropLog = PktDropLog::default();
        let bytes = vdl.write().unwrap();
        let vdl: PktDropLog = PktDropLog::read(bytes).unwrap();
        assert_eq!(vdl.op, SandeshOp::Add);
        assert_eq!(vdl.rid, 0);
        assert_eq!(vdl.core, 0);
        assert_eq!(vdl.log_idx, 0);
        assert_eq!(vdl.max_num_cores, 0);
        assert_eq!(vdl.pkt_droplog_max_bufsz, 0);
        assert_eq!(vdl.pkt_droplog_en, 0);
        assert_eq!(vdl.pkt_droplog_sysctl_en, 0);
        assert_eq!(vdl.pkt_droplog_arr, vec![]);
    }

    #[test]
    fn complex_request() {
        let mut vdl: PktDropLog = PktDropLog::default();

        vdl.op = SandeshOp::Dump;
        vdl.rid = 1;
        vdl.core = 2;
        vdl.log_idx = 3;
        vdl.max_num_cores = 4;
        vdl.pkt_droplog_max_bufsz = 5;
        vdl.pkt_droplog_en = 6;
        vdl.pkt_droplog_sysctl_en = 7;
        vdl.pkt_droplog_arr = vec![1, 2, 3, 4, 5];

        let bytes = vdl.write().unwrap();
        let vdl: PktDropLog = PktDropLog::read(bytes).unwrap();

        assert_eq!(vdl.rid, 1);
        assert_eq!(vdl.core, 2);
        assert_eq!(vdl.log_idx, 3);
        assert_eq!(vdl.max_num_cores, 4);
        assert_eq!(vdl.pkt_droplog_max_bufsz, 5);
        assert_eq!(vdl.pkt_droplog_en, 6);
        assert_eq!(vdl.pkt_droplog_sysctl_en, 7);
        assert_eq!(vdl.pkt_droplog_arr, vec![1, 2, 3, 4, 5]);
    }
}
