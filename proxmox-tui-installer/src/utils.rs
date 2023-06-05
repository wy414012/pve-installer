use std::{
    fmt,
    net::{AddrParseError, IpAddr},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
pub enum CidrAddressParseError {
    NoDelimiter,
    InvalidAddr(AddrParseError),
    InvalidMask(Option<ParseIntError>),
}

#[derive(Clone, Debug)]
pub struct CidrAddress {
    addr: IpAddr,
    mask: usize,
}

impl CidrAddress {
    pub fn new(addr: IpAddr, mask: usize) -> Result<Self, CidrAddressParseError> {
        if mask > 32 {
            Err(CidrAddressParseError::InvalidMask(None))
        } else {
            Ok(Self { addr, mask })
        }
    }

    pub fn addr(&self) -> IpAddr {
        self.addr
    }

    pub fn mask(&self) -> usize {
        self.mask
    }
}

impl FromStr for CidrAddress {
    type Err = CidrAddressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (addr, mask) = s
            .split_once('/')
            .ok_or(CidrAddressParseError::NoDelimiter)?;

        let mask = mask
            .parse()
            .map_err(|err| CidrAddressParseError::InvalidMask(Some(err)))?;

        if mask > 32 {
            Err(CidrAddressParseError::InvalidMask(None))
        } else {
            Ok(Self {
                addr: addr.parse().map_err(CidrAddressParseError::InvalidAddr)?,
                mask,
            })
        }
    }
}

impl fmt::Display for CidrAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.mask)
    }
}
