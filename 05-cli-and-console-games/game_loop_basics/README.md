# Game Loop Basics

## Overview

A game loop is the core structure of any game, repeatedly executing update and render operations. Understanding game loops is fundamental for building interactive applications. The pattern applies to games, simulations, and real-time systems. In Rust, implementing an efficient game loop requires managing state, handling input, and controlling frame rates.

## Theory

### The Game Loop Pattern

Every frame (iteration) of a game typically follows this sequence:

1. **Input** - Capture user actions (keyboard, mouse, etc.)
2. **Update** - Process game logic based on current state
3. **Render** - Display current game state to the player
4. **Timing** - Control frame rate and timing

### Why Game Loops Matter

- **Responsiveness** - Game reacts immediately to user input
- **Consistency** - Predictable update frequency
- **Interactivity** - Continuous state management
- **Performance** - Frame rate control prevents CPU waste

### Types of Game Loops

1. **Fixed Timestep** - Updates happen at fixed intervals
2. **Variable Timestep** - Updates based on actual elapsed time
3. **Frame-Limited** - Renders as fast as possible up to FPS cap

## Syntax

### Basic Game Loop Structure

```rust
fn main() {
    // Initialize game state
    let mut game_state = GameState::new();
    let mut running = true;

    while running {
        // Input phase
        let input = read_user_input();

        // Update phase
        game_state.update(input);

        // Render phase
        render(&game_state);

        // Check for exit condition
        if game_state.should_exit() {
            running = false;
        }
    }
}
```

### With Frame Rate Control

```rust
use std::time::{Instant, Duration};

fn main() {
    let frame_time = Duration::from_millis(33); // ~30 FPS
    let mut game_state = GameState::new();
    let mut running = true;

    while running {
        let frame_start = Instant::now();

        // Game loop logic
        let input = read_user_input();
        game_state.update(input);
        render(&game_state);

        // Frame rate control
        let frame_duration = frame_start.elapsed();
        if frame_duration < frame_time {
            std::thread::sleep(frame_time - frame_duration);
        }

        running = !game_state.should_exit();
    }
}
```

### State Management Pattern

```rust
struct GameState {
    player_pos: (i32, i32),
    score: i32,
    is_running: bool,
}

impl GameState {
    fn new() -> Self {
        GameState {
            player_pos: (0, 0),
            score: 0,
            is_running: true,
        }
    }

    fn update(&mut self, input: PlayerInput) {
        // Update player position based on input
        match input {
            PlayerInput::Up => self.player_pos.1 -= 1,
            PlayerInput::Down => self.player_pos.1 += 1,
            PlayerInput::Left => self.player_pos.0 -= 1,
            PlayerInput::Right => self.player_pos.0 += 1,
            PlayerInput::Quit => self.is_running = false,
        }
    }

    fn render(&self) {
        println!("Player: {:?}, Score: {}", self.player_pos, self.score);
    }

    fn should_exit(&self) -> bool {
        !self.is_running
    }
}
```

## Common Patterns

### Pattern 1: Simple Loop with State

```rust
let mut game_active = true;
let mut score = 0;

while game_active {
    // Get input
    let command = read_input();

    // Process
    match command {
        "up" => { /* move */ }
        "down" => { /* move */ }
        "quit" => game_active = false,
        _ => {}
    }

    // Render
    println!("Score: {}", score);
}
```

### Pattern 2: Struct-Based State

```rust
struct Game {
    state: GameState,
    is_running: bool,
}

impl Game {
    fn run(&mut self) {
        while self.is_running {
            self.handle_input();
            self.update();
            self.render();
        }
    }

    fn handle_input(&mut self) {
        // Input handling
    }

    fn update(&mut self) {
        // Game logic
    }

    fn render(&self) {
        // Display game state
    }
}
```

### Pattern 3: Delta Time for Updates

```rust
let mut last_time = Instant::now();

while running {
    let now = Instant::now();
    let delta_time = now.duration_since(last_time);
    last_time = now;

    // Update with delta time
    game_state.update(delta_time);

    // Render
    render(&game_state);
}
```

### Pattern 4: Event-Driven Approach

```rust
loop {
    for event in get_events() {
        match event {
            Event::Input(key) => handle_input(key),
            Event::Update => update_game(),
            Event::Render => render_screen(),
            Event::Quit => break,
        }
    }
}
```

### Pattern 5: Timed Events

```rust
struct GameLoop {
    tick: u64,
    events: Vec<TimedEvent>,
}

impl GameLoop {
    fn run(&mut self) {
        while !self.should_quit() {
            // Process events for current tick
            for event in self.events_at_tick(self.tick) {
                self.handle_event(event);
            }

            self.tick += 1;
        }
    }

    fn events_at_tick(&self, tick: u64) -> Vec<&TimedEvent> {
        self.events.iter().filter(|e| e.tick == tick).collect()
    }
}
```

## Common Mistakes

### Mistake 1: Blocking Input Operations

```rust
// ❌ WRONG - Blocks entire game loop waiting for input
let input = read_line();

// ✅ CORRECT - Non-blocking input or timeout
let input = read_input_with_timeout(Duration::from_millis(16));
```

### Mistake 2: No Frame Rate Control

```rust
// ❌ WRONG - Loop runs as fast as possible, consuming CPU
while running {
    update();
    render();
}

// ✅ CORRECT - Control frame rate
let frame_time = Duration::from_millis(33);
while running {
    let start = Instant::now();
    update();
    render();
    let elapsed = start.elapsed();
    if elapsed < frame_time {
        thread::sleep(frame_time - elapsed);
    }
}
```

### Mistake 3: Accumulating Mutable State

```rust
// ❌ WRONG - State scattered across variables
let mut x = 0;
let mut y = 0;
let mut score = 0;
let mut health = 100;

// ✅ CORRECT - Centralized state struct
struct GameState {
    x: i32,
    y: i32,
    score: i32,
    health: i32,
}
```

### Mistake 4: Mixed Concerns

```rust
// ❌ WRONG - All logic in one loop
while running {
    // Input, update, render all tangled together
}

// ✅ CORRECT - Separated concerns
while running {
    handle_input();  // Input phase
    update();        // Update phase
    render();        // Render phase
}
```

### Mistake 5: Ignoring Timing Issues

```rust
// ❌ WRONG - Frame rate varies based on workload
while running {
    do_game_logic();  // Takes 5-20ms
    render();         // Takes 10-30ms
}

// ✅ CORRECT - Consistent timing
let frame_time = Duration::from_millis(33);
while running {
    let frame_start = Instant::now();
    do_game_logic();
    render();
    sleep_if_needed(frame_time, frame_start.elapsed());
}
```

## Real-World Examples

### Example 1: Simple Counter Game

```rust
use std::io::{self, Write};

fn main() {
    let mut score = 0;
    let mut running = true;

    while running {
        // Render
        println!("Score: {}", score);
        print!("> ");
        io::stdout().flush().unwrap();

        // Input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Update
        match input {
            "up" => score += 1,
            "down" => score = (score - 1).max(0),
            "quit" => running = false,
            _ => println!("Unknown command"),
        }

        println!();
    }

    println!("Final score: {}", score);
}
```

### Example 2: Game State with Struct

```rust
struct Game {
    player_x: i32,
    player_y: i32,
    score: i32,
    running: bool,
}

impl Game {
    fn new() -> Self {
        Game {
            player_x: 0,
            player_y: 0,
            score: 0,
            running: true,
        }
    }

    fn run(&mut self) {
        while self.running {
            self.render();
            self.handle_input();
            self.update();
        }
    }

    fn render(&self) {
        println!("Player: ({}, {}), Score: {}", self.player_x, self.player_y, self.score);
    }

    fn handle_input(&mut self) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "w" => self.player_y -= 1,
            "s" => self.player_y += 1,
            "a" => self.player_x -= 1,
            "d" => self.player_x += 1,
            "q" => self.running = false,
            _ => {}
        }
    }

    fn update(&mut self) {
        // Game logic
        self.score += 1;
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
```

## Related Concepts

### Prerequisites
- Module 01: Functions, Control Flow
- Module 04: CLI Arguments, Simple Programs
- Basic struct usage and trait implementations

### Follow-ups
- Random Numbers (generating game events)
- Console I/O (interactive input/output)
- Simple Game Logic (game rules)
- Advanced Game Architecture (ECS, systems)

## Best Practices

1. **Separate Concerns** - Keep input, update, and render separate
2. **Centralize State** - Use structs for game state
3. **Control Frame Rate** - Ensure consistent timing
4. **Handle Input Gracefully** - Don't block on input
5. **Measure Performance** - Track actual frame rates
6. **Use Enums** - For game states and events
7. **Test Independently** - Test logic without rendering

## Summary

Game loops are the heartbeat of interactive applications. A well-designed game loop separates input handling, state updates, and rendering into distinct phases. Proper timing control ensures consistent gameplay and efficient CPU usage. Understanding the game loop pattern is essential for building engaging interactive experiences.

## Practice Exercise Ideas

1. Create a simple counter game with increment/decrement
2. Build a player movement simulator with position tracking
3. Implement a turn-based game with state management
4. Create a timed game that tracks elapsed frames
5. Build a game with multiple game states (menu, playing, game over)

