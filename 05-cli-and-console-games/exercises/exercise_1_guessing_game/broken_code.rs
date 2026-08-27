/// Broken Guessing Game - Fix the bugs!
///
/// There are 3 bugs in this code. Find and fix them to make the game work.

use std::io::{self, Write};

// BUG 1: This simple random generator has an issue
fn simple_random(max: u32) -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    (duration.as_nanos() as u32) % max  // ❌ BUG: Should be 1..=max, not 0..max
}

fn main() {
    println!("╔════════════════════════════════╗");
    println!("║ NUMBER GUESSING GAME           ║");
    println!("║ Guess a number (1-100)         ║");
    println!("╚════════════════════════════════╝\n");

    // BUG 2: Secret number can be 0 because of simple_random bug above
    let secret = simple_random(100);
    let mut guesses = 0;
    let mut game_running = true;

    while game_running {
        println!("Guesses: {}", guesses);
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // BUG 3: Input validation is incomplete - doesn't check range or invalid numbers
        let guess = match input.trim().parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input!");
                continue;  // ❌ BUG: Should check if number is in range 1-100
            }
        };

        guesses += 1;

        if guess < secret {
            println!("Too low! Try higher.\n");
        } else if guess > secret {
            println!("Too high! Try lower.\n");
        } else {
            println!("Correct! You found it!\n");

            // Calculate score
            let score = if guesses < 10 {
                100 - (guesses * 2)
            } else {
                10  // Minimum score
            } as i32;

            println!("╔════════════════════════════════╗");
            println!("║ GAME OVER                      ║");
            println!("║ Final Score: {} points         ║", score);
            println!("║ Guesses: {}                      ║", guesses);
            let efficiency = (100 - (guesses * 2)).max(10);
            println!("║ Efficiency: {}%                ║", efficiency);
            println!("╚════════════════════════════════╝");

            game_running = false;
        }
    }
}
