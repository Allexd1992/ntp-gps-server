# OLED Connector

TCP service for outputting information to OLED display via I2C interface.

## Overview

The OLED Connector Service provides a TCP socket interface for displaying real-time information on an OLED display. It receives JSON data containing GPS coordinates, NTP time, system time, and network information, then renders this data on an I2C-connected OLED display for the NTP GPS Server system.

## Architecture

### Service Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client        │    │   OLED          │    │   OLED Display  │
│   (Backend)     │◄──►│   Service       │◄──►│   (I2C)         │
│   Port: 8080    │    │   Port: 5050    │    │   Address: 0x3C │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Display Data Flow

```
1. Backend service sends display data via TCP socket
2. OLED service receives and parses JSON data
3. Service initializes I2C connection to OLED display
4. Data is formatted and rendered on display
5. Service returns operation status to client
```

## Features

- **Real-time Display**: Live updates of GPS, NTP, and system information
- **TCP Socket Interface**: JSON-based data transmission protocol
- **I2C Integration**: Direct communication with OLED display hardware
- **Multi-line Display**: Support for multiple information lines
- **Font Rendering**: Custom font support for better readability
- **Error Handling**: Comprehensive error responses and display status
- **High Performance**: Optimized for embedded systems

## API Reference

### Request Format

```json
{
  "gps": "12:34:56.789",
  "ntp": "12:34:56.789",
  "time": "2024-06-01 12:34:56",
  "status": "connected",
  "network": "192.168.1.100",
  "satellites": 8
}
```

### Response Format

#### Successful Display
```json
{
  "status": "success",
  "message": "Data displayed successfully",
  "display_info": {
    "lines_rendered": 4,
    "display_time": "12:34:56",
    "i2c_address": "0x3C"
  },
  "timestamp": "2024-06-01T12:34:56Z"
}
```

#### Error Response
```json
{
  "status": "error",
  "message": "I2C communication failed",
  "error_code": "I2C_ERROR",
  "timestamp": "2024-06-01T12:34:56Z"
}
```

### Error Codes

| Code | Description |
|------|-------------|
| `I2C_ERROR` | I2C communication failure |
| `DISPLAY_NOT_FOUND` | OLED display not detected |
| `INVALID_DATA` | Malformed display data |
| `FONT_ERROR` | Font loading or rendering error |
| `SERVICE_ERROR` | Internal service error |

## Usage Examples

### Python Client
```python
import socket
import json
import time

def send_display_data(gps_time, ntp_time, system_time, status="connected"):
    """Send data to OLED display service."""
    try:
        sock = socket.create_connection(("127.0.0.1", 5050), timeout=5)
        
        data = {
            "gps": gps_time,
            "ntp": ntp_time,
            "time": system_time,
            "status": status,
            "network": "192.168.1.100",
            "satellites": 8
        }
        
        sock.send(json.dumps(data).encode('utf-8'))
        
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

# Usage example
while True:
    current_time = time.strftime("%Y-%m-%d %H:%M:%S")
    result = send_display_data(
        gps_time="12:34:56.789",
        ntp_time="12:34:56.789",
        system_time=current_time,
        status="connected"
    )
    
    if result.get("status") == "success":
        print("Display updated successfully")
    else:
        print(f"Display error: {result.get('message')}")
    
    time.sleep(1)
```

### Command Line Testing
```bash
# Test display with sample data
echo '{"gps": "12:34:56", "ntp": "12:34:56", "time": "2024-06-01 12:34:56", "status": "connected"}' | nc 127.0.0.1 5050

# Test with network information
echo '{"gps": "12:34:56", "ntp": "12:34:56", "time": "2024-06-01 12:34:56", "status": "connected", "network": "192.168.1.100", "satellites": 8}' | nc 127.0.0.1 5050
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SERVER_ADDRESS` | `0.0.0.0` | Server bind address |
| `SERVER_PORT` | `5050` | Server port number |
| `DISPLAY_PORT` | `1` | I2C bus port number |
| `DISPLAY_ADDRESS` | `0x3C` | I2C device address |
| `LOG_LEVEL` | `INFO` | Logging level (DEBUG, INFO, WARNING, ERROR) |
| `DISPLAY_WIDTH` | `128` | OLED display width in pixels |
| `DISPLAY_HEIGHT` | `64` | OLED display height in pixels |

### Configuration File
Create `config.json` for advanced configuration:
```json
{
  "server": {
    "address": "0.0.0.0",
    "port": 5050,
    "timeout": 30
  },
  "display": {
    "i2c_port": 1,
    "i2c_address": "0x3C",
    "width": 128,
    "height": 64,
    "font_size": 10
  },
  "logging": {
    "level": "INFO",
    "file": "/var/log/oled-connector.log"
  }
}
```

## Installation

### Prerequisites
```bash
# Install I2C tools and Python dependencies
sudo apt update
sudo apt install i2c-tools python3 python3-pip

# Enable I2C interface (Raspberry Pi)
sudo raspi-config
# Interface Options → I2C → Enable

# Or manually enable I2C
echo "i2c_arm=on" | sudo tee -a /boot/config.txt
echo "i2c_arm_baudrate=400000" | sudo tee -a /boot/config.txt

# Install Python packages
pip3 install oled Pillow netifaces
```

### Manual Installation
```bash
# Clone or download service files
cd oled-connector

# Install Python dependencies
pip3 install -r requirements.txt

# Set executable permissions
chmod +x service.py

# Test I2C connection
sudo i2cdetect -y 1

# Run service
python3 service.py
```

### Systemd Service Installation
```bash
# Copy service file
sudo cp oled-connector.service /etc/systemd/system/

# Reload systemd
sudo systemctl daemon-reload

# Enable and start service
sudo systemctl enable oled-connector.service
sudo systemctl start oled-connector.service

# Check status
sudo systemctl status oled-connector.service
```

## Binary Compilation

### Using PyInstaller
```bash
# Install PyInstaller
pip3 install pyinstaller

# Create binary
pyinstaller \
    --onefile \
    --name oled-connector \
    --hidden-import=oled \
    --hidden-import=PIL \
    --hidden-import=netifaces \
    --add-data "fonts/*:fonts/" \
    --strip \
    --optimize=2 \
    service.py
```

### Binary Deployment
```bash
# Copy binary to system
sudo cp dist/oled-connector /opt/ntp-gps-server/oled-connector/

# Set permissions
sudo chmod +x /opt/ntp-gps-server/oled-connector/oled-connector
sudo chown root:root /opt/ntp-gps-server/oled-connector/oled-connector

# Update service file
sudo sed -i 's|ExecStart=.*|ExecStart=/opt/ntp-gps-server/oled-connector/oled-connector|' \
    /etc/systemd/system/oled-connector.service

# Reload and restart
sudo systemctl daemon-reload
sudo systemctl restart oled-connector.service
```

## Systemd Service Management

### Service File Creation
```bash
sudo tee /etc/systemd/system/oled-connector.service << 'EOF'
[Unit]
Description=OLED Display Connector Service
Documentation=https://github.com/your-repo/ntp-gps-server
After=network.target
Wants=network.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=/opt/ntp-gps-server/oled-connector
ExecStart=/opt/ntp-gps-server/oled-connector/oled-connector
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

# Environment variables
Environment=SERVER_ADDRESS=0.0.0.0
Environment=SERVER_PORT=5050
Environment=DISPLAY_PORT=1
Environment=DISPLAY_ADDRESS=0x3C

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
sudo systemctl enable oled-connector.service
sudo systemctl start oled-connector.service
sudo systemctl status oled-connector.service

# View logs
sudo journalctl -u oled-connector.service -f

# Restart service
sudo systemctl restart oled-connector.service
```

## Hardware Setup

### I2C Configuration
```bash
# Check I2C devices
sudo i2cdetect -y 1

# Expected output should show device at address 0x3C:
#      0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
# 00:          -- -- -- -- -- -- -- -- -- -- -- -- --
# 10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
# 20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
# 30: -- -- -- -- -- -- -- -- -- -- -- -- 3c -- -- --
# 40: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
```

### OLED Display Wiring
```
OLED Display    Raspberry Pi/Orange Pi
VCC        →    3.3V
GND        →    GND
SCL        →    GPIO3 (SCL)
SDA        →    GPIO2 (SDA)
```

## Monitoring and Logging

### Log Files
```bash
# View service logs
sudo journalctl -u oled-connector.service -f

# Check I2C communication
sudo i2cdetect -y 1
sudo i2cget -y 1 0x3C 0x00
```

### Health Check
```bash
# Test service health
echo '{"gps": "test", "ntp": "test", "time": "test"}' | nc -w 5 127.0.0.1 5050

# Check display status
sudo i2cdetect -y 1 | grep 3c
```

## Troubleshooting

### Common Issues

#### I2C Communication Errors
```bash
# Check I2C bus
sudo i2cdetect -y 1

# Check I2C permissions
sudo usermod -a -G i2c $USER

# Test I2C communication
sudo i2cget -y 1 0x3C 0x00

# Check I2C kernel modules
lsmod | grep i2c
```

#### Display Not Found
```bash
# Check hardware connections
sudo i2cdetect -y 1

# Test different I2C addresses
sudo i2cdetect -y 1 | grep -E "(3c|3d|3e|3f)"

# Check power supply
sudo i2cget -y 1 0x3C 0x00
```

#### Service Won't Start
```bash
# Check service logs
sudo journalctl -u oled-connector.service -n 50

# Test service manually
sudo python3 service.py

# Check dependencies
pip3 list | grep -E "(oled|Pillow|netifaces)"
```

### Debug Mode
```bash
# Run with debug logging
LOG_LEVEL=DEBUG python3 service.py

# Test I2C manually
sudo i2cdetect -y 1
sudo i2cget -y 1 0x3C 0x00
```

## Dependencies

- Python 3.7+
- oled library (for I2C OLED display communication)
- Pillow (for image processing and font rendering)
- netifaces (for network interface information)
- socket module (built-in)
- json module (built-in)
- I2C hardware interface
