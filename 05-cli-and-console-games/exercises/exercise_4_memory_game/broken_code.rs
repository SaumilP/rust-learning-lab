/// Broken Memory Game - Fix the bugs!
/// There are 3 bugs in this code

use std::io::{self, Write};
use std::time::Duration;

fn display_sequence(seq: &[u32]) {
    println!("Sequence: {}", seq.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
    std::thread::sleep(Duration::from_secs(1));
    println!("Memorize the sequence...");
    std::thread::sleep(Duration::from_secs(2));
}

fn get_player_sequence(expected_len: usize) -> Vec<u32> {
    let mut player_seq = Vec::new();
    println!("\nEnter the sequence:");

    for i in 0..expected_len {
        print!("Number {}: > ", i + 1);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // ❌ BUG 1: Doesn't validate that input is 1-4
        if let Ok(num) = input.trim().parse::<u32>() {
            player_seq.push(num);
        } else {
            println!("Invalid input!");
            // ❌ BUG 2: Doesn't ask again, just continues
            // This skips the current position!
        }
    }

    player_seq
}

fn main() {
    println!("╔════════════════════════════════╗");
    println!("║ MEMORY SEQUENCE GAME           ║");
    println!("╚════════════════════════════════╝\n");

    let mut sequence = Vec::new();
    let mut level = 1;

    loop {
        println!("Level {}", level);

        // Add new number to sequence
        let new_num = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u32)
            % 4
            + 1;
        sequence.push(new_num);

        display_sequence(&sequence);

        let player_seq = get_player_sequence(sequence.len());

        // ❌ BUG 3: Compares lengths but should compare contents
        if player_seq.len() != sequence.len() {
            println!("✗ Wrong!");
            println!("Sequence was: {:?}", sequence);
            println!("You entered:  {:?}", player_seq);

            println!("\n╔════════════════════════════════╗");
            println!("║ GAME OVER                      ║");
            println!("║ Final Level: {}                ║", level);
            let score = level * 10;
            println!("║ Score: {} points               ║", score);
            println!("╚════════════════════════════════╝");
            break;
        }

        println!("✓ Correct! Level Complete!\n");
        level += 1;
    }
}
