vr_type
----

`vr_type` is a Rust wrapper of `tf-vrouter/sandesh/vr.sandesh`.  
This library provides functionalities for translate between Rust terms and the sandesh binary format and  
transport sandesh messages to vrouter through `NETLINK_GENERIC`.

Example:
----

1. Add a veth interface to the vrouter and put its result on stdout.

```rust
fn add_interface() {
    let mut req = InterfaceRequest::default();
    req.name = "veth0".to_string();
    req.os_idx = 19;
    req._type = IfType::Virtual;
    req.transport = 1; // VIF_TRANSPORT_ETH
    req.mac =
        MacAddress::from_bytes(&[0xf6, 0xec, 0xca, 0xad, 0x67, 0x91]).unwrap();
    let res = send_sandesh_msg(Message::InterfaceRequest(req));
    println!("{:#?}", res);
}
```
