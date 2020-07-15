// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_bridge_table_data {
    use vr_type::vr_messages::sandesh::SandeshOp;
    use vr_type::vr_messages::vr_bridge_table_data::BridgeTableData;

    #[test]
    fn empty_request() {
        let btable: BridgeTableData = BridgeTableData::default();
        let bytes = btable.write().unwrap();
        let btable: BridgeTableData = BridgeTableData::read(bytes).unwrap();
        assert_eq!(btable.op, SandeshOp::Add);
        assert_eq!(btable.rid, 0);
        assert_eq!(btable.size, 0);
        assert_eq!(btable.dev, 0);
        assert_eq!(btable.file_path, "".to_string());
    }

    #[test]
    fn complex_request() {
        let mut btable: BridgeTableData = BridgeTableData::default();

        btable.op = SandeshOp::Dump;
        btable.rid = 1;
        btable.size = 2;
        btable.dev = 3;
        btable.file_path = "/path/to/bridge_table_data".to_string();

        let bytes = btable.write().unwrap();
        let btable: BridgeTableData = BridgeTableData::read(bytes).unwrap();

        assert_eq!(btable.op, SandeshOp::Dump);
        assert_eq!(btable.rid, 1);
        assert_eq!(btable.size, 2);
        assert_eq!(btable.dev, 3);
        assert_eq!(btable.file_path, "/path/to/bridge_table_data".to_string());
    }
}
