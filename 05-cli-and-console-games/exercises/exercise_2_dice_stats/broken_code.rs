/// Broken Dice Roller - Fix the bugs!
/// There are 3 bugs in this code

use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
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
    let rolls: u32 = input.trim().parse().unwrap_or(100);

    // BUG 1: Input validation is missing - should reject sides < 2 and rolls < 1
    println!("\nRolling {} dice with {} sides...", rolls, sides);

    let mut freq: HashMap<u32, u32> = HashMap::new();
    let mut total: u32 = 0;

    for _ in 0..rolls {
        let roll = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u32)
            % sides
            + 1;  // ✓ This is correct

        *freq.entry(roll).or_insert(0) += 1;
        total += roll;
    }

    println!("\n╔════════════════════════════════╗");
    println!("║ DICE STATISTICS                ║");
    println!("║ Total Rolls: {}                ║", rolls);
    println!("╠════════════════════════════════╣");

    for i in 1..=sides {
        let count = freq.get(&i).copied().unwrap_or(0);
        // BUG 2: Bar calculation is wrong - always shows 20 characters
        let bar_length = 20;  // ❌ Should be proportional to count
        let bar = "█".repeat(bar_length as usize) + &"░".repeat(20 - bar_length as usize);
        let percentage = (count as f64 / rolls as f64) * 100.0;

        println!("║ {:2}: [{}] {:3} ({:5.1}%) ║", i, bar, count, percentage);
    }

    println!("╚════════════════════════════════╝");

    let average = total as f64 / rolls as f64;
    // BUG 3: Expected value formula is incorrect
    let expected = (sides + 1) as f64;  // ❌ Should be (sides as f64 + 1.0) / 2.0

    println!("\n╔════════════════════════════════╗");
    println!("║ STATISTICS SUMMARY             ║");
    println!("║ Actual Average:     {:.2}        ║", average);
    println!("║ Expected Average:   {:.2}        ║", expected);

    if let Some(&min) = freq.keys().min() {
        println!("║ Min Roll:           {:4}        ║", min);
    }

    if let Some(&max) = freq.keys().max() {
        println!("║ Max Roll:           {:4}        ║", max);
    }

    println!("╚════════════════════════════════╝");
}
