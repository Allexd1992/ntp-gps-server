# NTP GPS Server Web GUI

React-based web interface for the NTP GPS Server system, providing a modern and intuitive user interface for system management and monitoring.

## Overview

The Web GUI is a React TypeScript application that provides a comprehensive web interface for managing and monitoring the NTP GPS Server system. It communicates with the Rust backend via REST API and provides real-time monitoring, configuration management, and system control capabilities.

## Architecture

### Frontend Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   React App     │    │   Redux Store   │    │   REST API      │
│   (TypeScript)  │◄──►│   (State Mgmt)  │◄──►│   (Backend)     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                        │                        │
         ▼                        ▼                        ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Components    │    │   Actions/      │    │   HTTP Client   │
│   (UI Modules)  │    │   Reducers      │    │   (Fetch API)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Component Structure

```
src/
├── components/           # UI Components
│   ├── head-panel/      # Header navigation
│   ├── button-panel/    # Control buttons
│   ├── main-window/     # Main layout
│   ├── ntp/            # NTP configuration
│   ├── gps/            # GPS monitoring
│   ├── display/        # Display settings
│   ├── rtc/            # RTC management
│   ├── network/        # Network configuration
│   ├── monitoring/     # System monitoring
│   ├── info-panel/     # System information
│   └── login-panel/    # Authentication
├── store/              # Redux state management
│   ├── settings/       # Application settings
│   ├── network/        # Network state
│   ├── monitoring/     # Monitoring data
│   ├── login/          # Authentication state
│   └── info/           # System information
└── api/                # API communication
    └── rest/           # REST API client
```

## Features

- **Real-time Monitoring**: Live system status and GPS/NTP data
- **Configuration Management**: Web-based settings for all system components
- **Network Management**: Network interface configuration
- **Authentication**: Secure login system
- **Responsive Design**: Modern Material-UI based interface
- **TypeScript**: Full type safety and better development experience
- **Redux State Management**: Centralized state management
- **REST API Integration**: Communication with Rust backend

## Technology Stack

- **React 18**: Modern React with hooks and functional components
- **TypeScript**: Type-safe JavaScript development
- **Material-UI (MUI)**: Modern UI component library
- **Redux Toolkit**: State management with RTK
- **React Redux**: React bindings for Redux
- **Fetch API**: HTTP client for API communication

## Build Instructions

### Prerequisites

#### System Requirements
- **Operating System**: Linux (Ubuntu 20.04+, Debian 11+), macOS 10.15+, or Windows 10+
- **Node.js**: Version 16.0.0 or higher (recommended: 18.x LTS)
- **npm**: Version 8.0.0 or higher (comes with Node.js)
- **Git**: For cloning the repository
- **Memory**: Minimum 2GB RAM, recommended 4GB+
- **Disk Space**: At least 1GB free space

#### Install Node.js

**Ubuntu/Debian:**
```bash
# Add NodeSource repository
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -

# Install Node.js and npm
sudo apt-get install -y nodejs

# Verify installation
node --version
npm --version
```

**CentOS/RHEL/Fedora:**
```bash
# Add NodeSource repository
curl -fsSL https://rpm.nodesource.com/setup_18.x | sudo bash -

# Install Node.js and npm
sudo yum install -y nodejs

# Verify installation
node --version
npm --version
```

**macOS:**
```bash
# Using Homebrew
brew install node

# Or download from official website
# https://nodejs.org/en/download/

# Verify installation
node --version
npm --version
```

**Windows:**
```bash
# Download and install from official website
# https://nodejs.org/en/download/

# Or using Chocolatey
choco install nodejs

# Verify installation
node --version
npm --version
```

### Build Process

#### 1. Clone Repository
```bash
# Clone the main repository
git clone https://github.com/your-repo/ntp-gps-server.git
cd ntp-gps-server/web-gui
```

#### 2. Install Dependencies
```bash
# Install all required dependencies
npm install

# Verify installation
npm list --depth=0
```

#### 3. Development Build
```bash
# Start development server
npm start

# The application will be available at http://localhost:3000
# Hot reload is enabled for development
```

#### 4. Production Build
```bash
# Create optimized production build
npm run build

# Build artifacts will be in the 'build' directory
# This includes minified JS, CSS, and optimized assets
```

#### 5. Test Build
```bash
# Run tests
npm test

# Run tests with coverage
npm test -- --coverage

# Run tests in watch mode
npm test -- --watch
```

### Build Configuration

#### Environment Variables
Create a `.env` file in the web-gui directory for custom configuration:

```bash
# Development environment
REACT_APP_SERVER_HOST=localhost
REACT_APP_SERVER_PORT=8080
REACT_APP_API_TIMEOUT=5000
REACT_APP_DEBUG_MODE=true

# Production environment
NODE_ENV=production
GENERATE_SOURCEMAP=false
```

#### TypeScript Configuration
The project uses TypeScript with strict configuration. Key settings in `tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "es5",
    "lib": ["dom", "dom.iterable", "es6"],
    "allowJs": true,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "noFallthroughCasesInSwitch": true,
    "module": "esnext",
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx"
  }
}
```

### Build Scripts

#### Available npm Scripts
```bash
# Development
npm start          # Start development server
npm run dev        # Alias for npm start

# Production
npm run build      # Create production build
npm run build:prod # Production build with optimizations

# Testing
npm test           # Run tests
npm run test:watch # Run tests in watch mode
npm run test:coverage # Run tests with coverage

# Code Quality
npm run lint       # Run ESLint
npm run lint:fix   # Fix ESLint issues
npm run format     # Format code with Prettier

# Utilities
npm run eject      # Eject from Create React App (one-way operation)
npm run analyze    # Analyze bundle size
```

#### Custom Build Scripts
```bash
# Build for specific environment
npm run build:dev    # Development build
npm run build:staging # Staging build
npm run build:prod   # Production build

# Build with specific optimizations
npm run build:analyze # Build with bundle analyzer
npm run build:stats   # Build with webpack stats
```

### Build Output

#### Development Build
- **Location**: Development server runs in memory
- **Features**: Hot reload, source maps, development tools
- **Size**: Larger due to development dependencies
- **Performance**: Optimized for development speed

#### Production Build
- **Location**: `build/` directory
- **Contents**:
  ```
  build/
  ├── static/
  │   ├── css/          # Minified CSS files
  │   ├── js/           # Minified JavaScript files
  │   └── media/        # Optimized images and assets
  ├── asset-manifest.json
  ├── favicon.ico
  ├── index.html        # Main HTML file
  ├── manifest.json     # PWA manifest
  └── robots.txt        # SEO robots file
  ```

#### Build Optimization
- **Code Splitting**: Automatic code splitting for better performance
- **Tree Shaking**: Unused code elimination
- **Minification**: JavaScript and CSS minification
- **Compression**: Gzip compression for static assets
- **Caching**: Optimized caching headers

### Build Troubleshooting

#### Common Build Issues

**1. Node.js Version Issues**
```bash
# Check Node.js version
node --version

# If version is too old, update Node.js
# Use nvm for version management
nvm install 18
nvm use 18
```

**2. Dependency Issues**
```bash
# Clear npm cache
npm cache clean --force

# Remove node_modules and reinstall
rm -rf node_modules package-lock.json
npm install
```

**3. TypeScript Errors**
```bash
# Check TypeScript configuration
npx tsc --noEmit

# Fix type issues
npm run lint:fix
```

**4. Memory Issues**
```bash
# Increase Node.js memory limit
export NODE_OPTIONS="--max-old-space-size=4096"
npm run build
```

**5. Build Performance**
```bash
# Use faster package manager
npm install -g pnpm
pnpm install
pnpm run build
```

#### Build Verification
```bash
# Verify build integrity
npm run build
ls -la build/

# Test production build locally
npx serve -s build -l 3000

# Check bundle size
npm run analyze
```

### Continuous Integration

#### GitHub Actions Example
```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
    
    - name: Install dependencies
      run: npm ci
    
    - name: Run tests
      run: npm test -- --coverage --watchAll=false
    
    - name: Build application
      run: npm run build
    
    - name: Upload build artifacts
      uses: actions/upload-artifact@v3
      with:
        name: build-files
        path: build/
```

### Deployment Builds

#### Docker Build
```bash
# Build Docker image
docker build -t ntp-gps-web-gui .

# Run container
docker run -p 80:80 ntp-gps-web-gui
```

#### Static Server Build
```bash
# Build for static server
npm run build

# Copy to web server
sudo cp -r build/* /var/www/html/
sudo chown -R www-data:www-data /var/www/html/
```

#### CDN Build
```bash
# Build with CDN optimization
REACT_APP_CDN_URL=https://cdn.example.com npm run build
```

## Installation

### Prerequisites

```bash
# Install Node.js (v16 or higher)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Verify installation
node --version
npm --version
```

### Development Setup

```bash
# Navigate to web-gui directory
cd ntp-gps-server/web-gui

# Install dependencies
npm install

# Start development server
npm start
```

The application will be available at `http://localhost:3000`

### Production Build

```bash
# Build for production
npm run build

# The build folder will contain optimized static files
# ready for deployment to a web server
```

## Configuration

### Environment Variables

The web GUI reads configuration from a dynamically generated `config.js` file:

```javascript
// Generated by backend at runtime
window.REACT_APP_SERVER_HOST = '192.168.1.100';
window.REACT_APP_SERVER_PORT = 8080;
```

### API Endpoints

The web GUI communicates with the following backend endpoints:

- `GET /api/v1/settings` - Get system settings
- `POST /api/v1/settings` - Update system settings
- `GET /api/v1/network` - Get network configuration
- `POST /api/v1/network` - Update network configuration
- `GET /api/v1/status` - Get system monitoring data
- `GET /api/v1/system` - Get system information
- `POST /api/v1/login` - Authenticate user

## Development

### Project Structure

```
web-gui/
├── public/              # Static assets
│   ├── index.html      # Main HTML template
│   └── favicon.ico     # Application icon
├── src/                # Source code
│   ├── components/     # React components
│   ├── store/          # Redux store
│   ├── api/            # API client
│   ├── App.tsx         # Main application component
│   └── index.tsx       # Application entry point
├── package.json        # Dependencies and scripts
├── tsconfig.json       # TypeScript configuration
└── README.md          # This file
```

### Available Scripts

```bash
# Start development server
npm start

# Build for production
npm run build

# Run tests
npm test

# Eject from Create React App
npm run eject
```

### Adding New Components

1. Create component directory in `src/components/`
2. Create component file with TypeScript interface
3. Add to Redux store if state management is needed
4. Import and use in main application

### State Management

The application uses Redux Toolkit for state management:

```typescript
// Example store slice
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface NetworkState {
  address: string;
  gateway: string;
  dns: string;
}

const networkSlice = createSlice({
  name: 'network',
  initialState: { address: '', gateway: '', dns: '' },
  reducers: {
    setAddress: (state, action: PayloadAction<string>) => {
      state.address = action.payload;
    },
  },
});
```

## Deployment

### Static File Deployment

```bash
# Build the application
npm run build

# Copy build folder to web server
sudo cp -r build/* /var/www/html/

# Set permissions
sudo chown -R www-data:www-data /var/www/html/
```

### Nginx Configuration

```nginx
server {
    listen 80;
    server_name your-domain.com;
    root /var/www/html;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location /api/ {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### Docker Deployment

```dockerfile
FROM node:18-alpine as build
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=build /app/build /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

## API Integration

### REST API Client

The web GUI uses a custom REST API client for backend communication:

```typescript
// Example API call
export const getSettings = async (callback: callbackType): Promise<void> => {
    fetch(uri + api.GET.settings.url, {
        mode: "cors",
    })
        .then(response => response.json())
        .then(data => {
            if (callback) callback(data as types.ISettings);
        })
        .catch(error => console.error(error));
}
```

### Error Handling

The application includes comprehensive error handling:

- Network request failures
- Authentication errors
- Invalid data responses
- Connection timeouts

## Security

### Authentication

- Login form with username/password
- Session management
- Secure API communication
- CORS configuration

### Input Validation

- Client-side form validation
- TypeScript type checking
- API response validation

## Monitoring and Debugging

### Development Tools

- React Developer Tools
- Redux DevTools
- Browser Developer Tools
- Network tab for API debugging

### Logging

```typescript
// Console logging for debugging
console.log("Request GET settings:", data);
console.error("API Error:", error);
```

## Troubleshooting

### Common Issues

#### Build Errors
```bash
# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install
```

#### API Connection Issues
- Check backend server is running
- Verify API endpoint configuration
- Check CORS settings
- Review network connectivity

#### TypeScript Errors
```bash
# Check TypeScript configuration
npx tsc --noEmit

# Fix type issues
npm run build
```

## Contributing

1. Follow TypeScript best practices
2. Use functional components with hooks
3. Implement proper error handling
4. Add TypeScript interfaces for all data structures
5. Follow Material-UI design patterns
6. Test components thoroughly

## License

This project is part of the NTP GPS Server system. See main project license for details.

## Support

For issues and questions:
- Check troubleshooting section above
- Review browser console for errors
- Check network tab for API issues
- Open issue in project repository
- Contact system administrator
