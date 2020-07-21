// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use super::sandesh::SandeshOp;
use super::vr_types::VrSandesh;
use super::vr_types_binding::vr_hugepage_config;
use crate::utils;
use std::convert::TryInto;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct HugepageConfig {
    pub op: SandeshOp,
    pub read_length: usize,
    pub mem: Vec<u64>,
    pub psize: Vec<u32>,
    pub resp: u32,
    pub mem_size: Vec<u32>,
    pub file_paths: Vec<i8>,
    pub file_path_size: Vec<u32>
}

impl HugepageConfig {
    pub fn write(&self) -> Result<Vec<u8>, &str> {
        let mut encoder: vr_hugepage_config = vr_hugepage_config::new();
        encoder.vhp_op = self.op as u32;
        encoder.vhp_mem = utils::into_mut_ptr(&self.mem);
        encoder.vhp_mem_size = self.mem.len() as u32;
        encoder.vhp_psize = utils::into_mut_ptr(&self.psize);
        encoder.vhp_psize_size = self.psize.len() as u32;
        encoder.vhp_resp = self.resp;
        encoder.vhp_mem_sz = utils::into_mut_ptr(&self.mem_size);
        encoder.vhp_mem_sz_size = self.mem_size.len() as u32;
        encoder.vhp_file_paths = utils::into_mut_ptr(&self.file_paths);
        encoder.vhp_file_paths_size = self.file_paths.len() as u32;
        encoder.vhp_file_path_sz = utils::into_mut_ptr(&self.file_path_size);
        encoder.vhp_file_path_sz_size = self.file_path_size.len() as u32;
        match encoder.write() {
            Err(_) => Err("Failed to write binary"),
            Ok(v) => Ok(v),
        }
    }

    pub fn read<'a>(buf: Vec<u8>) -> Result<HugepageConfig, &'a str> {
        let decoder: vr_hugepage_config = vr_hugepage_config::new();
        match decoder.read(&buf) {
            Err(_) => Err("Failed to write binary"),
            Ok(rxfer) => {
                let mut vhp: HugepageConfig = HugepageConfig::default();
                vhp.read_length = rxfer as usize;
                vhp.op = decoder.vhp_op.try_into().unwrap();
                vhp.mem = utils::free_buf(
                    decoder.vhp_mem,
                    decoder.vhp_mem_size as usize,
                );
                vhp.psize = utils::free_buf(
                    decoder.vhp_psize,
                    decoder.vhp_psize_size as usize,
                );
                vhp.mem_size = utils::free_buf(
                    decoder.vhp_mem_sz,
                    decoder.vhp_mem_sz_size as usize,
                );
                vhp.file_paths = utils::free_buf(
                    decoder.vhp_file_paths,
                    decoder.vhp_file_paths_size as usize,
                );
                vhp.file_path_size = utils::free_buf(
                    decoder.vhp_file_path_sz,
                    decoder.vhp_file_path_sz_size as usize,
                );
                vhp.resp = decoder.vhp_resp;
                Ok(vhp)
            }
        }
    }
}
