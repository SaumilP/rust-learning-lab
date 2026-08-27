# Conway's Game of Life in WebAssembly

Classic cellular automaton running at 60 FPS in the browser.

## Features

- Rendered on HTML5 Canvas
- Interactive: Click to toggle cells
- Play/Pause/Clear controls
- Adjustable speed
- ~100x faster than JavaScript implementation

## Building

```bash
wasm-pack build --target web
python3 -m http.server 8080
```

## The Rules

1. Any live cell with 2-3 neighbors survives
2. Any dead cell with exactly 3 neighbors becomes alive
3. All other cells die or stay dead

## Performance

- 60 FPS on 100x100 grid
- Memory-efficient bit-packing
- SIMD optimizations possible
- Demonstrates WASM's compute capabilities

## Implementation Highlights

- Universe stored as flat `Vec<u8>`
- Double buffering to avoid flickering
- Direct canvas manipulation from Rust
- Minimal JS/WASM boundary crossings
