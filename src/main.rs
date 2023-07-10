//! Playing with sockets

use clap::{Parser, ValueEnum};
use std::net::{IpAddr, Ipv4Addr};

pub mod tcp;
pub mod udp;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// CLI arguments
pub struct Args {
    /// Protocol to use
    #[arg(value_enum, short, long, default_value_t = Protocol::TCP)]
    protocol: Protocol,

    /// Address to listen on
    #[arg(short, long, default_value_t = Ipv4Addr::new(127, 0, 0, 1))]
    address: Ipv4Addr,

    /// Port to listen on
    #[arg(short = 'P', long, default_value_t = 8000)]
    port: u16,

    /// Output file for UDP messages
    #[arg(short, long, default_value_t = String::from("output.txt"))]
    file: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
/// Supported protocols
enum Protocol {
    TCP,
    UDP,
}

/// Number of threads in a threadpool
const NTHREADS: usize = 4;

/// Parse arguments and call socket handler
pub fn main() {
    let args = Args::parse();

    let result = match args.protocol {
        Protocol::TCP => tcp::listen(IpAddr::V4(args.address), args.port, NTHREADS),
        Protocol::UDP => udp::listen(IpAddr::V4(args.address), args.port, &args.file),
    };

    if let Err(e) = result {
        println!("Error: {}", e.to_string());
    }
}
