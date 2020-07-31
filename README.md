Rust binding to vr.sandesh APIs
----

[![Build Status](https://img.shields.io/travis/shun159/vr_types.svg?style=flat-square)](https://travis-ci.org/shun159/vr_types)

The purpose of the vr_types project is to provide friendly bindings to various tf-vrouter APIs.  
The goal is to not provide a 100% unified interface, but to make easy to hack tf-vrouter on your Laptop by yourself.

## Supported messages

The following request messages are supported.

- vr\_bridge\_table\_data
- vr\_drop\_stats\_req
- vr\_fc\_map\_req
- vr\_flow\_response
- vr\_flow\_req
- vr\_flow\_table\_data
- vr\_hugepage\_config
- vr\_interface\_req
- vr\_mem\_stats\_req
- vr\_mirror\_req
- vr\_mpls\_req
- vr\_nexthop\_req
- vr\_pkt\_drop\_log\_req
- vr\_qos\_map\_req
- vr\_response
- vr\_route\_req
- vr\_vrf\_req
- vr\_vrf\_assign\_req
- vr\_vrf\_stats\_req
- vr\_vxlan\_req

## Example

1. Fetch vrouter description

```rust
use vr_type::vr_messages::*;

fn vrouter_ops_request() {
    let mut vrouter_ops_body: VrouterOps = VrouterOps::default();
    vrouter_ops_body.op = SandeshOp::Get;
    let vrouter_ops_req = Message::VrouterOps(vrouter_ops_body);
    let vrouter_ops_rep = vrouter_ops_req.send_nl().unwrap();
    if let Message::VrouterOps(vrouter) = &vrouter_ops_rep[0] {
        println!("vrouter_ops response: {:#?}", vrouter);
        // You can see...
        // vouter_ops response: VrouterOps {
        //    op: Add,
        //    read_length: 476,
        //    rid: 0,
        //    mpls_labels: 5120,
        //    nexthops: 524288,
        //    bridge_entries: 262144,
        //    overflow_flow_bridge_entries: 0
        //    ...
        //  }
    }
}
```

## Status

Still under development. use only for tests

## Copyright and License

(c) 2020, Eishun Kondoh.
See [LICENSE](./LICENSE) for details
