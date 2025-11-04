# BumbleBees - WASM/Web Version

## 🎮 Play in Your Browser!

BumbleBees is fully compatible with WebAssembly and can run directly in modern web browsers.

## ✅ WASM Compatibility Status

**Status: FULLY COMPATIBLE** ✨

- ✅ **Core Game Loop**: Async game loop with `wasm-bindgen-futures`
- ✅ **Cross-Platform Storage**: LocalStorage (WASM) / File I/O (Desktop)
- ✅ **Conditional Compilation**: Platform-specific code using `#[cfg(target_arch = "wasm32")]`
- ✅ **Audio Support**: Ready for Web Audio API via macroquad
- ✅ **Rendering**: WebGL-based rendering via macroquad
- ✅ **Input Handling**: Mouse and keyboard events work in browser

## 🏗️ Building for WASM

### Quick Build

```bash
./build-wasm.sh
```

### Manual Build

1. **Install WASM target** (one-time setup):
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **Install wasm-bindgen-cli** (one-time setup):
   ```bash
   cargo install wasm-bindgen-cli --version 0.2.105
   ```

3. **Build the WASM binary**:
   ```bash
   cargo build --target wasm32-unknown-unknown --lib --release
   ```

4. **Generate JavaScript bindings**:
   ```bash
   wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/ten.wasm
   ```

## 🚀 Running Locally

1. **Start a local web server**:
   ```bash
   python3 -m http.server 8000
   ```

2. **Open in browser**:
   ```
   http://localhost:8000
   ```

3. The game will automatically load from `index.html`

## 📦 Deployment

### Static Hosting

Deploy to any static hosting service (GitHub Pages, Netlify, Vercel, etc.):

1. Build the WASM version (see above)
2. Upload these files to your host:
   - `index.html`
   - `pkg/ten.js`
   - `pkg/ten_bg.wasm`
   - `pkg/ten.d.ts`
   - `resources/` (game assets)

### Required Files Structure

```
your-site/
├── index.html
├── pkg/
│   ├── ten.js
│   ├── ten_bg.wasm
│   └── ten.d.ts
└── resources/
    ├── *.png (textures)
    └── *.wav (audio files)
```

## 🔧 Technical Details

### Platform-Specific Code

The game uses conditional compilation for platform-specific features:

```rust
#[cfg(target_arch = "wasm32")]
fn load_from_localstorage() { /* WASM implementation */ }

#[cfg(not(target_arch = "wasm32"))]
fn load_from_file() { /* Desktop implementation */ }
```

### Highscore Storage

- **Desktop**: Saves to `highscores.txt` file
- **WASM**: Saves to browser's LocalStorage using `web-sys`

### Audio

The game uses macroquad's audio system which automatically handles:
- Native audio on desktop
- Web Audio API in browsers

### Entry Points

- **Desktop**: `#[macroquad::main]` in `src/main.rs:939`
- **WASM**: `#[wasm_bindgen(start)]` in `src/main.rs:964`

## 🐛 Troubleshooting

### CORS Issues

If running locally with `file://` protocol, you may encounter CORS errors. Always use a web server (like `python3 -m http.server`).

### Audio Not Playing

Browsers require user interaction before playing audio. Click on the game canvas after loading.

### WASM File Not Found

Ensure the `pkg/` directory is in the same location as `index.html`, or update the import path in `index.html`.

### Large WASM Size

For production, consider:
- Using `wasm-opt` to optimize the WASM binary
- Enabling `opt-level = "z"` in Cargo.toml for smaller builds
- Using gzip/brotli compression on your web server

## 📊 Build Sizes

- **WASM Binary**: ~350KB (release build)
- **JavaScript Glue**: ~3.5KB
- **TypeScript Definitions**: ~1.2KB
- **Total Web Bundle**: ~354KB + assets

## 🎯 Browser Support

Tested and working on:
- ✅ Chrome/Edge (latest)
- ✅ Firefox (latest)
- ✅ Safari (latest)
- ✅ Opera (latest)

Requires:
- WebAssembly support
- WebGL support
- ES6 Modules

## 📝 Development Notes

### Hot Reload

For development with hot reload, consider using `microserver` or `live-server`:

```bash
npm install -g live-server
live-server --port=8000
```

### Debug Builds

For faster iteration during development:

```bash
cargo build --target wasm32-unknown-unknown --lib  # Remove --release
wasm-bindgen --target web --out-dir pkg --debug target/wasm32-unknown-unknown/debug/ten.wasm
```

### Logging

The game uses `log` crate. In WASM, logs appear in browser console.

## 🔗 Related Files

- `index.html` - Main HTML wrapper for WASM
- `wasm-status.html` - Build status page
- `build-wasm.sh` - Automated build script
- `Cargo.toml` - WASM dependencies configured
- `src/main.rs` - WASM entry point at line 964

## 🎮 Controls (WASM Version)

Same as desktop:
- **Arrow Keys**: Move left/right
- **Space**: Shoot
- **Mouse**: Click to start game
- **R**: Return to menu (on game over)

## 🚧 Known Limitations

- No native filesystem access (uses LocalStorage instead)
- Audio may need user interaction to start
- Performance depends on browser WebGL implementation

## 📚 Resources

- [macroquad WASM docs](https://macroquad.rs/articles/wasm/)
- [wasm-bindgen book](https://rustwasm.github.io/wasm-bindgen/)
- [MDN WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)

---

Built with ❤️ using Rust, macroquad, and WebAssembly
