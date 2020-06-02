pub const NH_DEAD: i8 = 0;
pub const NH_RCV: i8 = 1;
pub const NH_ENCAP: i8 = 2;
pub const NH_TUNNEL: i8 = 3;
pub const NH_RESOLVE: i8 = 4;
pub const NH_DISCARD: i8 = 5;
pub const NH_COMPOSITE: i8 = 6;
pub const NH_VRF_TRANSLATE: i8 = 7;
pub const NH_L2_RCV: i8 = 8;
pub const NH_MAX: i8 = 9;

pub const NH_FLAG_VALID: u32 = 0x000001;
pub const NH_FLAG_POLICY_ENABLED: u32 = 0x000002;
/* 0x000004 is free */
pub const NH_FLAG_TUNNEL_GRE: u32 = 0x000008;
pub const NH_FLAG_TUNNEL_UDP: u32 = 0x000010;
/*
 * Mcast flag can be appended to any type of nexthop, either an Encap,
 * composite etc
 */
pub const NH_FLAG_MCAST: u32 = 0x000020;
pub const NH_FLAG_TUNNEL_UDP_MPLS: u32 = 0x000040;
pub const NH_FLAG_TUNNEL_VXLAN: u32 = 0x000080;
pub const NH_FLAG_RELAXED_POLICY: u32 = 0x000100;
pub const NH_FLAG_COMPOSITE_FABRIC: u32 = 0x000200;
pub const NH_FLAG_COMPOSITE_ECMP: u32 = 0x000400;
pub const NH_FLAG_COMPOSITE_LU_ECMP: u32 = 0x000800;
pub const NH_FLAG_COMPOSITE_EVPN: u32 = 0x001000;
pub const NH_FLAG_COMPOSITE_ENCAP: u32 = 0x002000;
pub const NH_FLAG_COMPOSITE_TOR: u32 = 0x004000;
pub const NH_FLAG_ROUTE_LOOKUP: u32 = 0x008000;
pub const NH_FLAG_UNKNOWN_UC_FLOOD: u32 = 0x010000;
pub const NH_FLAG_TUNNEL_SIP_COPY: u32 = 0x020000;
pub const NH_FLAG_FLOW_LOOKUP: u32 = 0x040000;
pub const NH_FLAG_TUNNEL_PBB: u32 = 0x080000;
pub const NH_FLAG_MAC_LEARN: u32 = 0x100000;
pub const NH_FLAG_ETREE_ROOT: u32 = 0x200000;
pub const NH_FLAG_INDIRECT: u32 = 0x400000;
pub const NH_FLAG_L2_CONTROL_DATA: u32 = 0x800000;
pub const NH_FLAG_CRYPT_TRAFFIC: u32 = 0x01000000;
pub const NH_FLAG_L3_VXLAN: u32 = 0x02000000;
pub const NH_FLAG_TUNNEL_MPLS_O_MPLS: u32 = 0x04000000;
pub const NH_FLAG_VALIDATE_MCAST_SRC: u32 = 0x08000000;
