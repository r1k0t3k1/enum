use clap::{Parser, Subcommand, ValueEnum};
use std::net::Ipv4Addr;
use anyhow::anyhow;

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

    pub fn parse_ipaddresses(&self) -> anyhow::Result<Vec<Ipv4Addr>> {
        let mut ipaddresses = Vec::new();
        
        match &self.subcommand {
            SubCommands::PortScan { targets, ports:_, scan_type:_ } => {
                for ip in targets.clone().split(",") {
                    match ip.parse::<Ipv4Addr>() {
                        Ok(ipaddress) => ipaddresses.push(ipaddress),
                        Err(e) => return Err(anyhow!("{}: {}", e, ip)),
                    } 
                };
            },
            _ => (),
        }
        Ok(ipaddresses)
    }

    pub fn parse_ports(&self) -> anyhow::Result<Vec<u16>> {
        let mut scan_port = Vec::new();
        
        match &self.subcommand {
            SubCommands::PortScan { targets: _, ports, scan_type:_ } => {
                for p in ports.clone().split(",") {
                    let v: Vec<&str> = p.split("-").collect();
                    if v.len() > 2 {
                       return Err(anyhow!("Invalid port range specified. {}", p));
                    } 

                    if v.len() == 2 {
                        let start: u16;
                        let end: u16;

                        match v[0].parse::<u16>() {
                            Ok(port) => start = port,
                            Err(e) => return Err(anyhow!("{}: {}", e, v[0])),
                        } 
                        match v[1].parse::<u16>() {
                            Ok(port) => end = port,
                            Err(e) => return Err(anyhow!("{}: {}", e, v[1])),
                        } 
                        for i in start..=end {
                            scan_port.push(i);
                        }
                        continue;
                    }
                    match p.parse::<u16>() {
                        Ok(port) => scan_port.push(port.clone()),
                        Err(e) => return Err(anyhow!("{}: {}", e, p)),
                    } 
                };
            },
            _ => (),
        }
        Ok(scan_port)
    }
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
