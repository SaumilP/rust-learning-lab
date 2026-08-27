/// Broken Hangman Game - Fix the bugs!
/// There are 3 bugs in this code

use std::io::{self, Write};

fn main() {
    let words = vec!["example", "programming", "rust", "hangman", "computer"];
    let word = words[0];  // ❌ BUG 1: Should pick random word, not always first

    let mut guessed_letters = String::new();
    let mut wrong_guesses = 0;
    const MAX_WRONG: u32 = 6;

    println!("╔════════════════════════════════╗");
    println!("║ HANGMAN GAME                   ║");
    println!("╚════════════════════════════════╝\n");

    loop {
        // Display word progress
        print!("Word: ");
        for ch in word.chars() {
            if guessed_letters.contains(ch) {
                print!("{} ", ch);
            } else {
                print!("_ ");
            }
        }
        println!();

        println!("Guessed letters: {}", guessed_letters);
        println!("Wrong guesses: {}/{}\n", wrong_guesses, MAX_WRONG);

        // Check win condition
        let all_guessed = word.chars().all(|ch| guessed_letters.contains(ch));
        if all_guessed {
            println!("╔════════════════════════════════╗");
            println!("║ YOU WIN!                       ║");
            println!("║ Word: {}                 ║", word);
            let score = (MAX_WRONG - wrong_guesses) * 10;
            println!("║ Score: {} points               ║", score);
            println!("╚════════════════════════════════╝");
            break;
        }

        // Check lose condition
        if wrong_guesses >= MAX_WRONG {
            println!("╔════════════════════════════════╗");
            println!("║ GAME OVER                      ║");
            println!("║ Word: {}                 ║", word);
            println!("╚════════════════════════════════╝");
            break;
        }

        // Get guess
        print!("Guess a letter: > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let guess = match input.trim().chars().next() {
            Some(ch) => ch.to_lowercase().next().unwrap(),
            None => continue,
        };

        // ❌ BUG 2: Doesn't check for duplicate guesses
        guessed_letters.push(guess);

        // Check if guess is in word
        if word.contains(guess) {
            println!("✓ Correct!\n");
        } else {
            println!("✗ Wrong!\n");
            wrong_guesses += 1;
            // ❌ BUG 3: Should remove the wrong guess from guessed_letters,
            // but code doesn't undo the push above
        }
    }
}
