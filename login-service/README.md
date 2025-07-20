# Login Service

TCP service for user authentication with system integration.

## Overview

The Login Service provides a secure TCP socket interface for user authentication, validating credentials against the system's `/etc/shadow` file. It serves as the authentication backend for the NTP GPS Server web interface and other services requiring user verification.

## Architecture

### Service Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client        │    │   Login Service │    │   System        │
│   (Web GUI)     │◄──►│   (Python)      │◄──►│   (/etc/shadow) │
│   Port: 8080    │    │   Port: 7070    │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Authentication Flow

```
1. Client sends credentials via TCP socket
2. Service reads /etc/shadow file
3. Password hash comparison using crypt module
4. Authentication result returned to client
```

## Features

- **Secure Authentication**: Password validation against system `/etc/shadow`
- **TCP Socket Interface**: JSON-based request/response protocol
- **Built-in Security**: Uses Python's crypt module for secure hashing
- **Error Handling**: Comprehensive error responses and logging
- **System Integration**: Direct integration with Linux user management
- **High Performance**: Lightweight service with minimal resource usage

## API Reference

### Request Format

```json
{
  "login": "username",
  "password": "userpassword"
}
```

### Response Formats

#### Successful Authentication
```json
{
  "status": "success",
  "authenticated": true,
  "user": "username",
  "message": "Authentication successful",
  "timestamp": "2024-06-01T12:34:56Z"
}
```

#### Failed Authentication
```json
{
  "status": "error",
  "authenticated": false,
  "message": "Invalid credentials",
  "error_code": "AUTH_FAILED",
  "timestamp": "2024-06-01T12:34:56Z"
}
```

#### Service Error
```json
{
  "status": "error",
  "authenticated": false,
  "message": "Service unavailable",
  "error_code": "SERVICE_ERROR",
  "timestamp": "2024-06-01T12:34:56Z"
}
```

### Error Codes

| Code | Description |
|------|-------------|
| `AUTH_FAILED` | Invalid username or password |
| `USER_NOT_FOUND` | Username does not exist |
| `SERVICE_ERROR` | Internal service error |
| `INVALID_REQUEST` | Malformed request format |
| `PERMISSION_DENIED` | Cannot access /etc/shadow |

## Usage Examples

### Python Client
```python
import socket
import json

def authenticate_user(username, password):
    """Authenticate user against login service."""
    try:
        # Connect to login service
        sock = socket.create_connection(("127.0.0.1", 7070), timeout=5)
        
        # Prepare authentication request
        request = {
            "login": username,
            "password": password
        }
        
        # Send request
        sock.send(json.dumps(request).encode('utf-8'))
        
        # Receive response
        response_data = sock.recv(1024).decode('utf-8')
        response = json.loads(response_data)
        
        sock.close()
        
        return response
        
    except Exception as e:
        return {
            "status": "error",
            "authenticated": false,
            "message": f"Connection error: {str(e)}",
            "error_code": "CONNECTION_ERROR"
        }

# Usage example
result = authenticate_user("root", "orangepi")
if result.get("authenticated"):
    print("Authentication successful!")
else:
    print(f"Authentication failed: {result.get('message')}")
```

### Command Line Testing
```bash
# Test with netcat
echo '{"login": "root", "password": "orangepi"}' | nc 127.0.0.1 7070

# Test with telnet
telnet 127.0.0.1 7070
# Then type: {"login": "root", "password": "orangepi"}
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SERVER_HOST` | `0.0.0.0` | Server bind address |
| `SERVER_PORT` | `7070` | Server port number |
| `LOG_LEVEL` | `INFO` | Logging level (DEBUG, INFO, WARNING, ERROR) |
| `MAX_CONNECTIONS` | `10` | Maximum concurrent connections |
| `TIMEOUT` | `30` | Connection timeout in seconds |

## Installation

### Prerequisites
```bash
# Install Python 3.7+
sudo apt update
sudo apt install python3 python3-pip

# Install required packages
sudo apt install python3-crypt
```

### Manual Installation
```bash
# Clone or download service files
cd login-service

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
sudo cp login-detect.service /etc/systemd/system/

# Reload systemd
sudo systemctl daemon-reload

# Enable and start service
sudo systemctl enable login-detect.service
sudo systemctl start login-detect.service

# Check status
sudo systemctl status login-detect.service
```

### Binary Compilation

#### Using PyInstaller (Recommended)

##### Prerequisites
```bash
# Install PyInstaller
pip3 install pyinstaller

# Install additional dependencies for binary
pip3 install setuptools wheel
```

##### Basic Binary Creation
```bash
# Navigate to service directory
cd login-service

# Create standalone binary
pyinstaller --onefile --name login-service service.py

# Binary will be created in: dist/login-service
```

##### Optimized Binary Creation
```bash
# Create optimized binary with hidden imports
pyinstaller \
    --onefile \
    --name login-service \
    --hidden-import=crypt \
    --hidden-import=json \
    --hidden-import=socket \
    --hidden-import=logging \
    --hidden-import=os \
    --hidden-import=sys \
    --strip \
    --optimize=2 \
    service.py
```

##### Cross-Platform Compilation
```bash
# For ARM64 (Raspberry Pi, Orange Pi)
pyinstaller \
    --onefile \
    --name login-service-arm64 \
    --target-arch=arm64 \
    service.py

# For ARM32 (older Raspberry Pi)
pyinstaller \
    --onefile \
    --name login-service-arm32 \
    --target-arch=arm \
    service.py
```

#### Using cx_Freeze

##### Setup Configuration
```bash
# Install cx_Freeze
pip3 install cx_Freeze

# Create setup.py
cat > setup.py << 'EOF'
from cx_Freeze import setup, Executable
import sys

# Dependencies
build_exe_options = {
    "packages": ["crypt", "json", "socket", "logging", "os", "sys"],
    "excludes": ["tkinter", "unittest"],
    "include_files": [],
    "optimize": 2
}

# Base for console application
base = None
if sys.platform == "win32":
    base = "Win32GUI"

setup(
    name="login-service",
    version="1.0.0",
    description="Login Authentication Service",
    options={"build_exe": build_exe_options},
    executables=[
        Executable(
            "service.py", 
            base=base,
            target_name="login-service",
            icon=None
        )
    ]
)
EOF
```

##### Build Binary
```bash
# Build binary
python3 setup.py build

# Binary will be in: build/exe.linux-x86_64-3.x/login-service
```

#### Using Nuitka (Alternative)

##### Installation and Compilation
```bash
# Install Nuitka
pip3 install nuitka

# Compile to binary
python3 -m nuitka \
    --onefile \
    --follow-imports \
    --include-module=crypt \
    --include-module=json \
    --include-module=socket \
    --output-filename=login-service \
    service.py
```

#### Using Docker for Cross-Compilation

##### Dockerfile for ARM Compilation
```dockerfile
FROM python:3.9-slim

# Install build dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    g++ \
    make \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install PyInstaller
RUN pip install pyinstaller

# Copy source code
COPY . /app
WORKDIR /app

# Compile for ARM64
RUN pyinstaller \
    --onefile \
    --name login-service-arm64 \
    --target-arch=arm64 \
    service.py

# Compile for ARM32
RUN pyinstaller \
    --onefile \
    --name login-service-arm32 \
    --target-arch=arm \
    service.py
```

##### Build Commands
```bash
# Build Docker image
docker build -t login-service-builder .

# Extract binaries
docker create --name temp login-service-builder
docker cp temp:/app/dist/login-service-arm64 ./login-service-arm64
docker cp temp:/app/dist/login-service-arm32 ./login-service-arm32
docker rm temp
```

#### Binary Deployment

##### Manual Deployment
```bash
# Copy binary to system
sudo cp dist/login-service /opt/ntp-gps-server/login-service/

# Set permissions
sudo chmod +x /opt/ntp-gps-server/login-service/login-service
sudo chown root:root /opt/ntp-gps-server/login-service/login-service

# Update systemd service file
sudo sed -i 's|ExecStart=.*|ExecStart=/opt/ntp-gps-server/login-service/login-service|' \
    /etc/systemd/system/login-detect.service

# Reload and restart
sudo systemctl daemon-reload
sudo systemctl restart login-detect.service
```

##### Automated Deployment Script
```bash
#!/bin/bash
# deploy-binary.sh

BINARY_NAME="login-service"
INSTALL_DIR="/opt/ntp-gps-server/login-service"
SERVICE_FILE="/etc/systemd/system/login-detect.service"

echo "Deploying $BINARY_NAME binary..."

# Check if binary exists
if [ ! -f "dist/$BINARY_NAME" ]; then
    echo "Error: Binary not found in dist/$BINARY_NAME"
    exit 1
fi

# Create installation directory
sudo mkdir -p $INSTALL_DIR

# Copy binary
sudo cp dist/$BINARY_NAME $INSTALL_DIR/
sudo chmod +x $INSTALL_DIR/$BINARY_NAME
sudo chown root:root $INSTALL_DIR/$BINARY_NAME

# Update service file
if [ -f "$SERVICE_FILE" ]; then
    sudo sed -i "s|ExecStart=.*|ExecStart=$INSTALL_DIR/$BINARY_NAME|" $SERVICE_FILE
    echo "Updated service file: $SERVICE_FILE"
else
    echo "Warning: Service file not found: $SERVICE_FILE"
fi

# Reload systemd
sudo systemctl daemon-reload

echo "Binary deployed successfully!"
echo "Restart service with: sudo systemctl restart login-detect.service"
```

#### Binary Verification

##### Test Binary
```bash
# Test binary functionality
./dist/login-service --help

# Test authentication
echo '{"login": "root", "password": "test"}' | ./dist/login-service

# Check binary dependencies
ldd dist/login-service

# Check binary size
ls -lh dist/login-service
```

##### Performance Testing
```bash
# Test binary performance
time ./dist/login-service --test

# Compare with Python script
time python3 service.py --test

# Memory usage
/usr/bin/time -v ./dist/login-service
```

#### Troubleshooting Binary Issues

##### Common PyInstaller Issues
```bash
# Missing modules error
pyinstaller --hidden-import=missing_module service.py

# Import error
pyinstaller --collect-all=crypt service.py

# File not found error
pyinstaller --add-data "config.json:." service.py

# Permission denied
chmod +x dist/login-service
```

##### Binary Size Optimization
```bash
# Create minimal binary
pyinstaller \
    --onefile \
    --strip \
    --exclude-module=tkinter \
    --exclude-module=unittest \
    --exclude-module=email \
    --exclude-module=html \
    --exclude-module=http \
    --exclude-module=urllib \
    service.py
```

##### Debug Binary Issues
```bash
# Run with debug output
./dist/login-service --debug

# Check binary symbols
nm dist/login-service | grep -i main

# Check binary architecture
file dist/login-service

# Run with strace for system calls
strace ./dist/login-service
```

## Systemd Service Management

### Service File Creation

#### Basic Service File
```bash
# Create systemd service file
sudo tee /etc/systemd/system/login-detect.service << 'EOF'
[Unit]
Description=Login Authentication Service
After=network.target
Wants=network.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=/opt/ntp-gps-server/login-service
ExecStart=/opt/ntp-gps-server/login-service/login-service
# Or for Python script: ExecStart=/usr/bin/python3 /opt/ntp-gps-server/login-service/service.py
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

# Environment variables
Environment=SERVER_HOST=0.0.0.0
Environment=SERVER_PORT=7070
Environment=LOG_LEVEL=INFO

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ReadOnlyPaths=/
ReadWritePaths=/etc/shadow

[Install]
WantedBy=multi-user.target
EOF
```

#### Advanced Service File with Security
```bash
# Create enhanced service file with security features
sudo tee /etc/systemd/system/login-detect.service << 'EOF'
[Unit]
Description=Login Authentication Service
Documentation=https://github.com/your-repo/ntp-gps-server
After=network.target systemd-user-sessions.service
Wants=network.target
Conflicts=shutdown.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=/opt/ntp-gps-server/login-service
ExecStart=/opt/ntp-gps-server/login-service/login-service
ExecReload=/bin/kill -HUP $MAINPID
Restart=always
RestartSec=5
StartLimitInterval=60
StartLimitBurst=3
TimeoutStartSec=30
TimeoutStopSec=30

# Environment variables
Environment=SERVER_HOST=0.0.0.0
Environment=SERVER_PORT=7070
Environment=LOG_LEVEL=INFO
Environment=MAX_CONNECTIONS=10
Environment=TIMEOUT=30

# Security settings
NoNewPrivileges=true
PrivateTmp=true
PrivateDevices=true
ProtectSystem=strict
ProtectHome=true
ReadOnlyPaths=/
ReadWritePaths=/etc/shadow /var/log
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictRealtime=true
RestrictSUIDSGID=true
LockPersonality=true
MemoryDenyWriteExecute=true

# Resource limits
LimitNOFILE=1024
LimitNPROC=100
MemoryMax=50M
CPUQuota=10%

[Install]
WantedBy=multi-user.target
EOF
```

### Service Management Commands

#### Basic Service Operations
```bash
# Reload systemd configuration
sudo systemctl daemon-reload

# Enable service to start on boot
sudo systemctl enable login-detect.service

# Disable service from starting on boot
sudo systemctl disable login-detect.service

# Start service
sudo systemctl start login-detect.service

# Stop service
sudo systemctl stop login-detect.service

# Restart service
sudo systemctl restart login-detect.service

# Reload service (if supported)
sudo systemctl reload login-detect.service

# Check service status
sudo systemctl status login-detect.service

# Check if service is enabled
sudo systemctl is-enabled login-detect.service

# Check if service is active
sudo systemctl is-active login-detect.service
```

#### Advanced Service Management
```bash
# Show service configuration
sudo systemctl show login-detect.service

# Show service dependencies
sudo systemctl list-dependencies login-detect.service

# Show service environment
sudo systemctl show-environment login-detect.service

# Set service environment variable
sudo systemctl set-environment LOG_LEVEL=DEBUG

# Reset service environment
sudo systemctl unset-environment LOG_LEVEL

# Mask service (prevent starting)
sudo systemctl mask login-detect.service

# Unmask service
sudo systemctl unmask login-detect.service

# Revert service to vendor version
sudo systemctl revert login-detect.service
```

### Logging and Monitoring

#### View Service Logs
```bash
# View recent logs
sudo journalctl -u login-detect.service

# Follow logs in real-time
sudo journalctl -u login-detect.service -f

# View logs since last boot
sudo journalctl -u login-detect.service -b

# View logs for specific time period
sudo journalctl -u login-detect.service --since "2024-06-01 10:00:00" --until "2024-06-01 11:00:00"

# View logs with timestamps
sudo journalctl -u login-detect.service -o short-iso

# View only error messages
sudo journalctl -u login-detect.service -p err

# View logs with priority
sudo journalctl -u login-detect.service -p info

# Export logs to file
sudo journalctl -u login-detect.service --since "1 hour ago" > login-service.log
```

#### Service Metrics
```bash
# Show service resource usage
sudo systemctl show login-detect.service --property=MemoryCurrent,CPUUsageNSec

# Monitor service in real-time
sudo systemctl status login-detect.service --no-pager -l

# Check service restart count
sudo systemctl show login-detect.service --property=RestartCount

# Show service uptime
sudo systemctl show login-detect.service --property=ActiveEnterTimestamp
```

### Service Configuration

#### Environment Variables
```bash
# Create environment file
sudo tee /etc/default/login-detect << 'EOF'
# Login Service Environment Variables
SERVER_HOST=0.0.0.0
SERVER_PORT=7070
LOG_LEVEL=INFO
MAX_CONNECTIONS=10
TIMEOUT=30
DEBUG=false
EOF

# Update service to use environment file
sudo sed -i '/^\[Service\]/a EnvironmentFile=/etc/default/login-detect' \
    /etc/systemd/system/login-detect.service

# Reload systemd
sudo systemctl daemon-reload
```

#### Service Override
```bash
# Create service override directory
sudo mkdir -p /etc/systemd/system/login-detect.service.d/

# Create override file
sudo tee /etc/systemd/system/login-detect.service.d/override.conf << 'EOF'
[Service]
Environment=LOG_LEVEL=DEBUG
Environment=MAX_CONNECTIONS=20
RestartSec=10
EOF

# Reload systemd
sudo systemctl daemon-reload
sudo systemctl restart login-detect.service
```

### Troubleshooting Service Issues

#### Service Won't Start
```bash
# Check service status
sudo systemctl status login-detect.service

# Check detailed error messages
sudo journalctl -u login-detect.service -n 50 --no-pager

# Check service configuration
sudo systemctl cat login-detect.service

# Validate service file
sudo systemd-analyze verify /etc/systemd/system/login-detect.service

# Check for syntax errors
sudo systemd-analyze cat-config /etc/systemd/system/login-detect.service
```

#### Service Keeps Restarting
```bash
# Check restart count
sudo systemctl show login-detect.service --property=RestartCount

# View restart logs
sudo journalctl -u login-detect.service --since "5 minutes ago"

# Check service dependencies
sudo systemctl list-dependencies login-detect.service

# Test service manually
sudo -u root /opt/ntp-gps-server/login-service/login-service

# Check file permissions
ls -la /opt/ntp-gps-server/login-service/login-service
```

#### Permission Issues
```bash
# Check service user
sudo systemctl show login-detect.service --property=User

# Check file ownership
sudo ls -la /opt/ntp-gps-server/login-service/

# Fix permissions
sudo chown root:root /opt/ntp-gps-server/login-service/login-service
sudo chmod +x /opt/ntp-gps-server/login-service/login-service

# Check shadow file permissions
sudo ls -la /etc/shadow
sudo chmod 640 /etc/shadow
sudo chown root:shadow /etc/shadow
```

#### Network Issues
```bash
# Check if port is in use
sudo netstat -tlnp | grep 7070
sudo ss -tlnp | grep 7070

# Check firewall rules
sudo ufw status
sudo iptables -L | grep 7070

# Test port connectivity
telnet 127.0.0.1 7070
nc -zv 127.0.0.1 7070
```

### Service Automation Scripts

#### Complete Service Setup Script
```bash
#!/bin/bash
# setup-service.sh

SERVICE_NAME="login-detect"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
INSTALL_DIR="/opt/ntp-gps-server/login-service"
BINARY_NAME="login-service"

echo "Setting up ${SERVICE_NAME} systemd service..."

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root (use sudo)"
    exit 1
fi

# Create installation directory
sudo mkdir -p $INSTALL_DIR

# Copy binary if it exists
if [ -f "dist/$BINARY_NAME" ]; then
    sudo cp dist/$BINARY_NAME $INSTALL_DIR/
    sudo chmod +x $INSTALL_DIR/$BINARY_NAME
    sudo chown root:root $INSTALL_DIR/$BINARY_NAME
    echo "Binary copied to $INSTALL_DIR/$BINARY_NAME"
else
    echo "Warning: Binary not found, using Python script"
    sudo cp service.py $INSTALL_DIR/
    sudo chmod +x $INSTALL_DIR/service.py
    sudo chown root:root $INSTALL_DIR/service.py
fi

# Create service file
sudo tee $SERVICE_FILE << EOF
[Unit]
Description=Login Authentication Service
After=network.target
Wants=network.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/$BINARY_NAME
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal
Environment=SERVER_HOST=0.0.0.0
Environment=SERVER_PORT=7070
Environment=LOG_LEVEL=INFO
NoNewPrivileges=true
PrivateTmp=true
ReadWritePaths=/etc/shadow

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd
sudo systemctl daemon-reload

# Enable and start service
sudo systemctl enable $SERVICE_NAME.service
sudo systemctl start $SERVICE_NAME.service

# Check status
echo "Service status:"
sudo systemctl status $SERVICE_NAME.service --no-pager

echo "Service setup completed!"
echo "Use 'sudo systemctl status $SERVICE_NAME.service' to check status"
echo "Use 'sudo journalctl -u $SERVICE_NAME.service -f' to view logs"
```

#### Service Management Script
```bash
#!/bin/bash
# manage-service.sh

SERVICE_NAME="login-detect"

case "$1" in
    start)
        echo "Starting $SERVICE_NAME..."
        sudo systemctl start $SERVICE_NAME.service
        ;;
    stop)
        echo "Stopping $SERVICE_NAME..."
        sudo systemctl stop $SERVICE_NAME.service
        ;;
    restart)
        echo "Restarting $SERVICE_NAME..."
        sudo systemctl restart $SERVICE_NAME.service
        ;;
    status)
        echo "Status of $SERVICE_NAME:"
        sudo systemctl status $SERVICE_NAME.service --no-pager
        ;;
    logs)
        echo "Logs for $SERVICE_NAME:"
        sudo journalctl -u $SERVICE_NAME.service -f
        ;;
    enable)
        echo "Enabling $SERVICE_NAME..."
        sudo systemctl enable $SERVICE_NAME.service
        ;;
    disable)
        echo "Disabling $SERVICE_NAME..."
        sudo systemctl disable $SERVICE_NAME.service
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|status|logs|enable|disable}"
        exit 1
        ;;
esac
```

### Service Monitoring

#### Health Check Script
```bash
#!/bin/bash
# health-check.sh

SERVICE_NAME="login-detect"
PORT=7070

# Check if service is running
if ! systemctl is-active --quiet $SERVICE_NAME.service; then
    echo "ERROR: $SERVICE_NAME is not running"
    exit 1
fi

# Check if port is listening
if ! netstat -tlnp | grep -q ":$PORT "; then
    echo "ERROR: Port $PORT is not listening"
    exit 1
fi

# Test authentication
echo '{"login": "test", "password": "test"}' | nc -w 5 127.0.0.1 $PORT > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "OK: $SERVICE_NAME is healthy"
    exit 0
else
    echo "ERROR: $SERVICE_NAME is not responding"
    exit 1
fi
```

#### Service Monitoring with Cron
```bash
# Add to crontab for monitoring
# */5 * * * * /opt/ntp-gps-server/login-service/health-check.sh >> /var/log/login-service-health.log 2>&1
```

## Security Considerations

### File Permissions
```