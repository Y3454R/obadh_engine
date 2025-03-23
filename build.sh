#!/bin/bash

# Exit on any error
set -e

# Colors for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Show an error message and exit
error() {
    echo -e "${RED}ERROR: $1${NC}" >&2
    exit 1
}

# Show a success message
success() {
    echo -e "${GREEN}SUCCESS: $1${NC}"
}

# Show an info message
info() {
    echo -e "${BLUE}INFO: $1${NC}"
}

# Show a warning message
warning() {
    echo -e "${YELLOW}WARNING: $1${NC}"
}

# Check for required tools
check_requirements() {
    info "Checking requirements..."
    
    # Check for Rust
    if ! command -v rustc &> /dev/null; then
        error "Rust is not installed. Please install it from https://rustup.rs/"
    fi
    
    # Check for wasm-pack
    if ! command -v wasm-pack &> /dev/null; then
        info "wasm-pack is not installed. Installing..."
        cargo install wasm-pack || error "Failed to install wasm-pack"
    fi
    
    # Check for npm
    if ! command -v npm &> /dev/null; then
        error "Node.js/npm is not installed. Please install it from https://nodejs.org/"
    fi
    
    # Change to www directory and install npm dependencies if needed
    if [ -d "www" ] && [ -f "www/package.json" ]; then
        cd www
        if [ ! -d "node_modules" ]; then
            info "Installing npm dependencies..."
            npm install || error "Failed to install npm dependencies"
        fi
        cd ..
    fi
    
    success "All requirements satisfied"
}

# Clean function to remove temporary files
clean() {
    info "Cleaning build artifacts..."
    
    # Remove target directory
    if [ -d "target" ]; then
        rm -rf target
        info "Removed 'target' directory"
    fi
    
    # Remove wasm-pack generated files
    if [ -d "pkg" ]; then
        rm -rf pkg
        info "Removed 'pkg' directory"
    fi
    
    success "Clean complete!"
    return 0
}

# Build the WASM package and setup web files
build_wasm() {
    info "Building WASM Package..."
    
    # Check for www directory
    if [ ! -d "www" ]; then
        error "www directory does not exist. Please create it first."
    fi
    
    # Run the npm script to build wasm
    cd www
    npm run build-wasm || error "Failed to build WASM package"
    cd ..
    
    success "WASM build complete!"
    return 0
}

# Build Tailwind CSS
build_css() {
    info "Building CSS..."
    
    cd www
    npm run build || error "Failed to build CSS"
    cd ..
    
    success "CSS build complete!"
    return 0
}

# Build for distribution (GitHub Pages)
build_dist() {
    info "Building project for distribution (GitHub Pages)..."
    
    # First build the WASM package
    info "Building WebAssembly package..."
    wasm-pack build --target web --out-dir pkg || error "Failed to build WebAssembly package"
    
    # Change to www directory
    cd www || error "Failed to change to www directory"
    
    # Copy WASM files
    info "Copying WebAssembly files..."
    mkdir -p js
    cp ../pkg/*.js js/ || error "Failed to copy JS files"
    cp ../pkg/*.wasm js/ || error "Failed to copy WASM files"
    
    # Fix import paths in JS files
    info "Fixing module paths in JavaScript files..."
    for jsfile in js/*.js; do
        sed -i.bak 's|import.meta.url, \"../pkg/|import.meta.url, \"|g' "$jsfile" && rm -f "$jsfile.bak"
    done
    
    # Build CSS
    info "Building CSS..."
    npm run build || error "Failed to build CSS"
    
    # Return to project root
    cd ..
    
    # Success message
    success "Distribution build complete! Files are ready in the www directory."
    info "You can now commit the following files to your repository for GitHub Pages:"
    info "  - www/index.html"
    info "  - www/css/styles.css"
    info "  - www/js/*.js"
    info "  - www/js/*.wasm"
    info "  - docs/index.html (redirects to www/index.html)"
    return 0
}

# Serve the web application
serve() {
    info "Starting development server..."
    
    # Change to www directory
    cd www
    
    # Setup signal handling
    # This is a cleaner approach than using background processes with trap
    exec npm run serve
}

# Development mode with watch
dev() {
    info "Starting development environment with watch..."
    
    # Change to www directory
    cd www
    
    # Run npm dev command that handles CSS watch and server
    exec npm run dev
}

# Build everything and start the server
start() {
    info "Building and starting the server..."
    
    cd www
    npm run build-wasm && npm run build && npm run serve
    cd ..
}

# Display the help information
show_help() {
    echo "Obadh Engine Build Tool"
    echo "======================="
    echo "Usage:"
    echo "  ./build.sh wasm     # Build the WASM package"
    echo "  ./build.sh css      # Build Tailwind CSS"
    echo "  ./build.sh serve    # Start the development server only"
    echo "  ./build.sh dev      # Start development mode with file watching"
    echo "  ./build.sh start    # Build everything and start the server"
    echo "  ./build.sh dist     # Build for distribution (GitHub Pages)"
    echo "  ./build.sh clean    # Clean up build artifacts"
    echo ""
    echo "Note: Using 'dev' or 'serve' is the recommended way for development."
    echo "      Use 'dist' to prepare files for GitHub Pages deployment."
}

# Main execution
case "$1" in
    "clean")
        clean
        ;;
    "wasm")
        check_requirements
        build_wasm
        ;;
    "css")
        check_requirements
        build_css
        ;;
    "serve")
        check_requirements
        serve
        ;;
    "dev")
        check_requirements
        dev
        ;;
    "dist")
        check_requirements
        build_dist
        ;;
    "start")
        check_requirements
        clean && build_wasm && build_css && serve
        ;;
    *)
        show_help
        ;;
esac

exit $? 