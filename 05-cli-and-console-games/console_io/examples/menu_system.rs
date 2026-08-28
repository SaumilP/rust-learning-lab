/// Menu System - Console I/O Example
///
/// Demonstrates:
/// - Formatted console output
/// - Prompt flushing for user input
/// - Menu-driven program structure
/// - Input validation and error handling
///
/// Run with: cargo run --example menu_system
use std::io::{self, Write};

struct UserProfile {
    name: String,
    age: u32,
    email: String,
}

impl UserProfile {
    fn new() -> Self {
        UserProfile {
            name: String::new(),
            age: 0,
            email: String::new(),
        }
    }

    fn display(&self) {
        println!("\n╔════════════════════════════════╗");
        println!("║ USER PROFILE                   ║");
        println!("╠════════════════════════════════╣");
        println!("║ Name:  {:24}  ║", self.name);
        println!("║ Age:   {:24}  ║", self.age);
        println!("║ Email: {:24}  ║", self.email);
        println!("╚════════════════════════════════╝\n");
    }

    fn is_complete(&self) -> bool {
        !self.name.is_empty() && self.age > 0 && !self.email.is_empty()
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[H", esc = 27 as char);
}

fn print_main_menu() {
    println!("╔════════════════════════════════╗");
    println!("║ PROFILE MANAGER                ║");
    println!("╠════════════════════════════════╣");
    println!("║ 1. Set Name                    ║");
    println!("║ 2. Set Age                     ║");
    println!("║ 3. Set Email                   ║");
    println!("║ 4. View Profile                ║");
    println!("║ 5. Save Profile                ║");
    println!("║ 6. Exit                        ║");
    println!("╚════════════════════════════════╝");
    print!("Choose option (1-6): ");
    io::stdout().flush().unwrap();
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_positive_number() -> Option<u32> {
    print!("Enter value: ");
    io::stdout().flush().unwrap();

    let input = read_line();
    match input.parse::<u32>() {
        Ok(n) if n > 0 => Some(n),
        _ => {
            eprintln!("✗ Invalid number. Please enter a positive number.");
            None
        }
    }
}

fn read_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    read_line()
}

fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

fn set_name(profile: &mut UserProfile) {
    println!("\n--- Set Name ---");
    let name = read_string("Enter name: ");
    if name.is_empty() {
        eprintln!("✗ Name cannot be empty");
    } else if name.len() > 50 {
        eprintln!("✗ Name too long (max 50 characters)");
    } else {
        profile.name = name;
        println!("✓ Name set successfully");
    }
}

fn set_age(profile: &mut UserProfile) {
    println!("\n--- Set Age ---");
    if let Some(age) = read_positive_number() {
        if age > 150 {
            eprintln!("✗ Age seems unrealistic");
        } else {
            profile.age = age;
            println!("✓ Age set successfully");
        }
    }
}

fn set_email(profile: &mut UserProfile) {
    println!("\n--- Set Email ---");
    let email = read_string("Enter email: ");
    if !validate_email(&email) {
        eprintln!("✗ Invalid email format");
    } else {
        profile.email = email;
        println!("✓ Email set successfully");
    }
}

fn save_profile(profile: &UserProfile) {
    println!("\n--- Save Profile ---");
    if !profile.is_complete() {
        eprintln!("✗ Profile incomplete. Please fill in all fields:");
        if profile.name.is_empty() {
            eprintln!("  - Name is required");
        }
        if profile.age == 0 {
            eprintln!("  - Age is required");
        }
        if profile.email.is_empty() {
            eprintln!("  - Email is required");
        }
    } else {
        println!("✓ Profile saved successfully!");
        println!("\nProfile Summary:");
        println!("  Name:  {}", profile.name);
        println!("  Age:   {}", profile.age);
        println!("  Email: {}", profile.email);
    }
}

fn main() {
    clear_screen();
    let mut profile = UserProfile::new();
    let mut running = true;

    println!("╔════════════════════════════════╗");
    println!("║ Welcome to Profile Manager     ║");
    println!("║ Fill in your information below ║");
    println!("╚════════════════════════════════╝\n");

    while running {
        print_main_menu();
        let choice = read_line();

        match choice.as_str() {
            "1" => set_name(&mut profile),
            "2" => set_age(&mut profile),
            "3" => set_email(&mut profile),
            "4" => profile.display(),
            "5" => save_profile(&profile),
            "6" => {
                println!("\nGoodbye!");
                running = false;
            }
            _ => eprintln!("✗ Invalid option. Please choose 1-6."),
        }

        if running {
            println!("\nPress Enter to continue...");
            let _ = read_line();
        }
    }
}
