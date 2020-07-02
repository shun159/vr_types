// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::sandesh::SandeshOp;
use crate::utils;
use crate::vr_types::VrSandesh;
use crate::vr_types_binding::vr_hugepage_config;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct HugepageConfig {
    pub op: SandeshOp,
    pub mem: Vec<u64>,
    pub msize: Vec<u32>,
    pub resp: u32,
}

impl Default for HugepageConfig {
    fn default() -> HugepageConfig {
        HugepageConfig {
            op: SandeshOp::Add,
            mem: vec![],
            msize: vec![],
            resp: 0,
        }
    }
}

impl HugepageConfig {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_hugepage_config = vr_hugepage_config::new();
        encoder.vhp_op = self.op as u32;
        encoder.vhp_mem = utils::into_mut_ptr(&self.mem);
        encoder.vhp_mem_size = self.mem.len() as u32;
        encoder.vhp_msize = utils::into_mut_ptr(&self.msize);
        encoder.vhp_msize_size = self.msize.len() as u32;
        encoder.vhp_resp = self.resp;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<HugepageConfig, &'a str> {
        let decoder: vr_hugepage_config = vr_hugepage_config::new();
        match decoder.read(buf) {
            Err(_) => Err("Failed to write binary"),
            Ok(_) => {
                let mut vhp: HugepageConfig = HugepageConfig::default();
                vhp.op = decoder.vhp_op.try_into().unwrap();
                vhp.mem = utils::free_buf(
                    decoder.vhp_mem,
                    decoder.vhp_mem_size as usize,
                );
                vhp.msize = utils::free_buf(
                    decoder.vhp_msize,
                    decoder.vhp_mem_size as usize,
                );
                vhp.resp = decoder.vhp_resp;
                Ok(vhp)
            }
        }
    }
}

#[cfg(test)]
mod test_vr_vxlan {
    use crate::sandesh::SandeshOp;
    use crate::vr_hugepage_config::HugepageConfig;

    #[test]
    fn empty_request() {
        let vhp: HugepageConfig = HugepageConfig::default();
        let bytes = vhp.write().unwrap();
        let vhp: HugepageConfig = HugepageConfig::read(bytes).unwrap();
        assert_eq!(vhp.op, SandeshOp::Add);
        assert_eq!(vhp.mem, vec![]);
        assert_eq!(vhp.msize, vec![]);
        assert_eq!(vhp.resp, 0);
    }

    #[test]
    fn complex_request() {
        let mut vhp: HugepageConfig = HugepageConfig::default();
        vhp.op = SandeshOp::Dump;
        vhp.mem = vec![1, 2, 3, 4, 5];
        vhp.msize = vec![1, 2, 3, 4, 5];
        vhp.resp = 3;

        let bytes = vhp.write().unwrap();
        let vhp: HugepageConfig = HugepageConfig::read(bytes).unwrap();

        assert_eq!(vhp.op, SandeshOp::Dump);
        assert_eq!(vhp.mem, vec![1, 2, 3, 4, 5]);
        assert_eq!(vhp.msize, vec![1, 2, 3, 4, 5]);
        assert_eq!(vhp.resp, 3);
    }
}
