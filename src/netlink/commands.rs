// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use neli::consts::Cmd;
use neli::{impl_var, impl_var_base, impl_var_trait};

pub const SANDESH_REQUEST: u8 = 1;

impl_var_trait!(
    SandeshCommand, u8, Cmd,
    Request => SANDESH_REQUEST
);
