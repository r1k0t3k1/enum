use async_std::io;
use std::time::Duration;
use std::net::{Ipv4Addr, SocketAddrV4, Shutdown};
use async_std::net::TcpStream;
use anyhow::{Error, anyhow};
use futures::stream::FuturesUnordered;
use futures::prelude::*;

#[derive(Debug)]
pub struct PortScanner {
    targets: Vec<Ipv4Addr>,
    ports: Vec<u16>,
    scan_type: ScanType,
}

#[derive(Debug, PartialEq)]
pub enum ScanType {
    SYN,
    CONNECT, UDP,
}

impl PortScanner {
    pub fn new(targets: Vec<Ipv4Addr>, ports: Vec<u16>, scan_type: ScanType) -> Self {
        PortScanner {
            targets,
            ports,
            scan_type,
        }
    }

    pub async fn scan(&self) -> anyhow::Result<()> {
        if self.scan_type == ScanType::CONNECT {
            let mut tasks = FuturesUnordered::new();
            for target in &self.targets {
                for port in &self.ports {
                    let socket = SocketAddrV4::new(*target, *port);
                    tasks.push(self.try_connect(socket));
                    while let Some(Ok(socket)) = tasks.next().await {
                        println!("open: {}", socket.port());
                    }
                }
            } 
        }
        Err(anyhow!("Not Implemented."))
    }
    
    async fn try_connect(&self, socket: SocketAddrV4) -> anyhow::Result<SocketAddrV4> {
        let tries = 2;

        for current_try in 1..=tries {
            match &self.connect(socket).await {
                Ok(s) => {
                    s.shutdown(Shutdown::Both);
                    return Ok(socket);
                },
                Err(e) => {
                    if current_try == tries {
                        return Err(anyhow!("Max tries."));
                    }
                }
            }
        }
        unreachable!();
    }

    async fn connect(&self, socket: SocketAddrV4) -> anyhow::Result<TcpStream> {
        let stream = io::timeout(
            Duration::from_millis(500),
            async move { TcpStream::connect(socket).await },
        ).await?;
        Ok(stream)
    }
}

