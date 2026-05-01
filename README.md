trace

Universal Process and Syscall Tracing Driver

A fast, secure, and distribution-agnostic process tracing tool written in pure Rust for Linux systems.

Features
Real-time system call tracing using ptrace
Memory and CPU usage monitoring
Multiple output formats: Table, JSON, YAML
Compatible with major Linux distributions including Debian, Ubuntu, Arch, Fedora, and Alpine
Fully static binary using musl
Secure privilege dropping
Installation
Build from Source
git clone https://github.com/alizainx/Trace.git
cd trace
cargo build --release

Binary will be available at:

./target/release/trace
Usage
Basic Tracing
trace --process firefox
Trace by PID
trace --pid 1234
JSON Output
trace --pid 1234 --format json
YAML Output
trace --pid 1234 --format yaml
Save Output
trace --process firefox --output ./traces
System Information
trace info
List Processes
trace processes
Command-Line Options
-p, --process <NAME>    Trace a process by name
-P, --pid <PID>         Trace a process by PID
-f, --format <FORMAT>   Output format (table, json, yaml)
-o, --output <DIR>      Save output to directory
-l, --live              Enable live tracing (future support)
-v, --verbose           Enable verbose logging
-h, --help              Show help information
-V, --version           Show version information
Supported Platforms
Linux x86_64
Linux ARM64
Linux ARM
Output Formats
Table – Default human-readable format
JSON – Machine-readable for automation
YAML – Structured and configuration-friendly
Security
Safe error handling (no runtime panics)
Input validation for CLI arguments
Secure file operations
Privilege dropping support
Non-invasive, read-only tracing
Performance
Metric	Value
Startup Time	<100 ms
Memory Usage	~1.5 MB
Binary Size	~7.3 MB
Contributing

Contributions are welcome. Please open an issue to discuss changes before submitting a pull request.


Commit your changes
Push to your fork
Open a pull request


License

This project is licensed under the MIT License. See the LICENSE file for details.

# JSON output
trace --process code --json

# Live tracing
trace --process firefox --live
