# Hello WebAssembly

Your first Rust WebAssembly module running in the browser.

## What This Does

- Compiles Rust to WebAssembly
- Exports functions to JavaScript
- Runs in the browser without any JS framework

## Building

```bash
# Build the WASM module
wasm-pack build --target web

# Serve locally
python3 -m http.server 8080

# Open http://localhost:8080 in browser
```

## How It Works

1. **Rust Code** (`src/lib.rs`) - Defines functions with `#[wasm_bindgen]`
2. **wasm-pack** - Compiles Rust to WASM + generates JS bindings
3. **HTML** - Loads and calls the WASM module
4. **Browser** - Executes WASM at near-native speed

## Key Concepts

### `#[wasm_bindgen]` Attribute

Makes Rust functions callable from JavaScript:

```rust
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### JavaScript Imports

```javascript
import init, { greet } from './pkg/hello_wasm.js';

await init();
const message = greet("World");
```

## File Size

Typical WASM binary: ~15-20KB (optimized)

## Next Steps

- Try modifying the greeting message
- Add more functions
- Pass different data types
- Use console.log from Rust
