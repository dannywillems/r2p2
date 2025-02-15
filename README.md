# R2P2

R2P2 (Rust Peer-to-Peer) is a lightweight decentralized networking library
written in Rust. It uses only the standard library and low-level components for
efficiency.

## Features

- **Peer Discovery**: Uses UDP multicast to find other peers.
- **Messaging System**: Uses TCP for direct peer-to-peer communication.
- **Fully Decentralized**: No distinction between server and client; every node
  is a peer.
- **Efficient**: No external dependencies, purely built with Rust's standard
  library.

On top of the message system, encoding and encryption are provided.

What must be implemented:
- DDOS attack prevention
- handle peer disconnection
- handle blacklist + define what defines to be blacklisted
- encryption
- packets obfuscation to avoid censorship
- bandwith monitoring and limiting

Benchmarks should also be provided.

## Installation
Ensure you have Rust installed, then clone the repository and build the project:

```sh
cargo build --release
```

## Usage

Temporary

### Start a server

```sh
RUST_LOG=debug cargo run --release --bin r2p2-server -- 7001
```

### Start a client

```sh
RUST_LOG=debug cargo run --release --bin r2p2-client -- localhost 7001
```


### Start a Peer Node

To start a decentralized peer node:
```sh
cargo run --bin r2p2-peer -- --udp-port 6001 --tcp-port 7001
```

By default, it uses `UDP 6001` for discovery and `TCP 7001` for messaging. You
can start multiple nodes with different ports:


```sh
cargo run --bin r2p2-peer -- --udp-port 6002 --tcp-port 7002
cargo run --bin r2p2-peer -- --udp-port 6003 --tcp-port 7003
```

Each peer will:
- Discover other peers using UDP multicast.
- Accept incoming connections over TCP.
- Connect to known peers dynamically.

### Bootstrap Nodes

New peers need an initial set of nodes to connect to before full discovery can
happen. The bootstrap nodes are stored in a file (`bootstrap_nodes.txt`).

Example `bootstrap_nodes.txt`:

```
127.0.0.1:7001
127.0.0.1:7002
127.0.0.1:7003
```

When a peer starts, it will attempt to connect to these nodes first before using
multicast discovery.

To specify a custom bootstrap file:

```sh
cargo run --bin r2p2-peer -- --udp-port 6001 --tcp-port 7001 --bootstrap-file bootstrap_nodes.txt
```

## Architecture

- `discovery.rs`: Handles peer discovery using UDP multicast.
- `messaging.rs`: Manages peer-to-peer messaging using TCP.
- `node.rs`: Integrates discovery and messaging to form a decentralized peer.
- `bin/peer.rs`: CLI entry point for running a peer node.

## License

This project is licensed under the Apache license.

## Author
**Danny Willems**
- Email: [be.danny.willems@gmail.com](mailto:be.danny.willems@gmail.com)
- GitHub: [Danny Willems](https://github.com/dannywillems)

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## Inspirations

- [tezos-p2p](https://gitlab.com/tezos/tezos/-/blob/master/src/lib_p2p/)
- [libp2p](https://libp2p.io/)
- [ApnosNet](https://github.com/aptos-labs/aptos-core/tree/main/network)
- [Solana](https://solana.com/) - seems to be splitted in different crates
  - [Gossip](https://github.com/solana-labs/solana/blob/master/gossip/)
  - [Agave](https://github.com/anza-xyz/agave) - it seems that in Jan 2025 it has been forked there.
- [Nym/Sphinx](https://github.com/nymtech/sphinx/) - for packets
- [Sonic/Fantom](https://github.com/0xsoniclabs/sonic/tree/main/gossip)
  - It seems that Fantom has been rebranded into Sonic.
