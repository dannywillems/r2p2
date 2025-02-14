#![warn(clippy::all)]

use clap::Parser;
use log::info;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};
// use r2p2::node::Node;

/// R2P2 Peer: Runs as a decentralized node.
#[derive(Parser)]
#[command(version = "0.1.0", about = "Runs an R2P2 decentralized peer.")]
struct PeerArgs {
    /// Local UDP port for discovery
    #[arg(default_value = "6001")]
    udp_port: u16,

    /// Local TCP port for communication
    #[arg(default_value = "7001")]
    tcp_port: u16,
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 128] = [0; 128];
    stream.read(&mut buffer).unwrap();
    info!("{:?}", buffer);
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = PeerArgs::parse();
    let tcp_addr = format!("127.0.0.1:{}", args.tcp_port);

    let listener: TcpListener = TcpListener::bind(tcp_addr).unwrap();
    info!("[*] Starting R2P2 peer on TCP {}...", args.tcp_port);

    info!("Listening for messages");
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
    Ok(())
}
