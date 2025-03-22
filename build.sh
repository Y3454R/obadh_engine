#!/bin/bash

# Exit on any error
set -e

# Clean function to remove temporary files
clean() {
    echo "Cleaning build artifacts..."
    
    # Remove target directory
    if [ -d "target" ]; then
        rm -rf target
        echo "  Removed 'target' directory"
    fi
    
    echo "Clean complete!"
    return 0
}

# Build the engine
build_engine() {
    echo "Building Obadh Engine..."
    
    # Build regular release
    echo "Building Rust code..."
    cargo build --release || return 1
    
    echo "Build successful!"
    return 0
}

# Check for command line arguments
if [ "$1" = "clean" ]; then
    clean
    exit 0
elif [ "$1" = "build" ]; then
    build_engine
    exit $?
fi

# Default behavior - display help
echo "Obadh Engine Build Tool"
echo "Usage:"
echo "  ./build.sh build           # Build the engine"
echo "  ./build.sh clean           # Clean up build artifacts"
echo ""
echo "Example:"
echo "  ./build.sh build           # Build the engine" 