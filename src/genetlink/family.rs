use super::GenericNetlinkMessage;
use crate::netlink::raw::{NLM_F_EXCL, NLM_F_REQUEST};
use crate::netlink::{
    deserialize_attrs, NetlinkAttr, NetlinkMessage, Serialize,
};
use crate::vr_messages::*;
use eui48::MacAddress;
use netlink_sys::{Socket, SocketAddr};

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
        ty: 37,
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
