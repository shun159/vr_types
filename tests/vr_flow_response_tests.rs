// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod test_vr_vxlan {
    use vr_type::vr_messages::vr_flow::FlowOp;
    use vr_type::vr_messages::vr_flow_response::FlowResponse;

    #[test]
    fn empty_request() {
        let fresp: FlowResponse = FlowResponse::default();
        let bytes = fresp.write().unwrap();
        let fresp: FlowResponse = FlowResponse::read(bytes).unwrap();
        assert_eq!(fresp.op, FlowOp::Set);
        assert_eq!(fresp.rid, 0);
        assert_eq!(fresp.flags, 0);
        assert_eq!(fresp.index, 0);
        assert_eq!(fresp.bytes, 0);
        assert_eq!(fresp.packets, 0);
        assert_eq!(fresp.stats_oflow, 0);
        assert_eq!(fresp.gen_id, 0);
    }

    #[test]
    fn complex_request() {
        let mut fresp: FlowResponse = FlowResponse::default();

        fresp.op = FlowOp::List;
        fresp.rid = 1;
        fresp.flags = 2;
        fresp.index = 3;
        fresp.bytes = 4;
        fresp.packets = 5;
        fresp.stats_oflow = 6;
        fresp.gen_id = 7;

        let bytes = fresp.write().unwrap();
        let fresp: FlowResponse = FlowResponse::read(bytes).unwrap();

        assert_eq!(fresp.op, FlowOp::List);
        assert_eq!(fresp.rid, 1);
        assert_eq!(fresp.flags, 2);
        assert_eq!(fresp.index, 3);
        assert_eq!(fresp.bytes, 4);
        assert_eq!(fresp.packets, 5);
        assert_eq!(fresp.stats_oflow, 6);
        assert_eq!(fresp.gen_id, 7);
    }
}
