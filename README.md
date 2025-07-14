# Rust System Monitor

A professional, colorful terminal-based system monitor written in Rust. This tool provides real-time monitoring of system resources including CPU, memory, processes, network, and disk usage through an interactive terminal user interface.

## Features

- **Multi-tab Interface**: Navigate between Overview, Processes, Network, and Disks tabs
- **Real-time Monitoring**: Live updates of system metrics and resource usage
- **Process Management**: View, sort, and terminate processes
- **Colorful TUI**: Professional color-coded interface without emojis
- **System Information**: Display comprehensive system details
- **Resource Gauges**: Visual CPU and memory usage indicators
- **Historical Charts**: Sparkline charts for CPU, memory, and network trends
- **Cross-platform**: Works on Windows, Linux, and macOS

## Installation

### Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo package manager

### Build from Source

```bash
# Clone the repository
git clone https://github.com/mapcrafter2048/rust-system-monitor.git
cd rust-system-monitor

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Usage

### Keyboard Controls

- `q` - Quit the application
- `h`/`l` - Switch between tabs (left/right)
- `j`/`k` - Navigate up/down in process list
- `r` - Refresh data manually
- `s` - Sort processes (cycles through: Name, CPU, Memory, PID)
- `Del` - Kill selected process (requires confirmation)

### Interface Tabs

1. **Overview**: System information, resource usage gauges, and historical charts
2. **Processes**: Sortable process list with CPU and memory usage
3. **Network**: Network statistics and traffic history
4. **Disks**: Disk usage information for all mounted drives

## Architecture

The application is structured into several modules:

- `main.rs` - Application entry point and event loop
- `app.rs` - Core application state and system monitoring logic
- `ui.rs` - Terminal user interface rendering
- `system_info.rs` - Utility functions for data formatting

## Dependencies

- **ratatui**: Terminal user interface framework
- **sysinfo**: Cross-platform system information library
- **crossterm**: Terminal manipulation
- **tokio**: Async runtime
- **clap**: Command-line argument parsing
- **chrono**: Date and time handling
- **anyhow**: Error handling

## Performance

- Minimal CPU overhead during monitoring
- Efficient memory usage with bounded history buffers
- Optimized terminal rendering with differential updates
- Non-blocking UI updates with async event handling

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## System Requirements

- **Memory**: 10-50 MB RAM
- **CPU**: Minimal impact (<1% on modern systems)
- **Terminal**: Any terminal with ANSI color support
- **OS**: Windows 10+, Linux, macOS 10.12+

## Troubleshooting

### Permission Issues (Linux/macOS)
If you encounter permission errors when killing processes:
```bash
sudo ./target/release/rust-system-monitor
```

### Terminal Compatibility
If colors don't display correctly, ensure your terminal supports ANSI escape sequences and 256-color mode.

## Future Enhancements

- Configuration file support
- Process filtering and search
- System alerts and notifications
- Plugin system for custom metrics
- Remote monitoring capabilities
- Log file analysis

---

*Developed with ❤️ in Rust for system administrators and developers*