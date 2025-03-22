#!/bin/bash

# Exit on any error
set -e

# Check for command line arguments
if [ "$1" = "serve" ]; then
    echo "Building Obadh Engine and starting web server..."
    # Build the engine with WebAssembly target
    wasm-pack build --target web --out-dir www/pkg
    # Change to www directory and start server
    cd www
    echo "Starting web server at http://localhost:8000"
    python -m http.server
    exit 0
fi

echo "Building Obadh Engine..."

# Build the engine with WebAssembly target, outputting directly to www/pkg
wasm-pack build --target web --out-dir www/pkg

echo "Build complete! The WebAssembly files are available in www/pkg/"
echo ""
echo "To run the demo, use:"
echo "  ./build.sh serve"
echo ""
echo "Or manually start a web server:"
echo "  cd www"
echo "  python -m http.server"
echo "Then open http://localhost:8000 in your browser." 