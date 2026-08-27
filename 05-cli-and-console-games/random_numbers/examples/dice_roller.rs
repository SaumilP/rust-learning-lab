/// Dice Roller - Random Numbers Example
///
/// Demonstrates:
/// - Using the rand crate for random number generation
/// - Seeding for reproducibility
/// - Collecting statistics on random outcomes
///
/// Run with: cargo run --example dice_roller

use rand::Rng;
use std::collections::HashMap;
use std::io::{self, Write};

struct DiceRoller {
    sides: u32,
    rolls: Vec<u32>,
    stats: HashMap<u32, u32>,
}

impl DiceRoller {
    fn new(sides: u32) -> Self {
        DiceRoller {
            sides,
            rolls: Vec::new(),
            stats: HashMap::new(),
        }
    }

    fn roll(&mut self) -> u32 {
        let mut rng = rand::thread_rng();
        let result = rng.gen_range(1..=self.sides);
        self.rolls.push(result);

        // Update statistics
        *self.stats.entry(result).or_insert(0) += 1;

        result
    }

    fn roll_multiple(&mut self, count: u32) -> Vec<u32> {
        (0..count).map(|_| self.roll()).collect()
    }

    fn display_stats(&self) {
        println!("\n╔════════════════════════════════╗");
        println!("║ DICE STATISTICS                ║");
        println!("║ Total Rolls: {:4}              ║", self.rolls.len());
        println!("╠════════════════════════════════╣");

        let mut sorted_sides: Vec<u32> = self.stats.keys().copied().collect();
        sorted_sides.sort();

        for side in sorted_sides {
            let count = self.stats[&side];
            let percentage = (count as f64 / self.rolls.len() as f64) * 100.0;
            let bar_length = (count * 20) / (self.rolls.len() as u32).max(1);
            let bar = "█".repeat(bar_length as usize);

            println!("║ {:2}: [{:<20}] {:3} ({:5.1}%) ║", side, bar, count, percentage);
        }

        println!("╚════════════════════════════════╝");
    }

    fn get_average(&self) -> f64 {
        if self.rolls.is_empty() {
            return 0.0;
        }
        self.rolls.iter().sum::<u32>() as f64 / self.rolls.len() as f64
    }

    fn get_expected_average(&self) -> f64 {
        (self.sides as f64 + 1.0) / 2.0
    }
}

fn main() {
    println!("╔════════════════════════════════╗");
    println!("║ DICE ROLLER - Statistics Demo  ║");
    println!("╚════════════════════════════════╝\n");

    println!("How many sides? (default: 6)");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let sides: u32 = input.trim().parse().unwrap_or(6);

    println!("\nHow many rolls? (default: 100)");
    print!("> ");
    io::stdout().flush().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let count: u32 = input.trim().parse().unwrap_or(100);

    let mut roller = DiceRoller::new(sides);

    println!("\nRolling {} dice with {} sides...\n", count, sides);

    let results = roller.roll_multiple(count);

    println!("First 10 rolls: {:?}", &results[..results.len().min(10)]);

    roller.display_stats();

    println!("\n╔════════════════════════════════╗");
    println!("║ STATISTICS SUMMARY             ║");
    println!("║ Actual Average:     {:.2}        ║", roller.get_average());
    println!("║ Expected Average:   {:.2}        ║", roller.get_expected_average());
    let min = results.iter().min().unwrap_or(&0);
    let max = results.iter().max().unwrap_or(&0);
    println!("║ Min Roll:           {:4}        ║", min);
    println!("║ Max Roll:           {:4}        ║", max);
    println!("╚════════════════════════════════╝");
}
