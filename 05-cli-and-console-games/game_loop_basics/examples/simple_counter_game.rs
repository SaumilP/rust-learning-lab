/// Simple Counter Game - Game Loop Example
///
/// Demonstrates:
/// - Basic game loop structure (input -> update -> render)
/// - State management with a struct
/// - Frame-based game loop with timing
///
/// Run with: cargo run --example simple_counter_game
use std::io::{self, Write};
use std::time::{Duration, Instant};

struct GameState {
    counter: i32,
    is_running: bool,
    frames: u64,
}

impl GameState {
    fn new() -> Self {
        GameState {
            counter: 0,
            is_running: true,
            frames: 0,
        }
    }

    fn render(&self) {
        println!("\n╔════════════════════╗");
        println!("║ COUNTER GAME       ║");
        println!("║ Counter: {:8}    ║", self.counter);
        println!("║ Frames: {:9}    ║", self.frames);
        println!("╠════════════════════╣");
        println!("║ Commands:          ║");
        println!("║ u) Up    d) Down   ║");
        println!("║ r) Reset q) Quit   ║");
        println!("╚════════════════════╝");
        print!("> ");
        io::stdout().flush().unwrap();
    }

    fn handle_input(&mut self, input: &str) {
        match input.trim() {
            "u" => {
                self.counter += 1;
                println!("✓ Incremented to {}", self.counter);
            }
            "d" => {
                self.counter -= 1;
                println!("✓ Decremented to {}", self.counter);
            }
            "r" => {
                self.counter = 0;
                println!("✓ Reset to 0");
            }
            "q" => {
                self.is_running = false;
                println!("Thanks for playing!");
            }
            _ => println!("Invalid command. Try again."),
        }
    }

    fn update(&mut self) {
        self.frames += 1;
    }

    fn run(&mut self) {
        // Target 2 FPS for demonstration (500ms per frame)
        let frame_time = Duration::from_millis(500);

        println!("Welcome to Counter Game!");
        println!("This demonstrates a basic game loop pattern.\n");

        while self.is_running {
            let frame_start = Instant::now();

            // Render phase
            self.render();

            // Input phase
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            // Update phase
            self.handle_input(&input);
            self.update();

            // Frame rate control
            let elapsed = frame_start.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }
        }

        println!("\nFinal Score: {} points", self.counter);
        println!("Total Frames: {}", self.frames);
    }
}

fn main() {
    let mut game = GameState::new();
    game.run();
}
