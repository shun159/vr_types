pub mod family;
pub mod message;
pub mod raw;

pub use self::message::GenericNetlinkMessage;
pub use self::raw::*;
pub use crate::netlink::raw::*;
use crate::netlink::NetlinkError;
use crate::netlink::{deserialize_attrs, deserialize_u16};
use crate::netlink::{NetlinkAttr, NetlinkMessage};
pub use crate::vr_messages::*;
use netlink_sys::Protocol::Generic;
use netlink_sys::Socket;
use std::ffi::CString;
use thiserror::Error;

pub const VROUTER_GENETLINK_FAMILY_NAME: &str = "vrouter";
pub const NL_ATTR_VR_MESSAGE_PROTOCOL: u16 = 1;
pub const SANDESH_REQUEST: u8 = 1;
pub const NETLINK_VERSION: u8 = 2;

pub fn resolve_family_id(name: &str) -> Result<u16, NetlinkError> {
    let cstr_name = &CString::new(name).unwrap();
    let nl_attr =
        &[NetlinkAttr::new(CTRL_ATTR_FAMILY_NAME, cstr_name)] as &[_];
    let nl_msg = NetlinkMessage::new(
        GENL_ID_CTRL,
        NLM_F_REQUEST,
        GenericNetlinkMessage::new(
            CTRL_CMD_GETFAMILY,
            NETLINK_VERSION,
            nl_attr,
        ),
    );
    let socket = &Socket::new(Generic).unwrap();
    nl_msg.send_nl(socket);
    let nl_msg = NetlinkMessage::<Vec<u8>>::recv_nl(socket);
    for attr in deserialize_attrs(&nl_msg.payload.payload[..]) {
        let (ty, value) = attr?;
        if ty == CTRL_ATTR_FAMILY_ID {
            return Ok(deserialize_u16(value)?);
        }
    }

    Ok(0)
}

pub fn send_sandesh_msg(
    payload: Message,
) -> Result<Vec<Message>, MessageHandleError> {
    let nl_attr =
        &[NetlinkAttr::new(NL_ATTR_VR_MESSAGE_PROTOCOL, payload)] as &[_];
    let nl_msg = NetlinkMessage::new(
        resolve_family_id(VROUTER_GENETLINK_FAMILY_NAME).unwrap(),
        NLM_F_REQUEST,
        GenericNetlinkMessage::new(SANDESH_REQUEST, NETLINK_VERSION, nl_attr),
    );
    let socket = &Socket::new(Generic).unwrap();
    nl_msg.send_nl(socket);
    let nl_msg = NetlinkMessage::<Vec<u8>>::recv_nl(socket);
    for attr in deserialize_attrs(&nl_msg.payload.payload[..]) {
        let (ty, value) = attr.unwrap();
        if ty == NL_ATTR_VR_MESSAGE_PROTOCOL {
            return handle_sandesh_reply(value.to_vec());
        }
    }
    return Ok(vec![]);
}

// private functions

fn handle_sandesh_reply(
    buf: Vec<u8>,
) -> Result<Vec<Message>, MessageHandleError> {
    let vec: &mut Vec<Message> = &mut Vec::new();
    let resp = decode_header_message(&buf)?;
    for message in decode_messages(resp.read_length(), &buf[..]) {
        vec.push(message)
    }
    Ok(vec.to_vec())
}

fn decode_header_message(buf: &Vec<u8>) -> Result<Message, MessageHandleError> {
    match Message::from_bytes(buf.to_vec())? {
        Message::VrResponse(resp) if resp.code == 0 => {
            Ok(Message::VrResponse(resp))
        }
        Message::VrResponse(resp) if resp.code.abs() == libc::ENODEV => {
            Err(MessageHandleError::RequestError(OperationError::ENODEV))
        }
        Message::VrResponse(resp) if resp.code.abs() == libc::ENOMEM => {
            Err(MessageHandleError::RequestError(OperationError::ENOMEM))
        }
        Message::VrResponse(resp) if resp.code.abs() == libc::ENOENT => {
            Err(MessageHandleError::RequestError(OperationError::ENOENT))
        }
        Message::VrResponse(resp) if resp.code.abs() == libc::EINVAL => {
            Err(MessageHandleError::RequestError(OperationError::EINVAL))
        }
        Message::VrResponse(resp) if resp.code < 0 => {
            Err(MessageHandleError::RequestError(OperationError::UNKNOWN(
                resp.code,
            )))
        }
        _ => Err(MessageHandleError::MessageOutOfOrder),
    }
}

fn decode_messages(pos: usize, buf: &[u8]) -> VrMessageIter<'_> {
    VrMessageIter {
        prev_read_pos: pos,
        buf: buf,
    }
}

struct VrMessageIter<'a> {
    prev_read_pos: usize,
    buf: &'a [u8],
}

impl<'a> Iterator for VrMessageIter<'a> {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.to_vec().len() < self.prev_read_pos {
            return None;
        }
        let (_prev, leftover) = self.buf.split_at(self.prev_read_pos);
        match Message::from_bytes(leftover.to_vec()) {
            Err(_err) => None,
            Ok(req) => {
                self.prev_read_pos = req.read_length();
                self.buf = leftover;
                Some(req)
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum MessageHandleError {
    #[error("Invalid Sandesh message received")]
    InvalidBuffer(#[from] CodecError),
    #[error("The request has failed")]
    RequestError(#[from] OperationError),
    #[error("The Netlink operation has failed")]
    NetlinkError(#[from] NetlinkError),
    #[error("Expected that a vr_message comes first")]
    MessageOutOfOrder,
}
