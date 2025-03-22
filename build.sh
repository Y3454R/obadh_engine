#!/bin/bash

# Exit on any error
set -e

# Default port
PORT=8000

# Function to check if www directory is valid
check_www_directory() {
    if [ ! -d "www" ]; then
        echo "Error: 'www' directory not found!"
        return 1
    fi
    
    if [ ! -f "www/index.html" ]; then
        echo "Error: 'www/index.html' not found!"
        return 1
    fi
    
    return 0
}

# Clean function to remove temporary files
clean() {
    echo "Cleaning build artifacts..."
    
    # Remove target directory
    if [ -d "target" ]; then
        rm -rf target
        echo "  Removed 'target' directory"
    fi
    
    # Remove WebAssembly packages
    if [ -d "pkg" ]; then
        rm -rf pkg
        echo "  Removed 'pkg' directory"
    fi
    
    if [ -d "www/pkg" ]; then
        rm -rf www/pkg
        echo "  Removed 'www/pkg' directory"
    fi
    
    echo "Clean complete!"
    return 0
}

# Check for command line arguments
if [ "$1" = "serve" ]; then
    # Check if a port is specified
    if [ -n "$2" ] && [[ "$2" =~ ^[0-9]+$ ]]; then
        PORT=$2
    fi
    
    echo "Building Obadh Engine and starting web server..."
    # Build the engine with WebAssembly target
    wasm-pack build --target web --out-dir www/pkg
    
    # Verify www directory exists
    if ! check_www_directory; then
        echo "Cannot start server. Please ensure the www directory is properly set up."
        exit 1
    fi
    
    # Change to www directory and start server
    cd www
    echo "Starting web server at http://localhost:$PORT"
    
    # Check which Python command is available
    if command -v python3 &> /dev/null; then
        python3 -m http.server $PORT
    elif command -v python &> /dev/null; then
        python -m http.server $PORT
    else
        echo "Error: Neither python3 nor python is installed. Please install Python to run the server."
        exit 1
    fi
    
    exit 0
elif [ "$1" = "clean" ]; then
    clean
    exit 0
fi

echo "Building Obadh Engine..."

# Build the engine with WebAssembly target, outputting directly to www/pkg
wasm-pack build --target web --out-dir www/pkg

echo "Build complete! The WebAssembly files are available in www/pkg/"
echo ""
echo "To run the demo, use:"
echo "  ./build.sh serve           # Uses default port 8000"
echo "  ./build.sh serve PORT      # Specify a custom port number"
echo ""
echo "Other commands:"
echo "  ./build.sh clean           # Clean up build artifacts"
echo ""
echo "Or manually start a web server:"
echo "  cd www"
echo "  python3 -m http.server PORT   # or 'python -m http.server PORT' on some systems"
echo "Then open http://localhost:PORT in your browser." 