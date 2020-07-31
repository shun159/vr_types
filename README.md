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
