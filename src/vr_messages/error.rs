// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("Failed to write binary. return {0}")]
    Write(i32),
    #[error("Failed to read binary. return {0}")]
    Read(i32),
    #[error("Unknown message type.")]
    UnknownMessageType,
}

#[derive(Debug, Error)]
pub enum OperationError {
    #[error("No such device")]
    ENODEV,
    #[error("Out of memory")]
    ENOMEM,
    #[error("No such file or directory")]
    ENOENT,
    #[error("Invalid argument")]
    EINVAL,
    #[error("Device or resouce busy")]
    EBUSY,
    #[error("Resource exists")]
    EEXIST,
    #[error("No space left on device")]
    ENOSPC,
    #[error("Unknown Error. return: {0}")]
    UNKNOWN(i32),
}
