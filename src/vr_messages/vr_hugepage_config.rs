// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::utils;
use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_hugepage_config;
use std::convert::TryInto;

#[derive(Debug, Clone, Eq, PartialEq)]
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
