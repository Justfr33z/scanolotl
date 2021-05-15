use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use futures::{
    StreamExt,
    stream::FuturesOrdered,
};
use async_std::{
    io,
    future,
    net::{
        Ipv4Addr,
        Shutdown,
    }
};
use crate::{
    net::{
        Connect,
        Ipv4Iter,
        PacketEncoding,
    },
    mc::{
        Server,
        Information,
        proto::{
            Serialize,
            types::VarInt,
            packets::{
                Empty,
                Response,
                Handshake,
            },
        },
    },
};

pub use self::error::ScanError;

mod error;

pub struct Scanner<'a> {
    ips: &'a [Ipv4Addr],
    batch_size: u16,
    timeout: Duration,
    bar: ProgressBar,
}

impl<'a> Scanner<'a> {
    pub fn new(ips: &'a [Ipv4Addr], batch_size: u16, timeout: Duration) -> Self {
        let bar = ProgressBar::new(ips.len() as u64);
        bar.set_style(ProgressStyle::default_bar()
            .template("[{bar}] {percent}% {pos}/{len}")
            .progress_chars("=> "));

        Self {
            ips,
            batch_size,
            timeout,
            bar,
        }
    }

    pub async fn scan(&mut self) -> Vec<Ipv4Addr> {
        let mut ip_iter = Ipv4Iter::new(self.ips);
        let mut futures = FuturesOrdered::new();
        let mut valid_ips = Vec::new();

        for _ in 0..self.batch_size {
            match ip_iter.next() {
                None => break,
                Some(ip) => futures.push(self.scan_ip(ip)),
            }
        }

        while let Some(scan_result) = futures.next().await {
            if let Some(ip) = ip_iter.next() {
                futures.push(self.scan_ip(ip));
            }

            self.bar.inc(1);

            if let Ok(ip) = scan_result {
                valid_ips.push(ip);
            }
        }

        self.bar.finish();
        valid_ips
    }

    async fn scan_ip(&self, ip: Ipv4Addr) -> io::Result<Ipv4Addr> {
        let stream = io::timeout(self.timeout, async move {
            ip.connect(25565).await
        }).await?;

        let _ = stream.shutdown(Shutdown::Both);
        Ok(ip)
    }

    pub async fn check(&mut self) -> Vec<Server> {
        let mut ip_iter = Ipv4Iter::new(self.ips);
        let mut futures = FuturesOrdered::new();
        let mut valid_servers = Vec::new();

        for _ in 0..self.batch_size {
            match ip_iter.next() {
                None => break,
                Some(ip) => futures.push(self.check_ip(ip)),
            }
        }

        while let Some(check_result) = futures.next().await {
            if let Some(ip) = ip_iter.next() {
                futures.push(self.check_ip(ip));
            }

            self.bar.inc(1);

            if let Ok(server) = check_result {
                valid_servers.push(server);
            }
        }

        self.bar.finish();
        valid_servers
    }

    async fn check_ip(&self, ip: Ipv4Addr) -> Result<Server, ScanError> {
        let (_, mut cur) = future::timeout(self.timeout, async move {
            let mut stream = ip.connect(25565).await?;
            stream.write_packet(Handshake {
                proto_ver: VarInt(47),
                server_addr: ip.to_string(),
                server_port: 25565,
                next_state: VarInt(1)
            }).await?;

            stream.write_packet(Empty).await?;
            stream.read_packet().await
        }).await??;

        let resp = Response::decode(&mut cur).await?;
        let info: Information = serde_json::from_str(&resp.json)?;
        Ok(Server {
            ip: ip.to_string(),
            info,
        })
    }
}