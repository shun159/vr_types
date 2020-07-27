pub mod family;
pub mod message;
pub mod raw;

pub use self::message::GenericNetlinkMessage;
pub use self::raw::*;
pub use crate::netlink::raw::*;
use crate::netlink::NetlinkReadError;
use crate::netlink::{deserialize_attrs, deserialize_u16};
use crate::netlink::{NetlinkAttr, NetlinkMessage};
pub use crate::vr_messages::*;
use netlink_sys::Protocol::Generic;
use netlink_sys::Socket;
use std::ffi::CString;

pub const VROUTER_GENETLINK_FAMILY_NAME: &str = "vrouter";
pub const NL_ATTR_VR_MESSAGE_PROTOCOL: u16 = 1;
pub const SANDESH_REQUEST: u8 = 1;
pub const NETLINK_VERSION: u8 = 2;

pub fn resolve_family_id(name: &str) -> Result<u16, NetlinkReadError> {
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

//pub fn send_sandesh_msg(
//    payload: Message,
//) -> Result<Option<Message>, NetlinkReadError> {
//    let nl_attr =
//        &[NetlinkAttr::new(NL_ATTR_VR_MESSAGE_PROTOCOL, payload)] as &[_];
//    let nl_msg = NetlinkMessage::new(
//        resolve_family_id(VROUTER_GENETLINK_FAMILY_NAME).unwrap(),
//        NLM_F_REQUEST,
//        GenericNetlinkMessage::new(SANDESH_REQUEST, NETLINK_VERSION, nl_attr),
//    );
//    let socket = &Socket::new(Generic).unwrap();
//    nl_msg.send_nl(socket);
//    let nl_msg = NetlinkMessage::<Vec<u8>>::recv_nl(socket);
//    for attr in deserialize_attrs(&nl_msg.payload.payload[..]) {
//        let (ty, value) = attr?;
//        if ty == NL_ATTR_VR_MESSAGE_PROTOCOL {
//            match Message::from_bytes(value.to_vec()) {
//                Ok()
//            }
//        }
//    }
//
//    Ok(None)
//}
