# Auto-Build Setup for Mancala Development

## Quick Start

### Option 1: Using NPM Scripts (Recommended)
```bash
# Build once
npm run build

# Auto-rebuild on changes (recommended for development)
npm run dev

# Start local server
npm run serve
```

### Option 2: Direct Script
```bash
# Make executable (first time only)
chmod +x watch-build.sh

# Start auto-rebuild watcher
./watch-build.sh
```

## What the Auto-Builder Does

🔍 **Watches for changes** in:
- All Rust source files (`src/**/*.rs`)
- Cargo.toml configuration
- Any new files you create

⚡ **Automatically rebuilds** WASM when changes are detected

✅ **Shows build status** with clear success/error messages

## Development Workflow

1. Start the auto-builder: `npm run dev`
2. Make changes to your Rust code
3. Save the file
4. The WASM automatically rebuilds
5. Refresh your browser to see changes

## File Change Detection

The script uses the best available file watcher for your system:
- **Linux**: `inotifywait` (install with `sudo apt install inotify-tools`)
- **macOS**: `fswatch` (install with `brew install fswatch`) 
- **Fallback**: Polling every 3 seconds (works everywhere)

## Notes

- ✅ **CSS changes** don't need rebuilding - just refresh the browser
- 🔄 **Rust changes** trigger automatic WASM rebuild
- 📁 **Static files** (images, audio) don't need rebuilding
- ⏱️ Typical rebuild takes 15-20 seconds

Happy coding! 🎮✨