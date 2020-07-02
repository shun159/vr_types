// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::utils;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_qos_map_req;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct QosMapRequest {
    pub op: SandeshOp,
    pub rid: u16,
    pub id: u16,
    pub dscp: Vec<i8>,
    pub dscp_fc_id: Vec<i8>,
    pub mpls_qos: Vec<i8>,
    pub mpls_qos_fc_id: Vec<i8>,
    pub dotonep: Vec<i8>,
    pub dotonep_fc_id: Vec<i8>,
    pub marker: i16,
}

impl Default for QosMapRequest {
    fn default() -> QosMapRequest {
        QosMapRequest {
            op: SandeshOp::Add,
            rid: 0,
            id: 0,
            dscp: vec![],
            dscp_fc_id: vec![],
            mpls_qos: vec![],
            mpls_qos_fc_id: vec![],
            dotonep: vec![],
            dotonep_fc_id: vec![],
            marker: 0,
        }
    }
}

impl QosMapRequest {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_qos_map_req = vr_qos_map_req::new();
        encoder.h_op = self.op as u32;
        encoder.qmr_rid = self.rid;
        encoder.qmr_id = self.id;
        encoder.qmr_dscp = utils::into_mut_ptr(&self.dscp);
        encoder.qmr_dscp_size = self.dscp.len() as u32;
        encoder.qmr_dscp_fc_id = utils::into_mut_ptr(&self.dscp_fc_id);
        encoder.qmr_dscp_fc_id_size = self.dscp_fc_id.len() as u32;
        encoder.qmr_mpls_qos = utils::into_mut_ptr(&self.mpls_qos);
        encoder.qmr_mpls_qos_size = self.mpls_qos.len() as u32;
        encoder.qmr_mpls_qos_fc_id = utils::into_mut_ptr(&self.mpls_qos_fc_id);
        encoder.qmr_mpls_qos_fc_id_size = self.mpls_qos_fc_id.len() as u32;
        encoder.qmr_dotonep = utils::into_mut_ptr(&self.dotonep);
        encoder.qmr_dotonep_size = self.dotonep.len() as u32;
        encoder.qmr_dotonep_fc_id = utils::into_mut_ptr(&self.dotonep_fc_id);
        encoder.qmr_dotonep_fc_id_size = self.dotonep_fc_id.len() as u32;
        encoder.qmr_marker = self.marker;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<QosMapRequest, &'a str> {
        let decoder: vr_qos_map_req = vr_qos_map_req::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to read binary"),
            Ok(_) => {
                let mut qmr: QosMapRequest = QosMapRequest::default();
                qmr.op = decoder.h_op.try_into().unwrap();
                qmr.rid = decoder.qmr_rid;
                qmr.dscp = utils::free_buf(
                    decoder.qmr_dscp,
                    decoder.qmr_dscp_size as usize,
                );
                qmr.dscp_fc_id = utils::free_buf(
                    decoder.qmr_dscp_fc_id,
                    decoder.qmr_dscp_fc_id_size as usize,
                );
                qmr.mpls_qos = utils::free_buf(
                    decoder.qmr_mpls_qos,
                    decoder.qmr_mpls_qos_size as usize,
                );
                qmr.mpls_qos_fc_id = utils::free_buf(
                    decoder.qmr_mpls_qos_fc_id,
                    decoder.qmr_mpls_qos_fc_id_size as usize,
                );
                qmr.dotonep = utils::free_buf(
                    decoder.qmr_dotonep,
                    decoder.qmr_dotonep_size as usize,
                );
                qmr.dotonep_fc_id = utils::free_buf(
                    decoder.qmr_dotonep_fc_id,
                    decoder.qmr_dotonep_fc_id_size as usize,
                );
                qmr.id = decoder.qmr_id;
                qmr.marker = decoder.qmr_marker;
                Ok(qmr)
            }
        }
    }
}

#[cfg(test)]
mod test_vr_qos_map {
    use crate::sandesh::SandeshOp;
    use crate::vr_qos_map::QosMapRequest;

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
