#!/bin/bash

echo "🚀 Starting Mancala auto-build watcher..."
echo "📁 Watching for changes in Rust source files..."
echo "💡 Press Ctrl+C to stop"
echo ""

# Function to build WASM
build_wasm() {
    echo "🔄 Change detected! Building WASM..."
    wasm-pack build --target web --out-dir pkg
    if [ $? -eq 0 ]; then
        echo "✅ Build successful! $(date)"
        echo "🎯 Your game is ready to test"
    else
        echo "❌ Build failed! Check the errors above."
    fi
    echo ""
}

# Initial build
build_wasm

# Watch for changes in Rust files
if command -v inotifywait &> /dev/null; then
    # Linux - use inotify
    while inotifywait -r -e modify,create,delete src/ Cargo.toml 2>/dev/null; do
        build_wasm
    done
elif command -v fswatch &> /dev/null; then
    # macOS - use fswatch  
    fswatch -o src/ Cargo.toml | while read; do
        build_wasm
    done
else
    echo "⚠️  File watching not available (install inotify-tools or fswatch)"
    echo "🔧 Falling back to polling every 3 seconds..."
    
    # Fallback - check modification times
    last_mod=$(find src/ Cargo.toml -type f -exec stat -c %Y {} \; 2>/dev/null | sort -n | tail -1)
    
    while true; do
        sleep 3
        current_mod=$(find src/ Cargo.toml -type f -exec stat -c %Y {} \; 2>/dev/null | sort -n | tail -1)
        if [ "$current_mod" != "$last_mod" ]; then
            build_wasm
            last_mod=$current_mod
        fi
    done
fi