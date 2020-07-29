use crate::genetlink::send_sandesh_msg;
use crate::vr_messages::*;
use eui48::MacAddress;

pub fn test_p() {
    //-> Result<GenericNetlinkMessage<&[u8]>, FamilyError> {
    let mut req = InterfaceRequest::default();
    req.op = SandeshOp::Get;
    req.name = "veth0".to_string();
    req.os_idx = 19;
    req._type = IfType::Virtual;
    req.transport = 1; // VIF_TRANSPORT_ETH
    req.mac =
        MacAddress::from_bytes(&[0xf6, 0xec, 0xca, 0xad, 0x67, 0x91]).unwrap();
    let res = send_sandesh_msg(Message::InterfaceRequest(req));
    println!("{:#?}", res);
}

#[cfg(test)]
mod test_test_p {
    use crate::genetlink::family::test_p;

    #[test]
    fn test() {
        test_p();
    }
}
