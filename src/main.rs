use beni::ColorString;
use beni::colors::EightBitColors::*;
use futures::executor::block_on;

mod argparser;
mod scanner;

fn main() {
    let args = argparser::Argument::new();

    let mut targets = Vec::new();
    match args.parse_ipaddresses() {
        Ok(arg) => targets = arg,
        Err(e)  => eprintln!("{}", e.to_string().fg_color(Red)),
    };

    let mut ports = Vec::new();
    match args.parse_ports() {
        Ok(arg) => ports = arg,
        Err(e)  => eprintln!("{}", e.to_string().fg_color(Red)),
    };
    
    let scanner = scanner::PortScanner::new(
        targets,
        ports,
        scanner::ScanType::CONNECT,
    );

    let scan_result = block_on(scanner.scan());
}
