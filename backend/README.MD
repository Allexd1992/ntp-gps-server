# NTP GPS Server Backend

Rust-based backend service providing NTP server, GPS data processing, and REST API for the NTP GPS Server system.

## Overview

The Backend Service is the core component of the NTP GPS Server system, written in Rust for high performance and reliability. It provides NTP server functionality, GPS data processing, RTC management, and a comprehensive REST API for system management and monitoring.

## Architecture

### Service Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   NTP Clients   │    │   Backend       │    │   Python        │
│   (Port 123)    │◄──►│   Service       │◄──►│   Services      │
│                 │    │   (Rust)        │    │   (TCP Sockets) │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                              │                        │
                              ▼                        ▼
                       ┌─────────────────┐    ┌─────────────────┐
                       │   Web GUI       │    │   GPS Module    │
                       │   (Port 8080)   │    │   (UART)        │
                       └─────────────────┘    └─────────────────┘
```

### Data Flow

```
1. GPS Module → Backend (GPS Data Processing)
2. Backend → RTC Service (Time Synchronization)
3. Backend → OLED Service (Display Updates)
4. Backend → Network Service (Configuration)
5. Backend → Login Service (Authentication)
6. NTP Clients ↔ Backend (Time Requests)
7. Web GUI ↔ Backend (REST API)
```

## Features

- **NTP Server**: Full NTP protocol implementation for time synchronization
- **GPS Integration**: Real-time GPS data processing and parsing
- **REST API**: Comprehensive web API for system management
- **Service Orchestration**: TCP socket communication with Python services
- **Configuration Management**: TOML-based configuration system
- **Logging**: Structured logging with multiple levels
- **CORS Support**: Cross-origin resource sharing for web interface
- **Swagger Documentation**: Auto-generated API documentation
- **High Performance**: Rust-based implementation for optimal performance

## API Reference

### REST API Endpoints

#### System Information
```http
GET /api/v1/system/info
GET /api/v1/system/status
GET /api/v1/system/uptime
```

#### GPS Data
```http
GET /api/v1/gps/status
GET /api/v1/gps/data
GET /api/v1/gps/satellites
```

#### NTP Configuration
```http
GET /api/v1/ntp/config
POST /api/v1/ntp/config
GET /api/v1/ntp/status
GET /api/v1/ntp/stats
```

#### Time Management
```http
GET /api/v1/time/current
GET /api/v1/time/rtc
POST /api/v1/time/sync
```

#### Network Configuration
```http
GET /api/v1/network/config
POST /api/v1/network/config
GET /api/v1/network/status
```

#### Authentication
```http
POST /api/v1/auth/login
POST /api/v1/auth/logout
GET /api/v1/auth/status
```

### API Documentation

Access the interactive API documentation at:
```
http://localhost:8080/api/v1/swagger/
```

### Example API Usage

#### Get System Status
```bash
curl -X GET http://localhost:8080/api/v1/system/status
```

#### Get GPS Data
```bash
curl -X GET http://localhost:8080/api/v1/gps/data
```

#### Update NTP Configuration
```bash
curl -X POST http://localhost:8080/api/v1/ntp/config \
  -H "Content-Type: application/json" \
  -d '{
    "server_list": ["0.ru.pool.ntp.org:123"],
    "enable": true,
    "cycle": 8000
  }'
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Logging level (debug, info, warn, error) |
| `WEB_SERVER_PORT` | `8080` | Web API server port |
| `NTP_SERVER_PORT` | `123` | NTP server port |
| `CONFIG_PATH` | `config/settings.toml` | Configuration file path |
| `GPS_DEVICE` | `/dev/ttyAMA0` | GPS device path |
| `GPS_BAUDRATE` | `9600` | GPS baud rate |

### Configuration File (settings.toml)

```toml
[ntp]
server_list = ["0.ru.pool.ntp.org:123", "1.ru.pool.ntp.org:123"]
enable = true
cycle = 8000
port = 123

[gps]
enable = true
device = "/dev/ttyAMA0"
baudrate = 9600
timeout = 5000

[display]
enable = true
update_interval = 1000

[rtc]
enable = true
sync_interval = 3600
device = "/dev/rtc1"

[web]
port = 8080
host = "0.0.0.0"
cors_enabled = true

[logging]
level = "info"
file = "/var/log/ntp-gps-server.log"
```

## Installation

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install system dependencies
sudo apt update
sudo apt install build-essential pkg-config libssl-dev

# Check Rust installation
rustc --version
cargo --version
```

### Development Build

```bash
# Clone or download project
cd backend

# Install dependencies and build
cargo build

# Run in development mode
cargo run
```

### Release Build

```bash
# Build optimized release version
cargo build --release

# Binary will be in: target/release/backend

# Run release binary
./target/release/backend
```

### Cross-Compilation for ARM

```bash
# Install ARM target
rustup target add aarch64-unknown-linux-gnu

# Cross-compile for ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Binary will be in: target/aarch64-unknown-linux-gnu/release/backend
```

## Systemd Service Management

### Service File Creation

```bash
sudo tee /etc/systemd/system/ntp-server.service << 'EOF'
[Unit]
Description=NTP GPS Server Backend
Documentation=https://github.com/your-repo/ntp-gps-server
After=network.target
Wants=network.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=/opt/ntp-gps-server/backend
ExecStart=/opt/ntp-gps-server/backend/backend
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

# Environment variables
Environment=RUST_LOG=info
Environment=CONFIG_PATH=/opt/ntp-gps-server/backend/config/settings.toml
Environment=WEB_SERVER_PORT=8080
Environment=NTP_SERVER_PORT=123

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ReadWritePaths=/opt/ntp-gps-server/backend/config

# Resource limits
LimitNOFILE=4096
MemoryMax=200M
CPUQuota=20%

[Install]
WantedBy=multi-user.target
EOF
```

### Service Management

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable and start service
sudo systemctl enable ntp-server.service
sudo systemctl start ntp-server.service

# Check status
sudo systemctl status ntp-server.service

# View logs
sudo journalctl -u ntp-server.service -f

# Restart service
sudo systemctl restart ntp-server.service
```

## Binary Deployment

### Manual Deployment

```bash
# Create installation directory
sudo mkdir -p /opt/ntp-gps-server/backend

# Copy binary and configuration
sudo cp target/release/backend /opt/ntp-gps-server/backend/
sudo cp -r config /opt/ntp-gps-server/backend/

# Set permissions
sudo chown -R root:root /opt/ntp-gps-server/backend
sudo chmod +x /opt/ntp-gps-server/backend/backend

# Create log directory
sudo mkdir -p /var/log/ntp-gps-server
sudo chown root:root /var/log/ntp-gps-server
```

### Automated Deployment Script

```bash
#!/bin/bash
# deploy-backend.sh

BINARY_NAME="backend"
INSTALL_DIR="/opt/ntp-gps-server/backend"
SERVICE_FILE="/etc/systemd/system/ntp-server.service"

echo "Deploying $BINARY_NAME..."

# Check if binary exists
if [ ! -f "target/release/$BINARY_NAME" ]; then
    echo "Error: Binary not found in target/release/$BINARY_NAME"
    echo "Please run: cargo build --release"
    exit 1
fi

# Create installation directory
sudo mkdir -p $INSTALL_DIR

# Copy binary and configuration
sudo cp target/release/$BINARY_NAME $INSTALL_DIR/
sudo cp -r config $INSTALL_DIR/

# Set permissions
sudo chown -R root:root $INSTALL_DIR
sudo chmod +x $INSTALL_DIR/$BINARY_NAME

# Create log directory
sudo mkdir -p /var/log/ntp-gps-server
sudo chown root:root /var/log/ntp-gps-server

# Update service file
if [ -f "$SERVICE_FILE" ]; then
    sudo sed -i "s|ExecStart=.*|ExecStart=$INSTALL_DIR/$BINARY_NAME|" $SERVICE_FILE
    echo "Updated service file: $SERVICE_FILE"
fi

# Reload systemd
sudo systemctl daemon-reload

echo "Backend deployed successfully!"
echo "Start service with: sudo systemctl start ntp-server.service"
```

## Monitoring and Logging

### Log Files

```bash
# View service logs
sudo journalctl -u ntp-server.service -f

# View application logs
tail -f /var/log/ntp-gps-server.log

# Check NTP server status
sudo netstat -tlnp | grep :123

# Check web API status
curl http://localhost:8080/api/v1/system/status
```

### Health Check

```bash
# Test NTP server
ntpdate -q 127.0.0.1

# Test web API
curl -X GET http://localhost:8080/api/v1/system/info

# Check GPS data
curl -X GET http://localhost:8080/api/v1/gps/status
```

### Performance Monitoring

```bash
# Check memory usage
ps aux | grep backend

# Check CPU usage
top -p $(pgrep backend)

# Check open files
lsof -p $(pgrep backend)
```

## Troubleshooting

### Common Issues

#### Service Won't Start
```bash
# Check service logs
sudo journalctl -u ntp-server.service -n 50

# Check binary permissions
ls -la /opt/ntp-gps-server/backend/backend

# Test binary manually
sudo /opt/ntp-gps-server/backend/backend

# Check configuration file
cat /opt/ntp-gps-server/backend/config/settings.toml
```

#### NTP Server Issues
```bash
# Check NTP port
sudo netstat -tlnp | grep :123

# Test NTP manually
ntpdate -q 127.0.0.1

# Check firewall
sudo ufw status
sudo ufw allow 123/udp
```

#### GPS Issues
```bash
# Check GPS device
ls -la /dev/ttyAMA0

# Test GPS manually
sudo cat /dev/ttyAMA0

# Check GPS permissions
sudo usermod -a -G dialout $USER
```

#### Web API Issues
```bash
# Check web server port
sudo netstat -tlnp | grep :8080

# Test API manually
curl http://localhost:8080/api/v1/system/status

# Check CORS settings
curl -H "Origin: http://localhost:3000" \
     -H "Access-Control-Request-Method: GET" \
     -H "Access-Control-Request-Headers: X-Requested-With" \
     -X OPTIONS http://localhost:8080/api/v1/system/status
```

### Debug Mode

```bash
# Run with debug logging
RUST_LOG=debug ./target/release/backend

# Run with specific log level
RUST_LOG=backend=debug,ntp=info ./target/release/backend

# Check configuration
RUST_LOG=debug ./target/release/backend --config-path config/settings.toml
```

## Development

### Project Structure

```
backend/
├── src/                    # Source code
│   ├── main.rs            # Application entry point
│   ├── ntp/               # NTP server implementation
│   ├── gps/               # GPS data processing
│   ├── api/               # REST API handlers
│   ├── config/            # Configuration management
│   └── utils/             # Utility functions
├── config/                # Configuration files
│   └── settings.toml      # Main configuration
├── Cargo.toml             # Rust dependencies
├── ntp-server.service     # Systemd service file
└── README.md             # This file
```

### Building and Testing

```bash
# Build project
cargo build

# Run tests
cargo test

# Run with specific features
cargo run --features "gps,ntp,api"

# Check code formatting
cargo fmt

# Run clippy (linting)
cargo clippy
```

### Adding Dependencies

```bash
# Add new dependency
cargo add dependency_name

# Add dependency with features
cargo add dependency_name --features "feature1,feature2"

# Update dependencies
cargo update
```

## Dependencies

### Core Dependencies
- **tokio**: Async runtime for high-performance networking
- **rocket**: Web framework for REST API
- **serde**: Serialization/deserialization
- **chrono**: Date and time handling
- **log**: Structured logging
- **toml**: Configuration file parsing

### NTP Dependencies
- **sntpc**: NTP client implementation
- **byteorder**: Byte order utilities
- **net2**: Network utilities

### GPS Dependencies
- **gpsd_proto**: GPS protocol parsing
- **time**: Time utilities

### API Dependencies
- **utoipa**: OpenAPI documentation
- **rocket_cors**: CORS support
- **sysinfo**: System information

## License

This project is part of the NTP GPS Server system. See main project license for details.

## Support

For issues and questions:
- Check troubleshooting section above
- Review service logs
- Open issue in project repository
- Contact system administrator