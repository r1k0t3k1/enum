use std::net::IpAddr;
use thiserror::Error;
use anyhow::Error;

pub struct PortScanner {
    targets: Vec<IpAddr>,
    ports: Vec<u16>,
    scan_type: ScanType,
}

pub enum ScanType {
    SYN,
    CONNECT,
    UDP,
}

#[derive(Debug, Error)]
pub enum PortScanError {
    #[error("Port scan error.")] 
    AnyError,
    #[error("Not implemented.")] 
    NotImplementedError,
}

impl PortScanner {
    pub fn new(targets: Vec<IpAddr>, ports: Vec<u16>, scan_type: ScanType) -> Self {
        PortScanner {
            targets,
            ports,
            scan_type,
        }
    }

    pub fn scan(&self) -> anyhow::Result<()> {
        Err(PortScanError::NotImplementedError.into())
    }
}

