use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[clap(name = "subcommand")]
pub struct Argument {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Portscan {
        #[arg(short, long)]
        targets: String,
        #[arg(short, long)]
        ports: String,
        #[arg(short, long)]
        scantype: ScanType,
    },
    Dir,
}

#[derive(Debug, Clone, ValueEnum)]
enum ScanType {
    Syn,
    Connect,
    Udp,
}
