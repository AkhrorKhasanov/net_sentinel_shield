# NetSentinel Shield

NetSentinel Shield is a high-performance, command-line security tool written in Rust. It allows users to manage network security by blocking access to specified domains at the operating system level via the hosts file.

# Features

System-level Blocking: Effectively blocks websites across all browsers and system applications.

Non-Destructive: Uses custom markers to ensure it only manages the domains you add, leaving the rest of your hosts file untouched.

High Performance: Lightweight and optimized using Rust for fast execution.

# Installation

Ensure you have Rust installed on your system.

Clone the repository:

```bash
git clone https://github.com/AkhrorKhasanov/net_sentinel_shield.git
cd net_sentinel_shield
```

Build the project in release mode:

```bash
cargo build --release
```

You can find the executable in target/release/net_sentinel_shield.exe.
