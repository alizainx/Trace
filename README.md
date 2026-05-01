# trace

Universal Process and Syscall Tracing Driver

A fast, secure, and distribution-agnostic process tracing tool written in pure Rust for Linux systems.

## Features
Real-time system call tracing using ptrace
Memory and CPU usage monitoring
Multiple output formats (Table, JSON, YAML)
Compatible with Debian, Ubuntu, Arch, Fedora, Alpine.
Fully static binary using musl (x86_64-unknown-linux-musl)
Secure privilege dropping
Static Build (musl)

To build a fully static binary:
```
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

```
Binary will be available at:

```
target/x86_64-unknown-linux-musl/release/trace
Installation (System-wide)

You can install the binary globally and run it from anywhere:

sudo cp target/x86_64-unknown-linux-musl/release/trace /usr/bin/trace
sudo chmod +x /usr/bin/trace
```
Now you can run:
```
trace --help

Quick Usage
# Basic tracing
trace --process firefox

# JSON output
trace --process code --format json

# Live tracing
trace --process firefox --live

```
Contributors

Contributions are welcome. Anyone can contribute to this project.
