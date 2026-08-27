# Game Loop Basics - Key Takeaways

## Core Pattern

```rust
while running {
    handle_input();
    update();
    render();
}
```

## Three Phases

| Phase | Purpose | Timing |
|-------|---------|--------|
| Input | Capture user actions | Fast (~1ms) |
| Update | Process game logic | Variable (depends on logic) |
| Render | Display state | Fast (~10-30ms) |

## Frame Rate Control

```rust
use std::time::{Instant, Duration};

let frame_time = Duration::from_millis(33); // ~30 FPS
let start = Instant::now();

// game loop code

if start.elapsed() < frame_time {
    std::thread::sleep(frame_time - start.elapsed());
}
```

## State Management

```rust
// Centralized state
struct GameState {
    player_pos: (i32, i32),
    score: i32,
    is_running: bool,
}

// Methods for each phase
impl GameState {
    fn handle_input(&mut self, input: Input) { }
    fn update(&mut self) { }
    fn render(&self) { }
}
```

## Important Patterns

| Pattern | When to Use |
|---------|------------|
| Simple loop | Prototypes, small games |
| Struct-based | Organized state management |
| Event-driven | Complex interactions |
| Delta-time | Movement calculations |
| Tick-based | Turn-based games |

## Common Loop Structures

```rust
// Minimal
while running {
    handle_input();
    update();
    render();
}

// With timing
while running {
    let start = Instant::now();
    handle_input();
    update();
    render();
    sleep_to_target_fps(start);
}

// With state struct
impl Game {
    fn run(&mut self) {
        while self.running {
            self.handle_input();
            self.update();
            self.render();
        }
    }
}
```

## Critical Concepts

✓ Game loop runs continuously until exit condition
✓ Each iteration processes one "frame" of game
✓ Order matters: input → update → render
✓ State should be centralized in struct
✓ Frame rate should be controlled
✓ Input should be non-blocking

## FPS Calculation

```rust
let fps = 60;  // 60 frames per second
let frame_time_ms = 1000 / fps;  // 16.67 ms per frame

// 30 FPS = 33ms per frame
// 60 FPS = 16ms per frame
// 120 FPS = 8ms per frame
```

## Important Notes

✓ Render happens every frame
✓ Update logic should handle variable time deltas
✓ Input handling should be responsive
✓ CPU usage determined by frame rate
✓ Most games target 30-60 FPS
✓ Separate rendering from game logic

## Quick Reference

```rust
// Timing
use std::time::{Instant, Duration};

// Basic loop structure
let mut running = true;
while running {
    // Input
    let cmd = read_input();

    // Update
    if cmd == "quit" { running = false; }

    // Render
    display_game();
}

// Frame rate limiting
let target_frame_time = Duration::from_millis(33);
let frame_start = Instant::now();
// ... game logic ...
let elapsed = frame_start.elapsed();
if elapsed < target_frame_time {
    std::thread::sleep(target_frame_time - elapsed);
}
```

## Common Mistakes to Avoid

1. ❌ Blocking on input - makes game freeze
2. ❌ No frame rate control - wastes CPU
3. ❌ State scattered across variables - hard to manage
4. ❌ Mixing phases - hard to debug
5. ❌ No exit condition - infinite loop

