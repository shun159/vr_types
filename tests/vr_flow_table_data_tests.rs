// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_flow_table_data {
    use vr_type::vr_messages::vr_flow::FlowOp;
    use vr_type::vr_messages::vr_flow_table_data::FlowTableData;

    #[test]
    fn empty_request() {
        let ftable: FlowTableData = FlowTableData::default();
        let bytes = ftable.write().unwrap();
        let ftable: FlowTableData = FlowTableData::read(bytes).unwrap();
        assert_eq!(ftable.op, FlowOp::Set);
        assert_eq!(ftable.rid, 0);
        assert_eq!(ftable.size, 0);
        assert_eq!(ftable.dev, 0);
        assert_eq!(ftable.file_path, "".to_string());
        assert_eq!(ftable.used_entries, 0);
        assert_eq!(ftable.processed, 0);
        assert_eq!(ftable.deleted, 0);
        assert_eq!(ftable.added, 0);
        assert_eq!(ftable.created, 0);
        assert_eq!(ftable.changed, 0);
        assert_eq!(ftable.hold_oflows, 0);
        assert_eq!(ftable.cpus, 0);
        assert_eq!(ftable.oflow_entries, 0);
        assert_eq!(ftable.hold_stat, vec![]);
        assert_eq!(ftable.burst_free_tokens, 0);
        assert_eq!(ftable.hold_entries, 0);
    }

    #[test]
    fn complex_request() {
        let mut ftable: FlowTableData = FlowTableData::default();

        ftable.op = FlowOp::List;
        ftable.rid = 1;
        ftable.size = 2;
        ftable.dev = 3;
        ftable.file_path = "/path/to/flow_table_data".to_string();
        ftable.used_entries = 5;
        ftable.processed = 6;
        ftable.deleted = 7;
        ftable.added = 8;
        ftable.created = 9;
        ftable.changed = 10;
        ftable.hold_oflows = 11;
        ftable.cpus = 12;
        ftable.oflow_entries = 13;
        ftable.hold_stat = vec![1, 2, 3, 4, 5];
        ftable.burst_free_tokens = 15;
        ftable.hold_entries = 16;

        let bytes = ftable.write().unwrap();
        let ftable: FlowTableData = FlowTableData::read(bytes).unwrap();

        assert_eq!(ftable.op, FlowOp::List);
        assert_eq!(ftable.rid, 1);
        assert_eq!(ftable.size, 2);
        assert_eq!(ftable.dev, 3);
        assert_eq!(ftable.file_path, "/path/to/flow_table_data".to_string());
        assert_eq!(ftable.used_entries, 5);
        assert_eq!(ftable.processed, 6);
        assert_eq!(ftable.deleted, 7);
        assert_eq!(ftable.added, 8);
        assert_eq!(ftable.created, 9);
        assert_eq!(ftable.changed, 10);
        assert_eq!(ftable.hold_oflows, 11);
        assert_eq!(ftable.cpus, 12);
        assert_eq!(ftable.oflow_entries, 13);
        assert_eq!(ftable.hold_stat, vec![1, 2, 3, 4, 5]);
        assert_eq!(ftable.burst_free_tokens, 15);
        assert_eq!(ftable.hold_entries, 16);
    }
}
