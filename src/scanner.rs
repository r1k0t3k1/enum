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
            let mut sockets = Vec::new();

            //batch size = 1024
            //show ulimit -> $ ulimit -Sn

            for target in &self.targets {
                for port in &self.ports {
                    sockets.push(SocketAddrV4::new(*target, *port));
                }
            } 
           
            let mut socket_iterator = sockets.iter();

            for _ in 0..999 {
                if let Some(socket) = socket_iterator.next() {
                    tasks.push(self.try_connect(*socket));
                } else {
                    break;
                }
            }

            while let Some(result) = tasks.next().await {
                if let Some(socket) = socket_iterator.next() {
                    tasks.push(self.try_connect(*socket));
                }

                match result {
                    Ok(socket) => {
                        println!("open: {}", socket.port());
                    },
                    Err(e) => {
                        //println!("{}", e);
                        //return Err(anyhow!("connect error: {}", e));
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
                        return Err(anyhow!("Max tries. port: {}", socket.port()));
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

