# Section 11: WebAssembly with Rust

## Overview

WebAssembly (WASM) is a binary instruction format that runs in web browsers at near-native speed. Rust is one of the best languages for WebAssembly development, offering memory safety, small binary sizes, and excellent tooling.

## Why Rust for WebAssembly?

✅ **Memory Safety** - No undefined behavior in WASM modules <br />
✅ **Small Binaries** - Rust compiles to compact WASM files (50-200KB typical) <br />
✅ **Zero-cost Abstractions** - High-level code with low-level performance <br />
✅ **Mature Ecosystem** - `wasm-bindgen`, `wasm-pack`, `web-sys` <br />
✅ **No Garbage Collector** - Predictable performance

## What You'll Learn

1. **WASM Basics** - Compile Rust to WebAssembly
2. **wasm-bindgen** - JavaScript interop
3. **web-sys** - Access browser APIs
4. **wasm-pack** - Build and publish WASM packages
5. **Performance** - Optimize WASM binary size
6. **Integration** - Use WASM in web applications

## Section Contents

### 01-hello-wasm/
Your first WebAssembly module - "Hello World" in the browser

### 02-dom-manipulation/
Manipulate the DOM from Rust using web-sys

### 03-canvas-graphics/
Draw graphics on HTML canvas using Rust

### 04-game-of-life/
Interactive Conway's Game of Life (classic WASM example)

### 05-data-processing/
High-performance data processing in the browser

### 06-npm-package/
Build and publish a Rust WASM package to npm

## Prerequisites

- Rust installed (rustup)
- Node.js and npm
- Basic JavaScript knowledge
- Understanding of ownership and borrowing

## Tools Required

```bash
# Install wasm-pack
cargo install wasm-pack

# Install npm dependencies (for examples)
npm install

# Serve examples locally
python3 -m http.server 8080
```

## Project Structure

```
11-webassembly/
├── README.md
├── 01-hello-wasm/
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   ├── index.html
│   └── README.md
├── 02-dom-manipulation/
├── 03-canvas-graphics/
├── 04-game-of-life/
├── 05-data-processing/
└── 06-npm-package/
```

## Key Concepts

### WASM Build Target

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

### JavaScript Interop

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### Browser APIs

```rust
use web_sys::console;

#[wasm_bindgen]
pub fn log_message() {
    console::log_1(&"Hello from Rust!".into());
}
```

## Performance Tips

1. **Minimize Size**
   - Use `wee_alloc` for smaller allocator
   - Enable `opt-level = "z"` for size optimization
   - Use `wasm-opt` for further optimization

2. **Reduce Startup Time**
   - Lazy-load WASM modules
   - Split large modules
   - Use streaming compilation

3. **Optimize Hot Paths**
   - Profile with browser DevTools
   - Use SIMD when available
   - Minimize JS/Rust boundary crossings

## Common Use Cases

✅ **Image/Video Processing** - Filters, encoding, manipulation <br />
✅ **Games** - 2D/3D games with high performance <br />
✅ **Data Visualization** - Complex charts and graphs <br />
✅ **Cryptography** - Encryption, hashing at native speed <br />
✅ **Compression** - Fast compression/decompression <br />
✅ **Physics Simulations** - Real-time physics engines <br />

## Debugging

```bash
# Build with debug symbols
wasm-pack build --dev

# Use browser DevTools
# Source maps work automatically

# Rust panic messages appear in console
```

## Resources

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [web-sys Documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)
- [awesome-rust-and-webassembly](https://github.com/rustwasm/awesome-rust-and-webassembly)

## Next Steps

After completing this section:
- Build a complete WASM application
- Integrate WASM into a React/Vue/Svelte app
- Explore `wasm-pack` plugin for webpack
- Learn about WASI (WebAssembly System Interface)

---

**Estimated Time**: 12-16 hours
**Difficulty**: ★★★★☆ (Advanced)
**Prerequisites**: Core Rust concepts, basic JavaScript
