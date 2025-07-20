# Network Service

TCP service for managing network settings with NetworkManager integration.

## Overview

The Network Service provides a secure TCP socket interface for managing network configuration through NetworkManager. It allows dynamic configuration of network interfaces, IP addresses, gateways, and DNS settings for the NTP GPS Server system.

## Architecture

### Service Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client        │    │   Network       │    │   NetworkManager│
│   (Web GUI)     │◄──►│   Service       │◄──►│   (nmcli)       │
│   Port: 8080    │    │   Port: 7575    │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Network Configuration Flow

```
1. Client sends network configuration request via TCP socket
2. Service validates request format and parameters
3. Service executes nmcli commands with sudo privileges
4. NetworkManager applies configuration changes
5. Service returns operation result to client
```

## Features

- **Network Management**: Dynamic IP address, gateway, and DNS configuration
- **TCP Socket Interface**: JSON-based request/response protocol
- **NetworkManager Integration**: Uses nmcli for reliable network configuration
- **Error Handling**: Comprehensive error responses and validation
- **Security**: Privileged operations with proper access controls
- **High Performance**: Lightweight service with minimal resource usage

## API Reference

### Request Formats

#### Get Network Configuration
```json
{
  "type": "GET"
}
```

#### Set Network Configuration
```json
{
  "type": "SET",
  "config": {
    "address": "192.168.1.100/24",
    "gateway": "192.168.1.1",
    "dns": ["8.8.8.8", "8.8.4.4"],
    "connection_name": "Wired connection 1"
  }
}
```

#### Set DHCP Configuration
```json
{
  "type": "SET",
  "config": {
    "dhcp": true,
    "connection_name": "Wired connection 1"
  }
}
```

### Response Formats

#### Successful Get Response
```json
{
  "status": "success",
  "data": {
    "interface": "eth0",
    "connection_name": "Wired connection 1",
    "address": "192.168.1.100/24",
    "gateway": "192.168.1.1",
    "dns": ["8.8.8.8", "8.8.4.4"],
    "status": "connected",
    "dhcp": false
  },
  "timestamp": "2024-06-01T12:34:56Z"
}
```

#### Successful Set Response
```json
{
  "status": "success",
  "message": "Network configuration updated successfully",
  "data": {
    "interface": "eth0",
    "new_address": "10.0.0.2/24",
    "new_gateway": "10.0.0.1"
  },
  "timestamp": "2024-06-01T12:34:56Z"
}
```

#### Error Response
```json
{
  "status": "error",
  "message": "Invalid IP address format",
  "error_code": "INVALID_IP",
  "timestamp": "2024-06-01T12:34:56Z"
}
```

### Error Codes

| Code | Description |
|------|-------------|
| `INVALID_IP` | Invalid IP address or subnet format |
| `INVALID_GATEWAY` | Invalid gateway address |
| `INVALID_DNS` | Invalid DNS server address |
| `NMCLI_ERROR` | NetworkManager command failed |
| `PERMISSION_DENIED` | Insufficient privileges |
| `CONNECTION_NOT_FOUND` | Network connection not found |
| `SERVICE_ERROR` | Internal service error |

## Usage Examples

### Python Client
```python
import socket
import json

def get_network_config():
    """Get current network configuration."""
    try:
        sock = socket.create_connection(("127.0.0.1", 7575), timeout=10)
        
        request = {"type": "GET"}
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

def set_network_config(address, gateway, dns_servers):
    """Set network configuration."""
    try:
        sock = socket.create_connection(("127.0.0.1", 7575), timeout=10)
        
        request = {
            "type": "SET",
            "config": {
                "address": address,
                "gateway": gateway,
                "dns": dns_servers
            }
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

# Usage examples
config = get_network_config()
if config.get("status") == "success":
    print(f"Current IP: {config['data']['address']}")

result = set_network_config("192.168.1.100/24", "192.168.1.1", ["8.8.8.8", "1.1.1.1"])
if result.get("status") == "success":
    print("Network configuration updated!")
```

### Command Line Testing
```bash
# Get current configuration
echo '{"type": "GET"}' | nc 127.0.0.1 7575

# Set static IP
echo '{"type": "SET", "config": {"address": "192.168.1.100/24", "gateway": "192.168.1.1", "dns": ["8.8.8.8"]}}' | nc 127.0.0.1 7575

# Set DHCP
echo '{"type": "SET", "config": {"dhcp": true}}' | nc 127.0.0.1 7575
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SERVER_HOST` | `0.0.0.0` | Server bind address |
| `SERVER_PORT` | `7575` | Server port number |
| `CONNECTION_NAME` | `Wired connection 1` | Default network connection name |
| `LOG_LEVEL` | `INFO` | Logging level (DEBUG, INFO, WARNING, ERROR) |
| `TIMEOUT` | `30` | Command timeout in seconds |

### Configuration File
Create `config.json` for advanced configuration:
```json
{
  "server": {
    "host": "0.0.0.0",
    "port": 7575,
    "timeout": 30
  },
  "network": {
    "default_connection": "Wired connection 1",
    "fallback_dns": ["8.8.8.8", "8.8.4.4"]
  },
  "logging": {
    "level": "INFO",
    "file": "/var/log/network-service.log"
  }
}
```

## Installation

### Prerequisites
```bash
# Install NetworkManager and nmcli
sudo apt update
sudo apt install network-manager network-manager-cli

# Install Python dependencies
sudo apt install python3 python3-pip
```

### Manual Installation
```bash
# Clone or download service files
cd network-service

# Install Python dependencies
pip3 install -r requirements.txt

# Set executable permissions
chmod +x service.py

# Run service
python3 service.py
```

### Systemd Service Installation
```bash
# Copy service file
sudo cp network-service.service /etc/systemd/system/

# Reload systemd
sudo systemctl daemon-reload

# Enable and start service
sudo systemctl enable network-service.service
sudo systemctl start network-service.service

# Check status
sudo systemctl status network-service.service
```

## Binary Compilation

### Using PyInstaller
```bash
# Install PyInstaller
pip3 install pyinstaller

# Create binary
pyinstaller \
    --onefile \
    --name network-service \
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
sudo cp dist/network-service /opt/ntp-gps-server/network-service/

# Set permissions
sudo chmod +x /opt/ntp-gps-server/network-service/network-service
sudo chown root:root /opt/ntp-gps-server/network-service/network-service

# Update service file
sudo sed -i 's|ExecStart=.*|ExecStart=/opt/ntp-gps-server/network-service/network-service|' \
    /etc/systemd/system/network-service.service

# Reload and restart
sudo systemctl daemon-reload
sudo systemctl restart network-service.service
```

## Systemd Service Management

### Service File Creation
```bash
sudo tee /etc/systemd/system/network-service.service << 'EOF'
[Unit]
Description=Network Configuration Service
Documentation=https://github.com/your-repo/ntp-gps-server
After=network.target NetworkManager.service
Wants=network.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=/opt/ntp-gps-server/network-service
ExecStart=/opt/ntp-gps-server/network-service/network-service
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

# Environment variables
Environment=SERVER_HOST=0.0.0.0
Environment=SERVER_PORT=7575
Environment=CONNECTION_NAME=Wired connection 1

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
sudo systemctl enable network-service.service
sudo systemctl start network-service.service
sudo systemctl status network-service.service

# View logs
sudo journalctl -u network-service.service -f

# Restart service
sudo systemctl restart network-service.service
```

## Security Considerations

### Network Security
- Service binds to localhost only (127.0.0.1)
- Requires sudo privileges for network configuration
- NetworkManager provides additional security layers

### Access Control
```bash
# Configure sudo access for nmcli
echo "network-service ALL=(ALL) NOPASSWD: /usr/bin/nmcli" | sudo tee /etc/sudoers.d/network-service

# Restrict network interface access
sudo chmod 600 /etc/NetworkManager/NetworkManager.conf
```

## Monitoring and Logging

### Log Files
```bash
# View service logs
sudo journalctl -u network-service.service -f

# Check NetworkManager logs
sudo journalctl -u NetworkManager.service -f
```

### Health Check
```bash
# Test service health
echo '{"type": "GET"}' | nc -w 5 127.0.0.1 7575

# Check network status
nmcli connection show
nmcli device status
```

## Troubleshooting

### Common Issues

#### Service Won't Start
```bash
# Check NetworkManager status
sudo systemctl status NetworkManager.service

# Check service logs
sudo journalctl -u network-service.service -n 50

# Test nmcli manually
sudo nmcli connection show
```

#### Network Configuration Fails
```bash
# Check NetworkManager logs
sudo journalctl -u NetworkManager.service -f

# Test nmcli commands manually
sudo nmcli connection modify "Wired connection 1" ipv4.addresses "192.168.1.100/24"

# Check connection status
nmcli connection show --active
```

#### Permission Issues
```bash
# Check sudo configuration
sudo -l

# Test nmcli with sudo
sudo nmcli connection show

# Fix permissions
sudo chmod +x /opt/ntp-gps-server/network-service/network-service
```

### Debug Mode
```bash
# Run with debug logging
LOG_LEVEL=DEBUG python3 service.py

# Test nmcli commands
nmcli --version
nmcli connection show
```

## Dependencies

- Python 3.7+
- NetworkManager and nmcli
- sudo privileges for network configuration
- subprocess module (built-in)
- socket module (built-in)
- json module (built-in)
