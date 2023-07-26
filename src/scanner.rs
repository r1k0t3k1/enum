use std::net::{Ipv4Addr, SocketAddrV4};
use async_std::net::TcpStream;
use anyhow::{Error, anyhow};

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
            println!("scan start");
            for target in &self.targets {
                for port in &self.ports {
                    let socket = SocketAddrV4::new(*target, *port);
                    let stream = TcpStream::connect(socket).await?;
                }
            } 
        }
        Err(anyhow!("Not Implemented."))
    }
}

