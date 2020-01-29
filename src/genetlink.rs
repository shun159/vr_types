//extern crate neli;

//use neli::consts::{CtrlAttr, CtrlCmd, GenlId, NlFamily, NlmF, Nlmsg};
use neli::consts::NlFamily;
//use neli::genl::Genlmsghdr;
use neli::socket::NlSocket;
use std::io::{Error, ErrorKind};

const GROUP_ID: u32 = 0x4;
//const GENL_ID:  i32 = (libc::NLMSG_MIN_TYPE + 0x10);
const FAMILY_NAME: &str = "vrouter";
const MCAST_GROUP: &str = "VRouterGroup";

pub struct Genetlink {
    pub socket:  NlSocket,
    pub mcgroup: u32,
    pub family:  u16
}

impl Genetlink {
    pub fn connect() -> Result<Genetlink, Error> {
        match Self::nl_connect() {
            Result::Ok(mut s)  => {
                s.set_mcast_groups(vec![GROUP_ID]).unwrap();
                match Self::resolve_group_and_family(&mut s) {
                    Result::Ok((m, f)) => {
                        let genl = Genetlink{socket: s, mcgroup: m, family: f};
                        return Result::Ok(genl)
                    },

                    Result::Err(e) => {
                        return Result::Err(e)
                    }
                }
            },

            Result::Err(e) =>
                return Result::Err(e)
        }
    }

    fn resolve_group_and_family(s: &mut NlSocket) -> Result<(u32, u16), Error> {
        let mcgroup = (*s).resolve_nl_mcast_group(FAMILY_NAME, MCAST_GROUP);
        let family  = (*s).resolve_genl_family(FAMILY_NAME);
        match (mcgroup, family) {
            (Result::Ok(m), Result::Ok(f)) =>
                return Result::Ok((m, f)),

            (_, _) => {
                let msg = "Failed to resolve mcast group or family";
                return Result::Err(Error::new(ErrorKind::Other, msg))
            }
        }
    }

    fn nl_connect() -> Result<NlSocket, Error> {
        return NlSocket::connect(
            NlFamily::Generic,    // proto
            None,                 // pid
            Some(vec![GROUP_ID]), // groups
            true                  // track_seq
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect() {
        //let nl = Genetlink::connect().unwrap();
        //assert_eq!(nl.mcgroup, 19);
        //assert_eq!(nl.family,  37);
    }
}
