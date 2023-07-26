use clap::{Parser, Subcommand, ValueEnum};
use std::net::{IpAddr, Ipv4Addr};
use thiserror::Error;


#[derive(Parser, Debug)]
#[command(name = "renum")]
#[command(author = "rikoteki <r1k0t3k1@bird.ocn.ne.jp>")]
#[command(version = "1.0")]
#[command(about = "Enumeration tool", long_about = None)]
pub struct Argument {
    #[command(subcommand)]
    subcommand: SubCommands,
}

impl Argument {
    pub fn new() -> Self {
        Argument::parse()
    }

    pub fn parse_ipaddresses(&self) -> anyhow::Result<Vec<IpAddr>> {
        let mut ipaddresses = Vec::new();
        
        match &self.subcommand {
            SubCommands::PortScan { targets, ports:_, scan_type:_ } => {
                for ip in targets.clone().split(",") {
                    match ip.parse() {
                        Ok(ipaddress) => ipaddresses.push(IpAddr::V4(ipaddress)),
                        Err(_) => return Err(ParserError::InvalidIpAddressError{ ipaddress: ip.to_string() }.into()),
                    } 
                };
            },
            _ => (),
        }
        Ok(ipaddresses)
    }

//    pub fn parse_ports(&self) -> anyhow::Result<Vec<u16>> {
//        let mut ports = Vec::new();
//        
//        match &self.subcommand {
//            SubCommands::PortScan { targets: _, ports, scan_type:_ } => {
//                for p in ports.clone().split(",") {
//                    match ip.parse() {
//                        Ok(ipaddress) => ipaddresses.push(IpAddr::V4(ipaddress)),
//                        Err(_) => return Err(clap::Error::new(ErrorKind::InvalidValue).into()),
//                    } 
//                };
//            },
//            _ => (),
//        }
//        Ok(ipaddresses)
//    }
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[command(name = "portscan")]
    PortScan {
        #[arg(short, long)]
        targets: String,
        #[arg(short, long)]
        ports: String,
        #[arg(short, long)]
        scan_type: ScanType,
    },
    Dir,
}

#[derive(Debug, Clone, ValueEnum)]
enum ScanType {
    Syn,
    Connect,
    Udp,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid IP address specified. ipaddress: {}", .ipaddress)]
    InvalidIpAddressError { ipaddress: String },
}
