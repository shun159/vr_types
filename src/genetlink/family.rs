use super::raw::{
    CTRL_ATTR_FAMILY_ID, CTRL_ATTR_FAMILY_NAME, CTRL_CMD_GETFAMILY,
    GENL_ID_CTRL,
};
use super::GenericNetlinkMessage;
use crate::netlink::raw::{NLM_F_EXCL, NLM_F_REQUEST};
use crate::netlink::{
    deserialize_attrs, deserialize_u16, NetlinkAttr, NetlinkMessage, Serialize,
};
use crate::vr_messages::*;
use eui48::MacAddress;
use netlink_sys::{Socket, SocketAddr};
use std::ffi::CString;
use std::io;
use thiserror::Error;

const BUFFER_SIZE: usize = 512;

pub fn test_p(socket: &Socket) {
    //-> Result<GenericNetlinkMessage<&[u8]>, FamilyError> {
    let mut addr = SocketAddr::new(0, 0);
    let mut req = InterfaceRequest::default();
    req.op = SandeshOp::Get;
    req.name = "veth0".to_string();
    req.os_idx = 50;
    req._type = IfType::Virtual;
    req.transport = 1; // VIF_TRANSPORT_ETH
    req.mac =
        MacAddress::from_bytes(&[0xae, 0x07, 0xbe, 0x70, 0x12, 0x2a]).unwrap();
    socket.get_address(&mut addr).unwrap();

    let msg = NetlinkMessage {
        ty: resolve_family_id(socket, "vrouter").unwrap().unwrap(),
        flags: NLM_F_REQUEST | NLM_F_EXCL,
        seq: 1,
        pid: std::process::id(),
        payload: GenericNetlinkMessage {
            cmd: 1,
            version: 0,
            payload: &[NetlinkAttr {
                ty: 1,
                payload: Message::InterfaceRequest(req),
            }] as &[_],
        },
    };
    let mut buffer = [0; 10000];
    let msg_len = msg.len() as usize;
    msg.serialize(&mut buffer[..msg_len]);
    socket.send(&buffer[..msg_len], 0).unwrap();

    let mut buffer = [0; 10000];
    let reply_len = socket.recv(&mut buffer, 0).unwrap();
    let nl_msg = NetlinkMessage::deserialize(&buffer[..reply_len]);
    let genl_msg = GenericNetlinkMessage::deserialize(nl_msg.payload).unwrap();
    for attr in deserialize_attrs(genl_msg.payload) {
        let (ty, value) = attr.unwrap();
        if ty == 1 {
            match Message::from_bytes(value.to_vec()) {
                Ok(Message::VrResponse(resp))
                    if resp.op == SandeshOp::Response =>
                {
                    println!("ok resp {:#?}", resp);
                    return;
                }
                resp => {
                    println!("err {:?}", resp);
                    return;
                }
            };
        }
    }
}

pub fn resolve_family_id(
    socket: &Socket,
    name: &str,
) -> Result<Option<u16>, FamilyError> {
    let mut addr = SocketAddr::new(0, 0);
    socket.get_address(&mut addr)?;

    let msg = NetlinkMessage {
        ty: GENL_ID_CTRL,
        flags: NLM_F_REQUEST,
        seq: 1,
        pid: addr.port_number(),
        payload: GenericNetlinkMessage {
            cmd: CTRL_CMD_GETFAMILY,
            version: 2,
            payload: &[NetlinkAttr {
                ty: CTRL_ATTR_FAMILY_NAME,
                payload: &CString::new(name).unwrap(),
            }] as &[_],
        },
    };
    let mut buffer = [0; BUFFER_SIZE];
    let msg_len = msg.len() as usize;
    msg.serialize(&mut buffer[..msg_len]);
    socket.send(&buffer[..msg_len], 0)?;

    let reply_len = socket.recv(&mut buffer, 0)?;
    let nl_msg = NetlinkMessage::deserialize(&buffer[..reply_len]);
    let genl_msg = GenericNetlinkMessage::deserialize(nl_msg.payload)?;
    for attr in deserialize_attrs(genl_msg.payload) {
        let (ty, value) = attr?;
        if ty == CTRL_ATTR_FAMILY_ID {
            return Ok(Some(deserialize_u16(value)?));
        }
    }
    Ok(None)
}

#[derive(Debug, Error)]
pub enum FamilyError {
    #[error("io")]
    Io(#[from] io::Error),
    #[error("invalid netlink message")]
    Netlink(#[from] crate::netlink::InvalidBuffer),
    #[error("invalid generic netlink message")]
    GenericNetlink(#[from] super::message::InvalidBuffer),
}

#[cfg(test)]
mod test_test {
    use crate::genetlink::family::test_p;
    use netlink_sys::Protocol::Generic;
    use netlink_sys::Socket;

    #[test]
    fn test() {
        let socket = Socket::new(Generic).unwrap();
        let res = test_p(&socket);
        println!("{:?}", res);
    }
}
