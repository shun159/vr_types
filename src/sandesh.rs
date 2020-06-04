// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Op {
    Add,
    Get,
    Del,
    Dump,
    Response,
    Reset,
}
