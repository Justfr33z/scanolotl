use async_trait::async_trait;
use crate::scanner::ScanError;
use std::{
    slice::Iter,
    str::FromStr,
};
use async_std::{
    io,
    net::{
        Ipv4Addr,
        TcpStream,
        SocketAddrV4,
    },
};

#[async_trait]
pub trait Connect {
    async fn connect(&self, port: u16) -> io::Result<TcpStream>;
}

#[async_trait]
impl Connect for Ipv4Addr {
    async fn connect(&self, port: u16) -> io::Result<TcpStream> {
        TcpStream::connect(SocketAddrV4::new(*self, port)).await
    }
}

pub trait UInt {
    fn to_u32(&self) -> u32;
}

impl UInt for Ipv4Addr {
    fn to_u32(&self) -> u32 {
        let mut ip = 0;

        for oct in &self.octets() {
            ip = (ip << 8) + *oct as u32;
        }

        ip
    }
}

pub struct Ipv4Iter<'a> {
    iter: Iter<'a, Ipv4Addr>,
}

impl<'a> Ipv4Iter<'a> {
    pub fn new(ips: &'a [Ipv4Addr]) -> Self {
        Self {
            iter: ips.iter()
        }
    }
}

impl<'a> Iterator for Ipv4Iter<'a> {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(ip) => Some(*ip),
        }
    }
}

pub struct Ipv4Subnet {
    pub ip: Ipv4Addr,
    pub mask: Ipv4Addr,
}

impl Ipv4Subnet {
    pub fn network(&self) -> Ipv4Addr {
        Ipv4Addr::from(self.ip.to_u32() & self.mask.to_u32())
    }

    pub fn hosts(&self) -> Vec<Ipv4Addr> {
        (self.network().to_u32() + 1..self.broadcast().to_u32())
            .map(|r| Ipv4Addr::from(r))
            .collect::<Vec<Ipv4Addr>>()
    }

    pub fn broadcast(&self) -> Ipv4Addr {
        Ipv4Addr::from(self.network().to_u32() + !self.mask.to_u32())
    }
}

// TODO: Make better yes yes :)
impl FromStr for Ipv4Subnet {
    type Err = ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("/");

        let ip = match split.next() {
            None => return Err(ScanError::Error("Failed to parse string because ip part is missing".to_owned())),
            Some(ip) => Ipv4Addr::from_str(ip)?,
        };

        let mask = match split.next() {
            None => return Err(ScanError::Error("Failed to parse string because cidr part is missing".to_owned())),
            Some(cidr) => {
                let cidr = cidr.parse::<u8>()?;
                let mut bin = String::new();

                for i in 0..32 {
                    if cidr > i {
                        bin += "1";
                    } else {
                        bin += "0";
                    }
                }

                let mut octets = [0, 0, 0, 0];
                for i in 0..octets.len() {
                    octets[i] = u8::from_str_radix(&bin[i * 8..i * 8 + 8], 2)?;
                }

                Ipv4Addr::from(octets)
            }
        };

        Ok(Self {
            ip,
            mask
        })
    }
}