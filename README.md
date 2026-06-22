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

# Usage

Important: This application modifies system files (hosts). You must run your terminal or command prompt as an Administrator for the tool to function correctly.

1. Preparing your blocklist
2. 
Create a simple text file (e.g., domains.txt) and list the domains you wish to block, one per line:

facebook.com
www.facebook.com
instagram.com
www.instagram.com
chess.com
www.chess.com
monkeytype.com
www.monkeytype.com

2. Blocking Domains
To block the domains listed in your text file, run:

```bash
net_sentinel_shield.exe block /path/to/domains.txt
```

3. Restoring Access
To remove the blocked domains and restore your hosts file to its original state, run:

```bash
net_sentinel_shield.exe restart
