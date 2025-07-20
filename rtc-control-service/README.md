# RTC Control Service

TCP service for managing hardware clock (RTC) with system time synchronization.

## Overview

The RTC Control Service provides a secure TCP socket interface for managing the hardware real-time clock (RTC). It allows getting and setting system time, synchronizing between RTC and system clock, and maintaining accurate timekeeping for the NTP GPS Server system.

## Architecture

### Service Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client        │    │   RTC Control   │    │   System        │
│   (Backend)     │◄──►│   Service       │◄──►│   (hwclock)     │
│   Port: 8080    │    │   Port: 6060    │    │   /dev/rtc1     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Time Synchronization Flow

```
1. Client sends time request via TCP socket
2. Service validates request and parameters
3. Service executes hwclock commands with sudo privileges
4. RTC hardware is read/written as needed
5. Service returns operation result to client
```

## Features

- **Hardware Clock Management**: Direct RTC device control via hwclock
- **System Time Synchronization**: Bidirectional sync between RTC and system clock
- **TCP Socket Interface**: JSON-based request/response protocol
- **Time Format Support**: Unix timestamp and human-readable formats
- **Error Handling**: Comprehensive error responses and validation
- **Security**: Privileged operations with proper access controls
- **High Precision**: Accurate timekeeping for NTP server operations

## API Reference

### Request Formats

#### Get RTC Time
```json
{
  "cmd": "get"
}
```

#### Set RTC Time (Unix Timestamp)
```json
{
  "cmd": "set",
  "ts": 1717230000
}
```

#### Set RTC Time (ISO String)
```json
{
  "cmd": "set",
  "time": "2024-06-01T12:34:56Z"
}
```

#### Sync RTC to System
```json
{
  "cmd": "sync_to_rtc"
}
```

#### Sync System to RTC
```json
{
  "cmd": "sync_to_system"
}
```

### Response Formats

#### Successful Get Response
```json
{
  "status": "success",
  "data": {
    "rtc_time": "2024-06-01 12:34:56",
    "system_time": "2024-06-01 12:34:56",
    "timestamp": 1717230000,
    "device": "/dev/rtc1",
    "drift": 0.5
  },
  "timestamp": "2024-06-01T12:34:56Z"
}
```

#### Successful Set Response
```json
{
  "status": "success",
  "message": "RTC time set successfully",
  "data": {
    "old_time": "2024-06-01 12:30:00",
    "new_time": "2024-06-01 12:34:56",
    "device": "/dev/rtc1"
  },
  "timestamp": "2024-06-01T12:34:56Z"
}
```

#### Error Response
```json
{
  "status": "error",
  "message": "Invalid timestamp format",
  "error_code": "INVALID_TIMESTAMP",
  "timestamp": "2024-06-01T12:34:56Z"
}
```

### Error Codes

| Code | Description |
|------|-------------|
| `INVALID_TIMESTAMP` | Invalid timestamp format |
| `HWCLOCK_ERROR` | hwclock command failed |
| `PERMISSION_DENIED` | Insufficient privileges |
| `DEVICE_NOT_FOUND` | RTC device not found |
| `SYNC_ERROR` | Time synchronization failed |
| `SERVICE_ERROR` | Internal service error |

## Usage Examples

### Python Client
```python
import socket
import json
import time

def get_rtc_time():
    """Get current RTC time."""
    try:
        sock = socket.create_connection(("127.0.0.1", 6060), timeout=10)
        
        request = {"cmd": "get"}
        sock.send(json.dumps(request).encode('utf-8'))
        
        response_data = sock.recv(1024).decode('utf-8')
        response = json.loads(response_data)
        
        sock.close()
        return response
        
    except Exception as e:
        return {
            "status": "error",
            "message": f"Connection error: {str(e)}",
            "error_code": "CONNECTION_ERROR"
        }

def set_rtc_time(timestamp):
    """Set RTC time using Unix timestamp."""
    try:
        sock = socket.create_connection(("127.0.0.1", 6060), timeout=10)
        
        request = {
            "cmd": "set",
            "ts": timestamp
        }
        
        sock.send(json.dumps(request).encode('utf-8'))
        
        response_data = sock.recv(1024).decode('utf-8')
        response = json.loads(response_data)
        
        sock.close()
        return response
        
    except Exception as e:
        return {
            "status": "error",
            "message": f"Connection error: {str(e)}",
            "error_code": "CONNECTION_ERROR"
        }

def sync_rtc_to_system():
    """Sync RTC time to system time."""
    try:
        sock = socket.create_connection(("127.0.0.1", 6060), timeout=10)
        
        request = {"cmd": "sync_to_rtc"}
        sock.send(json.dumps(request).encode('utf-8'))
        
        response_data = sock.recv(1024).decode('utf-8')
        response = json.loads(response_data)
        
        sock.close()
        return response
        
    except Exception as e:
        return {
            "status": "error",
            "message": f"Connection error: {str(e)}",
            "error_code": "CONNECTION_ERROR"
        }

# Usage examples
rtc_info = get_rtc_time()
if rtc_info.get("status") == "success":
    print(f"RTC Time: {rtc_info['data']['rtc_time']}")

# Set RTC to current time
current_timestamp = int(time.time())
result = set_rtc_time(current_timestamp)
if result.get("status") == "success":
    print("RTC time updated successfully!")

# Sync RTC to system
sync_result = sync_rtc_to_system()
if sync_result.get("status") == "success":
    print("RTC synchronized to system time!")
```

### Command Line Testing
```bash
# Get current RTC time
echo '{"cmd": "get"}' | nc 127.0.0.1 6060

# Set RTC time
echo '{"cmd": "set", "ts": 1717230000}' | nc 127.0.0.1 6060

# Sync RTC to system time
echo '{"cmd": "sync_to_rtc"}' | nc 127.0.0.1 6060

# Sync system to RTC time
echo '{"cmd": "sync_to_system"}' | nc 127.0.0.1 6060
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SERVER_ADDRESS` | `0.0.0.0` | Server bind address |
| `SERVER_PORT` | `6060` | Server port number |
| `DEVICE_PATH` | `/dev/rtc1` | RTC device path |
| `LOG_LEVEL` | `INFO` | Logging level (DEBUG, INFO, WARNING, ERROR) |
| `TIMEOUT` | `30` | Command timeout in seconds |

### Configuration File
Create `config.json` for advanced configuration:
```json
{
  "server": {
    "address": "0.0.0.0",
    "port": 6060,
    "timeout": 30
  },
  "rtc": {
    "device_path": "/dev/rtc1",
    "backup_device": "/dev/rtc0",
    "sync_interval": 3600
  },
  "logging": {
    "level": "INFO",
    "file": "/var/log/rtc-control-service.log"
  }
}
```

## Installation

### Prerequisites
```bash
# Install system utilities
sudo apt update
sudo apt install hwclock util-linux

# Check RTC devices
ls -la /dev/rtc*

# Install Python dependencies
sudo apt install python3 python3-pip
```

### Manual Installation
```bash
# Clone or download service files
cd rtc-control-service

# Install Python dependencies
pip3 install -r requirements.txt

# Set executable permissions
chmod +x service.py

# Test RTC access
sudo hwclock --show

# Run service
python3 service.py
```

### Systemd Service Installation
```bash
# Copy service file
sudo cp rtc_control_service.service /etc/systemd/system/

# Reload systemd
sudo systemctl daemon-reload

# Enable and start service
sudo systemctl enable rtc_control_service.service
sudo systemctl start rtc_control_service.service

# Check status
sudo systemctl status rtc_control_service.service
```

## Binary Compilation

### Using PyInstaller
```bash
# Install PyInstaller
pip3 install pyinstaller

# Create binary
pyinstaller \
    --onefile \
    --name rtc-control-service \
    --hidden-import=subprocess \
    --hidden-import=json \
    --hidden-import=socket \
    --strip \
    --optimize=2 \
    service.py
```

### Binary Deployment
```bash
# Copy binary to system
sudo cp dist/rtc-control-service /opt/ntp-gps-server/rtc-control-service/

# Set permissions
sudo chmod +x /opt/ntp-gps-server/rtc-control-service/rtc-control-service
sudo chown root:root /opt/ntp-gps-server/rtc-control-service/rtc-control-service

# Update service file
sudo sed -i 's|ExecStart=.*|ExecStart=/opt/ntp-gps-server/rtc-control-service/rtc-control-service|' \
    /etc/systemd/system/rtc_control_service.service

# Reload and restart
sudo systemctl daemon-reload
sudo systemctl restart rtc_control_service.service
```

## Systemd Service Management

### Service File Creation
```bash
sudo tee /etc/systemd/system/rtc-control-service.service << 'EOF'
[Unit]
Description=RTC Control Service
Documentation=https://github.com/your-repo/ntp-gps-server
After=network.target
Wants=network.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=/opt/ntp-gps-server/rtc-control-service
ExecStart=/opt/ntp-gps-server/rtc-control-service/rtc-control-service
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

# Environment variables
Environment=SERVER_ADDRESS=0.0.0.0
Environment=SERVER_PORT=6060
Environment=DEVICE_PATH=/dev/rtc1

# Security settings
NoNewPrivileges=true
PrivateTmp=true

[Install]
WantedBy=multi-user.target
EOF
```

### Service Management
```bash
# Basic operations
sudo systemctl daemon-reload
sudo systemctl enable rtc-control-service.service
sudo systemctl start rtc-control-service.service
sudo systemctl status rtc-control-service.service

# View logs
sudo journalctl -u rtc-control-service.service -f

# Restart service
sudo systemctl restart rtc-control-service.service
```

## Hardware Setup

### RTC Device Configuration
```bash
# Check available RTC devices
ls -la /dev/rtc*

# Check RTC kernel modules
lsmod | grep rtc

# Test RTC functionality
sudo hwclock --show
sudo hwclock --systohc
sudo hwclock --hctosys
```

### RTC Module Setup
```bash
# Enable RTC in device tree (if needed)
echo "dtoverlay=i2c-rtc,pcf8563" | sudo tee -a /boot/config.txt
# Use appropriate RTC chip (pcf8563, ds3231, etc.)

# Set RTC permissions
sudo chmod 666 /dev/rtc1

# Check RTC status
sudo hwclock --debug
```

## Monitoring and Logging

### Log Files
```bash
# View service logs
sudo journalctl -u rtc-control-service.service -f

# Check RTC status
sudo hwclock --show
sudo hwclock --debug
```

### Health Check
```bash
# Test service health
echo '{"cmd": "get"}' | nc -w 5 127.0.0.1 6060

# Check RTC device
ls -la /dev/rtc*
sudo hwclock --show
```

## Troubleshooting

### Common Issues

#### RTC Device Not Found
```bash
# Check RTC devices
ls -la /dev/rtc*

# Check kernel modules
lsmod | grep rtc

# Load RTC module
sudo modprobe rtc-pcf8563

# Check device tree
sudo dtoverlay -l
```

#### Permission Denied
```bash
# Check RTC permissions
ls -la /dev/rtc*

# Fix permissions
sudo chmod 666 /dev/rtc1
sudo chown root:root /dev/rtc1

# Test hwclock manually
sudo hwclock --show
```

#### Service Won't Start
```bash
# Check service logs
sudo journalctl -u rtc-control-service.service -n 50

# Test service manually
sudo python3 service.py

# Check hwclock availability
which hwclock
hwclock --version
```

### Debug Mode
```bash
# Run with debug logging
LOG_LEVEL=DEBUG python3 service.py

# Test hwclock commands manually
sudo hwclock --show
sudo hwclock --debug
```

## Dependencies

- Python 3.7+
- hwclock command (util-linux package)
- date command (coreutils package)
- sudo privileges for time setting
- subprocess module (built-in)
- socket module (built-in)
- json module (built-in)
- RTC hardware device
