# trace

**Universal Process & Syscall Tracing Driver**

A fast, secure, and distro-agnostic process tracing tool written in pure Rust for Zainium OS and other Linux distributions.

### Features

- Real-time syscall tracing using ptrace
- Memory and CPU usage monitoring
- Multiple output formats (Table, JSON, YAML)
- Works on Debian, Ubuntu, Arch, Fedora, Alpine, and Zainium OS
- Fully static binary (musl)
- Secure privilege dropping

### Quick Usage

```bash
# Basic tracing
trace --process firefox
