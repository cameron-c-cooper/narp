use std::{net::IpAddr, str::FromStr};

struct Target<'a> {
    addr: IpAddr,
    addr_type: (bool, bool),
    os: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
struct AddrErr;

impl FromStr for Target<'_> {
    type Err = AddrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(addr) = IpAddr::from_str(s) {
            let addr_type = (addr.is_ipv4(), addr.is_ipv6());
            return Ok(Target {
                addr,
                addr_type,
                os: None,
            });
        } else {
            Err(AddrErr)
        }
    }
}
