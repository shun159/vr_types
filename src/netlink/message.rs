use super::raw::{nlmsghdr, NLMSG_LENGTH};
use super::Serialize;
use crate::genetlink::GenericNetlinkMessage;
use netlink_sys::Socket;
use std::process;
use zerocopy::LayoutVerified;

#[derive(Debug)]
pub struct NetlinkMessage<P> {
    pub ty: u16,
    pub flags: u16,
    pub seq: u32,
    pub pid: u32,
    pub payload: P,
}

impl<P: Serialize> NetlinkMessage<P> {
    pub fn new(ty: u16, flags: u16, seq: u32, payload: P) -> NetlinkMessage<P> {
        NetlinkMessage {
            ty: ty,
            flags: flags,
            seq: seq,
            pid: process::id(),
            payload: payload,
        }
    }

    pub fn send_nl(&self, socket: &Socket) {
        let mut buffer = [0; 1000];
        let byte_size = self.len() as usize;
        self.serialize(&mut buffer[..byte_size]);
        socket.send(&buffer[..byte_size], 0).unwrap();
    }

    // Generic NETLINK message specfic shortcut fucntion
    pub fn recv_nl(
        socket: &Socket,
    ) -> NetlinkMessage<GenericNetlinkMessage<Vec<u8>>> {
        let mut buffer = [0; 1000];
        let reply_len = socket.recv(&mut buffer, 0).unwrap();
        let nl_msg = NetlinkMessage::deserialize(&buffer[..reply_len]);
        let genl_msg =
            GenericNetlinkMessage::deserialize(nl_msg.payload).unwrap();
        NetlinkMessage::new(
            nl_msg.ty,
            nl_msg.flags,
            nl_msg.pid,
            GenericNetlinkMessage::new(
                genl_msg.cmd,
                genl_msg.version,
                genl_msg.payload.to_vec(),
            ),
        )
    }
}

impl<P: Serialize> Serialize for NetlinkMessage<P> {
    fn len(&self) -> u32 {
        NLMSG_LENGTH(self.payload.len())
    }

    fn serialize(&self, buf: &mut [u8]) {
        let header_len = NLMSG_LENGTH(0) as usize;
        let (header, payload) = buf.split_at_mut(header_len);
        let mut header =
            LayoutVerified::<_, nlmsghdr>::new(header).expect("invalid buffer");
        header.nlmsg_len = self.len();
        header.nlmsg_type = self.ty;
        header.nlmsg_flags = self.flags;
        header.nlmsg_seq = self.seq;
        header.nlmsg_pid = self.pid;
        self.payload.serialize(payload);
    }
}

impl<'a> NetlinkMessage<&'a [u8]> {
    pub fn deserialize(buf: &'a [u8]) -> Self {
        let header_len = NLMSG_LENGTH(0) as usize;
        let (header, payload) = buf.split_at(header_len);
        let header =
            LayoutVerified::<_, nlmsghdr>::new(header).expect("invalid buffer");
        Self {
            ty: header.nlmsg_type,
            flags: header.nlmsg_flags,
            seq: header.nlmsg_seq,
            pid: header.nlmsg_pid,
            payload,
        }
    }
}
