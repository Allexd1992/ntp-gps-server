# NTP GPS Server

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Node.js](https://img.shields.io/badge/node.js-18+-green.svg)](https://nodejs.org)
[![Python](https://img.shields.io/badge/python-3.7+-yellow.svg)](https://www.python.org)

A multi-service NTP GPS server system with Rust backend, React web interface, and Python TCP services for embedded Linux systems.

## Overview

The NTP GPS Server is a distributed system designed for precise time synchronization and GPS data processing. It provides a complete solution for time server deployment on single-board computers like Orange Pi and Raspberry Pi.

## Table of Contents

### Main Documentation
- [Architecture](#architecture) - System architecture and component overview
- [Hardware Requirements](#hardware-requirements) - Required hardware and pin connections
- [Sequence Diagrams](#sequence-diagrams) - System interaction flows
- [Frontend Build and Integration](#frontend-build-and-integration) - Build and deploy web interface
- [Service Links](#service-links) - Links to individual service documentation
- [General Purpose](#general-purpose) - System capabilities and use cases

### Service Documentation
- **[Backend Service](backend/README.md)** - Rust NTP server and REST API implementation
- **[Web GUI Service](web-gui/README.md)** - React web interface for system management
- **[OLED Connector Service](oled-connector/README.md)** - OLED display service via TCP socket
- **[Network Service](network-service/README.md)** - Network configuration management service
- **[RTC Control Service](rtc-control-service/README.md)** - Hardware clock (RTC) management service
- **[Login Service](login-service/README.md)** - User authentication service

### Quick Links
- [Port-Services Mapping](#port-services-mapping) - Service port assignments
- [Hardware Setup](#hardware-requirements) - Hardware requirements and connections
- [Build Instructions](#frontend-build-and-integration) - Frontend build and backend integration

## Architecture

The project consists of several services for working with NTP, GPS, RTC, network settings and information display:

- **backend/** — Rust server-side. Implements NTP server, GPS work, RTC, settings storage, REST API for web interface.
- **web-gui/** — React+Redux web interface for server management and monitoring.
- **oled-connector/** and **oled_i2c_connector/** — Python services for outputting information to OLED display via TCP socket.
- **network-service/** — Python service for managing network settings via socket.
- **rtc-control-service/** — Python service for managing hardware clock (RTC) via socket.
- **login-service/** — Python service for user authentication via socket.

```
┌──────────────────────┐
│     Backend Server   │
│     (Rust)           │
│   ┌─────────────┐    │
│   │   NTP       │    │
│   │  Server     │    │
│   └─────────────┘    │
│   ┌─────────────┐    │
│   │   REST      │    │
│   │   API       │    │
│   └─────────────┘    │
└──────────┬───────────┘
           │
           ├──────────────┐
           │              ▼
           │    ┌─────────────────┐
           │    │   Web GUI       │
           │    │   (React)       │
           │    │   Port: 8080    │
           │    └─────────────────┘
           │
           ├──────────────┐
           │              ▼
           │    ┌─────────────────┐
           │    │   OLED Service  │
           │    │   Port: 5050    │
           │    └─────────┬───────┘
           │              ▼
           │    ┌─────────────────┐
           │    │   OLED Display  │
           │    │   (I2C)         │
           │    └─────────────────┘
           │
           ├──────────────┐
           │              ▼
           │    ┌─────────────────┐
           │    │ Network Service │
           │    │ Port: 7575      │
           │    └─────────┬───────┘
           │              ▼
           │    ┌─────────────────┐
           │    │   Network       │
           │    │   Interfaces    │
           │    └─────────────────┘
           │
           ├──────────────┐
           │              ▼
           │    ┌─────────────────┐
           │    │ RTC Control     │
           │    │ Service         │
           │    │ Port: 6060      │
           │    └─────────┬───────┘
           │              ▼
           │    ┌─────────────────┐
           │    │   External RTC  │
           │    │   (Hardware)    │
           │    └─────────────────┘
           │
           ├──────────────┐
           │              ▼
           │    ┌─────────────────┐
           │    │ Login Service   │
           │    │ Port: 7070      │
           │    └─────────┬───────┘
           │              ▼
           │    ┌─────────────────┐
           │    │   /etc/shadow   │
           │    │   (Auth)        │
           │    └─────────────────┘
           │
           └──────────────┐
                          ▼
                ┌─────────────────┐
                │   GPS Module    │
                │   (Hardware)    │
                │   GNSS          │
                └─────────────────┘
```
### Core Components

#### 1. Backend Service (Rust)
- **Purpose**: Central orchestrator and NTP server
- **Ports**: 123 (NTP), 8080 (REST API)
- **Responsibilities**:
  - NTP server implementation
  - GPS data processing and parsing
  - Configuration storage and management
  - REST API for web interface
  - Inter-service communication coordination
  - System monitoring and logging

#### 2. Web GUI (React + Redux)
- **Purpose**: User interface for system management
- **Port**: 8080 (served by backend)
- **Features**:
  - Real-time system monitoring
  - Configuration management
  - GPS data visualization
  - Network settings interface
  - User authentication interface
  - System status dashboard

#### 3. Python Microservices

##### OLED Connector Service
- **Port**: 5050
- **Purpose**: Display information on OLED screen
- **Protocol**: TCP socket with JSON messages
- **Hardware**: I2C OLED display
- **Features**:
  - Real-time time display
  - GPS status information
  - NTP server status
  - System notifications

##### Network Service
- **Port**: 7575
- **Purpose**: Network configuration management
- **Protocol**: TCP socket with JSON messages
- **Tools**: nmcli (NetworkManager CLI)
- **Features**:
  - Network interface configuration
  - IP address management
  - Gateway and DNS settings
  - Network status monitoring
  - WiFi/Ethernet configuration

##### RTC Control Service
- **Port**: 6060
- **Purpose**: External hardware clock management
- **Protocol**: TCP socket with JSON messages
- **Tools**: hwclock, date commands
- **Features**:
  - External RTC time reading
  - RTC time synchronization
  - System clock management
  - Time drift monitoring
  - Backup time source

##### Login Service
- **Port**: 7070
- **Purpose**: User authentication and access control
- **Protocol**: TCP socket with JSON messages
- **Security**: /etc/shadow validation
- **Features**:
  - User authentication
  - Password validation
  - Access control
  - Session management
  - Security logging

## Hardware Requirements

### Required Hardware
- **Single-board computer**: Orange Pi, Raspberry Pi, or compatible ARM-based system
- **GPS module**: UART-based GPS receiver (NEO-6M, NEO-8M, etc.)
- **OLED display**: I2C OLED display (SSD1306, SH1106, etc.)
- **RTC module**: Hardware real-time clock (DS3231, PCF8563, etc.)
- **Network**: Ethernet or WiFi connectivity

### Optional Hardware
- **Power supply**: Stable 5V power supply
- **Case**: Protective enclosure
- **Antenna**: External GPS antenna for better signal
- **Cooling**: Heat sink or fan for high-performance boards

### Pin Connections

#### GPS Module
- **VCC**: 3.3V or 5V (depending on module)
- **GND**: Ground
- **TX**: UART RX pin (GPIO 15 on Raspberry Pi)
- **RX**: UART TX pin (GPIO 14 on Raspberry Pi)

#### OLED Display
- **VCC**: 3.3V
- **GND**: Ground
- **SCL**: I2C SCL pin (GPIO 3 on Raspberry Pi)
- **SDA**: I2C SDA pin (GPIO 2 on Raspberry Pi)

#### RTC Module
- **VCC**: 3.3V
- **GND**: Ground
- **SCL**: I2C SCL pin (shared with OLED)
- **SDA**: I2C SDA pin (shared with OLED)

## Sequence Diagrams

### Time Synchronization Sequence
```
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│  GPS    │    │ Backend │    │ RTC     │    │ System  │    │ OLED    │
│ Module  │    │ Service │    │ Service │    │ Clock   │    │ Display │
└────┬────┘    └────┬────┘    └────┬────┘    └────┬────┘    └────┬────┘
     │              │              │              │              │
     │ GPS Data     │              │              │              │
     │─────────────►│              │              │              │
     │              │ Parse GPS    │              │              │
     │              │─────────────►│              │              │
     │              │              │ Set RTC Time │              │
     │              │              │─────────────►│              │
     │              │              │              │ Update Clock │
     │              │              │              │─────────────►│
     │              │              │              │              │
     │              │ Display Data │              │              │
     │              │─────────────►│              │              │
     │              │              │              │              │
```

### Network Configuration Sequence
```
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│ Web GUI │    │ Backend │    │Network  │    │ nmcli   │
│         │    │ Service │    │Service  │    │         │
└────┬────┘    └────┬────┘    └────┬────┘    └────┬────┘
     │              │              │              │
     │Config Request│              │              │
     │─────────────►│              │              │
     │              │ Network Get  │              │
     │              │─────────────►│              │
     │              │              │ nmcli show   │
     │              │              │─────────────►│
     │              │              │ Network Info │
     │              │              │◄─────────────│
     │              │ Network Data │              │
     │              │◄─────────────│              │
     │ Config Data  │              │              │
     │◄─────────────│              │              │
```

### Authentication Sequence
```
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│ Web GUI │    │ Backend │    │ Login   │    │/etc/    │
│         │    │ Service │    │Service  │    │shadow   │
└────┬────┘    └────┬────┘    └────┬────┘    └────┬────┘
     │              │              │              │
     │ Login Request│              │              │
     │─────────────►│              │              │
     │              │ Auth Request │              │
     │              │─────────────►│              │
     │              │              │ Validate     │
     │              │              │─────────────►│
     │              │              │ Auth Result  │
     │              │              │◄─────────────│
     │              │ Auth Response│              │
     │              │◄─────────────│              │
     │ Auth Result  │              │              │
     │◄─────────────│              │              │
```

## Frontend Build and Integration

### Build Frontend
```bash
# Navigate to web-gui directory
cd ntp-gps-server/web-gui

# Install dependencies
npm install

# Build for production
npm run build
```

### Integrate with Backend
```bash
# Copy build artifacts to backend static directory
sudo mkdir -p /opt/ntp-gps-server/backend/static
sudo cp -r build/* /opt/ntp-gps-server/backend/static/

# Set proper permissions
sudo chown -R root:root /opt/ntp-gps-server/backend/static
sudo chmod -R 644 /opt/ntp-gps-server/backend/static/*
```



## Service Links

### Service Documentation
- **[Backend Service](backend/README.md)** - Rust NTP server and REST API
- **[Web GUI Service](web-gui/README.md)** - React web interface
- **[OLED Connector Service](oled-connector/README.md)** - OLED display service
- **[Network Service](network-service/README.md)** - Network configuration service
- **[RTC Control Service](rtc-control-service/README.md)** - Hardware clock service
- **[Login Service](login-service/README.md)** - Authentication service

### Port-Services Mapping

| Port | Service | Description |
|------|---------|-------------|
| 123  | NTP Server | NTP server (backend) |
| 8080 | Web GUI | Web interface (backend) |
| 5050 | OLED Connector | Output to OLED display |
| 6060 | RTC Control | Hardware clock management |
| 7070 | Login Service | User authentication |
| 7575 | Network Service | Network settings management |

## General Purpose

The NTP GPS Server system provides:

- **Precise Time Synchronization**: NTP server with GPS time source
- **Hardware Integration**: OLED display, RTC module, GPS receiver
- **Network Management**: Web-based network configuration
- **System Monitoring**: Real-time status monitoring and logging
- **User Authentication**: Secure access control
- **Embedded Optimization**: Designed for single-board computers
- **Production Ready**: Systemd services, logging, error handling

### Use Cases
- **Time Server**: Primary or secondary NTP server
- **GPS Time Source**: Precise time from GPS satellites
- **Network Infrastructure**: Time synchronization for local networks
- **IoT Applications**: Time reference for IoT devices
- **Embedded Systems**: Time server for embedded Linux systems