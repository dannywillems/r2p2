#![warn(clippy::all)]

use clap::Parser;
use log::info;
use std::{io::Write, net::TcpStream};

/// R2P2 Peer: Runs as a decentralized node.
#[derive(Parser)]
#[command(version = "0.1.0", about = "Runs an R2P2 decentralized peer.")]
struct PeerArgs {
    /// IP to send packets to
    #[arg(default_value = "127.0.0.1")]
    ip: String,

    /// Local TCP port for communication
    #[arg(default_value = "7001")]
    tcp_port: u16,
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = PeerArgs::parse();

    let tcp_addr = format!("{}:{}", args.ip, args.tcp_port);

    info!("[*] Connecting to client {tcp_addr}");
    let mut stream = TcpStream::connect(tcp_addr)?;
    let msg: [u8; 128] = [1; 128];
    info!("Sending message 1");
    let _ = stream.write_all(&msg);

    Ok(())
}
