#!/bin/bash
# Build script — compiles Rust engine + Qt frontend.
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

echo "=== Building Rust engine (release) ==="
FEATURES=""
if [ "$1" == "ffmpeg" ]; then
    FEATURES="--features ffmpeg"
fi
cargo build --release $FEATURES

echo ""
echo "=== Building Qt frontend ==="
cd qt
mkdir -p build
cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . --config Release -j$(nproc 2>/dev/null || sysctl -n hw.ncpu)

echo ""
echo "=== Build complete ==="
echo "Executable: qt/build/ProVideoEditor"
echo ""
echo "Note: Make sure Qt6 is installed:"
echo "  Ubuntu: sudo apt install qt6-base-dev qt6-multimedia-dev"
echo "  macOS:  brew install qt"
echo "  Windows: Install Qt6 via Qt installer or vcpkg"
