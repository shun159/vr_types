// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_hugepage_config {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_hugepage_config::HugepageConfig;

    #[test]
    fn empty_request() {
        let vhp: HugepageConfig = HugepageConfig::default();
        let bytes = vhp.write().unwrap();
        let vhp: HugepageConfig = HugepageConfig::read(bytes).unwrap();
        assert_eq!(vhp.op, SandeshOp::Add);
        assert_eq!(vhp.mem, vec![]);
        assert_eq!(vhp.psize, vec![]);
        assert_eq!(vhp.resp, 0);
    }

    #[test]
    fn complex_request() {
        let mut vhp: HugepageConfig = HugepageConfig::default();
        vhp.op = SandeshOp::Dump;
        vhp.mem = vec![1, 2, 3, 4, 5];
        vhp.psize = vec![1, 2, 3, 4, 5];
        vhp.resp = 3;

        let bytes = vhp.write().unwrap();
        let vhp: HugepageConfig = HugepageConfig::read(bytes).unwrap();

        assert_eq!(vhp.op, SandeshOp::Dump);
        assert_eq!(vhp.mem, vec![1, 2, 3, 4, 5]);
        assert_eq!(vhp.psize, vec![1, 2, 3, 4, 5]);
        assert_eq!(vhp.resp, 3);
    }
}
