# Rust Port Scanner

A simple asynchronous TCP port scanner written in Rust using `tokio` and `clap`.

## Features

- Scan a range of TCP ports on a target IP or domain.
- Asynchronous and concurrent scanning for speed.
- Configurable timeout, concurrency, and port range via command-line arguments.

## Requirements

- Rust (stable)
- Cargo (Rust package manager)

## Installation

Clone the repository and build the project:

```bash
git clone 
cd port-scanner
cargo build --release
```

## Usage

Run the scanner with the following options:

```bash
cargo run -- --target <IP_OR_DOMAIN> [OPTIONS]
```

## Arguments
```
    --target, -t — Target IP address or domain to scan (required).

    --start-port — Starting port number (default: 1).

    --end-port — Ending port number (default: 1024).

    --timeout-ms, -T — Timeout in milliseconds for each connection attempt (default: 500).

    --concurrency, -c — Maximum number of concurrent connection attempts (default: 100).
```

## Example

Scan ports 1 to 500 on 192.168.0.1 with 300ms timeout and concurrency of 50:
```bash
cargo run -- --target 192.168.0.1 --start-port 1 --end-port 500 --timeout-ms 300 --concurrency 50
```
Or using short options:
```bash
cargo run -- -t 192.168.0.1 -T 300 -c 50
```

License

MIT License