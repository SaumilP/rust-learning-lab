# Simple Game Logic

## Overview

Game logic is the set of rules that define how a game works. It determines valid actions, calculates outcomes, and manages state transitions. Building effective game logic requires understanding decision structures, state management, and turn-based mechanics. This module covers implementing game rules, win/lose conditions, scoring systems, and turn-based gameplay - the fundamental mechanics that make games engaging.

## Theory

### What is Game Logic?

Game logic encompasses all the rules and mechanics that make a game function:
- **Rules** - What actions are valid, what outcomes occur
- **State** - Current condition of the game world
- **Transitions** - How state changes in response to actions
- **Conditions** - Win/lose/continue states

### Core Components

1. **Input Validation** - Check if player action is legal
2. **Effect Resolution** - Determine what happens after action
3. **State Update** - Modify game state accordingly
4. **Win/Lose Detection** - Check terminal conditions
5. **Feedback** - Tell player what happened

### Common Game Mechanics

1. **Turn-Based** - Players take turns sequentially
2. **Scoring** - Points awarded for actions
3. **Resources** - Health, mana, stamina tracked and consumed
4. **Inventory** - Items collected and used
5. **Levels** - Progression through difficulty tiers
6. **Probability** - Random outcomes based on rules

### State Management in Games

Games maintain state that represents the current world:

```
Game State = {
  Player State (position, health, inventory, score),
  Environment State (obstacles, items, enemies),
  Game Meta State (turn count, elapsed time, phase),
  Condition State (win/lose/continue)
}
```

## Syntax

### Basic Game Logic Structure

```rust
struct GameLogic {
    player_health: i32,
    player_score: i32,
    is_game_over: bool,
}

impl GameLogic {
    fn handle_action(&mut self, action: Action) -> Result<String, String> {
        // Validate
        if !self.is_valid_action(&action) {
            return Err("Invalid action".to_string());
        }

        // Process
        let outcome = self.process_action(action);

        // Update state
        self.apply_outcome(&outcome);

        // Check conditions
        self.check_win_lose();

        Ok(outcome)
    }

    fn is_valid_action(&self, action: &Action) -> bool {
        // Validation logic
        true
    }

    fn process_action(&self, action: Action) -> String {
        // Calculate outcome
        "Action result".to_string()
    }

    fn apply_outcome(&mut self, outcome: &str) {
        // Update state
    }

    fn check_win_lose(&mut self) {
        if self.player_health <= 0 {
            self.is_game_over = true;
        }
    }
}
```

### Turn-Based Game Structure

```rust
struct TurnBasedGame {
    current_player: Player,
    opponent: Player,
    whose_turn: Turn,
    game_state: GameState,
}

#[derive(PartialEq)]
enum Turn {
    Player,
    Opponent,
}

impl TurnBasedGame {
    fn process_turn(&mut self, action: Action) -> Result<String, String> {
        // Validate action is legal
        self.validate_action(&action)?;

        // Process action
        let result = match self.whose_turn {
            Turn::Player => self.process_player_action(action),
            Turn::Opponent => self.process_opponent_action(action),
        };

        // Update game state
        self.apply_result(&result);

        // Switch turns
        self.whose_turn = if self.whose_turn == Turn::Player {
            Turn::Opponent
        } else {
            Turn::Player
        };

        // Check win condition
        if self.is_game_over() {
            return Ok("Game Over".to_string());
        }

        Ok(result)
    }

    fn process_player_action(&mut self, action: Action) -> String {
        // Player-specific logic
        "Player attacked".to_string()
    }

    fn process_opponent_action(&mut self, action: Action) -> String {
        // Opponent-specific logic
        "Opponent defended".to_string()
    }

    fn is_game_over(&self) -> bool {
        self.current_player.health <= 0 || self.opponent.health <= 0
    }
}
```

### Scoring System

```rust
struct ScoringSystem {
    current_score: i32,
    multiplier: i32,
}

impl ScoringSystem {
    fn award_points(&mut self, points: i32) {
        let total = points * self.multiplier;
        self.current_score += total;
    }

    fn apply_multiplier(&mut self, multiplier: i32) {
        self.multiplier = multiplier;
    }

    fn reset_multiplier(&mut self) {
        self.multiplier = 1;
    }

    fn get_score(&self) -> i32 {
        self.current_score
    }
}
```

### Win/Lose Condition Checking

```rust
enum GameResult {
    Ongoing,
    PlayerWon,
    PlayerLost,
    Draw,
}

fn check_game_result(game: &Game) -> GameResult {
    // Check loss conditions first
    if game.player.health <= 0 {
        return GameResult::PlayerLost;
    }

    // Check win conditions
    if game.opponents.is_empty() {
        return GameResult::PlayerWon;
    }

    if game.turns_elapsed > MAX_TURNS {
        return GameResult::Draw;
    }

    // Game ongoing
    GameResult::Ongoing
}
```

### Action Validation

```rust
fn validate_action(game: &Game, action: &Action) -> bool {
    match action {
        Action::Attack { target } => {
            // Can only attack if enemies exist
            game.enemies.len() > 0 && *target < game.enemies.len()
        }
        Action::UseItem { item_id } => {
            // Can only use if player has item
            game.player.inventory.contains(&item_id)
        }
        Action::Move { direction } => {
            // Can only move to valid positions
            let new_pos = game.player.position.move_in(*direction);
            game.is_walkable(new_pos)
        }
        Action::Defend => true, // Always valid
    }
}
```

## Common Patterns

### Pattern 1: Simple Turn-Based Combat

```rust
use std::io::{self, Write};

struct CombatGame {
    player_health: i32,
    enemy_health: i32,
    player_turn: bool,
}

impl CombatGame {
    fn new() -> Self {
        CombatGame {
            player_health: 20,
            enemy_health: 20,
            player_turn: true,
        }
    }

    fn player_attack(&mut self) {
        let damage = 5;
        self.enemy_health -= damage;
        println!("Player attacks for {} damage!", damage);
        self.player_turn = false;
    }

    fn player_defend(&mut self) {
        println!("Player defends!");
        self.player_turn = false;
    }

    fn enemy_turn(&mut self) {
        let action = if self.enemy_health < 10 { "defend" } else { "attack" };

        match action {
            "attack" => {
                let damage = 3;
                self.player_health -= damage;
                println!("Enemy attacks for {} damage!", damage);
            }
            "defend" => {
                println!("Enemy defends!");
            }
            _ => {}
        }

        self.player_turn = true;
    }

    fn check_game_over(&self) -> bool {
        self.player_health <= 0 || self.enemy_health <= 0
    }

    fn run(&mut self) {
        while !self.check_game_over() {
            println!("\nPlayer HP: {} | Enemy HP: {}", self.player_health, self.enemy_health);

            if self.player_turn {
                println!("1) Attack  2) Defend");
                print!("> ");
                io::stdout().flush().unwrap();

                let mut choice = String::new();
                io::stdin().read_line(&mut choice).unwrap();

                match choice.trim() {
                    "1" => self.player_attack(),
                    "2" => self.player_defend(),
                    _ => println!("Invalid action"),
                }
            } else {
                self.enemy_turn();
            }
        }

        println!("\nGAME OVER");
        if self.player_health > 0 {
            println!("Player wins!");
        } else {
            println!("Enemy wins!");
        }
    }
}
```

### Pattern 2: Scoring with Combos

```rust
struct ComboScorer {
    current_combo: i32,
    total_score: i32,
}

impl ComboScorer {
    fn new() -> Self {
        ComboScorer {
            current_combo: 0,
            total_score: 0,
        }
    }

    fn successful_action(&mut self) {
        self.current_combo += 1;
        let points = 10 * self.current_combo;
        self.total_score += points;
        println!("Combo x{}: +{} points", self.current_combo, points);
    }

    fn failed_action(&mut self) {
        if self.current_combo > 1 {
            println!("Combo broken! Final combo: x{}", self.current_combo);
        }
        self.current_combo = 0;
    }

    fn get_score(&self) -> i32 {
        self.total_score
    }
}
```

### Pattern 3: State Machine for Game Phases

```rust
#[derive(PartialEq)]
enum GamePhase {
    Menu,
    Playing,
    Paused,
    GameOver,
}

struct GameWithPhases {
    phase: GamePhase,
    score: i32,
}

impl GameWithPhases {
    fn handle_input(&mut self, input: &str) {
        match self.phase {
            GamePhase::Menu => {
                if input == "start" {
                    self.phase = GamePhase::Playing;
                }
            }
            GamePhase::Playing => {
                match input {
                    "pause" => self.phase = GamePhase::Paused,
                    "attack" => self.score += 10,
                    _ => {}
                }
            }
            GamePhase::Paused => {
                if input == "resume" {
                    self.phase = GamePhase::Playing;
                }
            }
            GamePhase::GameOver => {}
        }
    }
}
```

### Pattern 4: Resource Management

```rust
struct Player {
    health: i32,
    max_health: i32,
    mana: i32,
    max_mana: i32,
}

impl Player {
    fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    fn cast_spell(&mut self, mana_cost: i32) -> bool {
        if self.mana >= mana_cost {
            self.mana -= mana_cost;
            true
        } else {
            false
        }
    }

    fn restore_mana(&mut self, amount: i32) {
        self.mana = (self.mana + amount).min(self.max_mana);
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}
```

### Pattern 5: Event-Based Game Logic

```rust
enum GameEvent {
    PlayerAttacked { damage: i32 },
    PlayerHealed { amount: i32 },
    ItemCollected { name: String },
    LevelUp,
    GameOver,
}

fn handle_event(game: &mut Game, event: GameEvent) {
    match event {
        GameEvent::PlayerAttacked { damage } => {
            game.player.health -= damage;
            println!("Player takes {} damage", damage);
        }
        GameEvent::PlayerHealed { amount } => {
            game.player.health += amount;
            println!("Player heals {} HP", amount);
        }
        GameEvent::ItemCollected { name } => {
            game.player.inventory.push(name.clone());
            println!("Collected: {}", name);
        }
        GameEvent::LevelUp => {
            game.player.level += 1;
            println!("Level up! Now level {}", game.player.level);
        }
        GameEvent::GameOver => {
            game.running = false;
        }
    }
}
```

## Common Mistakes

### Mistake 1: Not Validating Actions

```rust
// ❌ WRONG - No validation
fn process_action(&mut self, action: Action) {
    match action {
        Action::Attack(target) => {
            // Assumes target exists - could crash
            self.enemies[target].health -= 10;
        }
        _ => {}
    }
}

// ✅ CORRECT - Validate first
fn process_action(&mut self, action: Action) -> Result<String, String> {
    match action {
        Action::Attack(target) => {
            if target >= self.enemies.len() {
                return Err("Invalid target".to_string());
            }
            self.enemies[target].health -= 10;
            Ok("Attack successful".to_string())
        }
        _ => Ok("Action completed".to_string()),
    }
}
```

### Mistake 2: Checking Win/Lose at Wrong Time

```rust
// ❌ WRONG - Checks after every state change
loop {
    handle_input();
    update_state();
    check_win_condition();  // Called too frequently
    check_lose_condition(); // Redundant checks
    render();
}

// ✅ CORRECT - Check once per game loop
loop {
    handle_input();
    update_state();
    render();
    check_game_result();  // Single check after all updates
}
```

### Mistake 3: Forgetting State Consistency

```rust
// ❌ WRONG - State can become inconsistent
player.health = -5;  // Negative health allowed
player.score = -100; // Negative score allowed

// ✅ CORRECT - Maintain invariants
player.take_damage(10);      // Ensures health >= 0
player.award_points(100);    // Ensures score valid
```

### Mistake 4: Complex Nested Conditions

```rust
// ❌ WRONG - Hard to follow logic
if action == "attack" {
    if enemy_exists {
        if player_has_ap {
            if enemy_not_dead {
                if attack_hits {
                    // Finally do something
                }
            }
        }
    }
}

// ✅ CORRECT - Early returns and validation
fn process_attack(game: &mut Game) -> Result<String, String> {
    validate_enemy_exists(game)?;
    validate_player_ap(game)?;
    validate_enemy_alive(game)?;

    let hits = calculate_hit(game);
    if !hits {
        return Ok("Miss!".to_string());
    }

    apply_damage(game);
    Ok("Hit!".to_string())
}
```

### Mistake 5: Unclear Responsibility Division

```rust
// ❌ WRONG - Logic spread across many functions
fn update() {
    validate_input();
    apply_effects();
    check_collisions();
    update_positions();
    check_win_conditions();
    render_state();
    // Mixed concerns - hard to test
}

// ✅ CORRECT - Clear separation of concerns
fn handle_input(&mut self, input: Action) -> Result<(), String> { }
fn process_turn(&mut self) { }
fn check_game_state(&self) -> GameState { }
fn render(&self) { }
```

## Real-World Examples

### Example 1: Number Guessing Game with Scoring

```rust
use rand::Rng;
use std::io::{self, Write};

struct GuessingGame {
    secret: i32,
    guesses_made: i32,
    score: i32,
}

impl GuessingGame {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        GuessingGame {
            secret: rng.gen_range(1..=100),
            guesses_made: 0,
            score: 100,
        }
    }

    fn make_guess(&mut self, guess: i32) -> String {
        self.guesses_made += 1;
        self.score = (self.score - 1).max(0);

        match guess.cmp(&self.secret) {
            std::cmp::Ordering::Less => "Too low!".to_string(),
            std::cmp::Ordering::Greater => "Too high!".to_string(),
            std::cmp::Ordering::Equal => {
                format!("Correct! Guesses: {}, Score: {}", self.guesses_made, self.score)
            }
        }
    }

    fn run(&mut self) {
        loop {
            print!("Guess (1-100): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let guess: i32 = input.trim().parse().unwrap_or(0);

            let result = self.make_guess(guess);
            println!("{}", result);

            if guess == self.secret {
                break;
            }
        }
    }
}

fn main() {
    let mut game = GuessingGame::new();
    game.run();
}
```

### Example 2: Rock-Paper-Scissors with Stats

```rust
use rand::seq::SliceRandom;
use std::io::{self, Write};

struct RPSGame {
    player_wins: i32,
    computer_wins: i32,
    rounds: i32,
}

#[derive(PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl RPSGame {
    fn new() -> Self {
        RPSGame {
            player_wins: 0,
            computer_wins: 0,
            rounds: 0,
        }
    }

    fn get_computer_choice(&self) -> Choice {
        let choices = vec![Choice::Rock, Choice::Paper, Choice::Scissors];
        let mut rng = rand::thread_rng();
        choices.choose(&mut rng).unwrap().clone()
    }

    fn determine_winner(&mut self, player: &Choice, computer: &Choice) -> String {
        let result = match (player, computer) {
            (Choice::Rock, Choice::Scissors) => {
                self.player_wins += 1;
                "Player wins!"
            }
            (Choice::Paper, Choice::Rock) => {
                self.player_wins += 1;
                "Player wins!"
            }
            (Choice::Scissors, Choice::Paper) => {
                self.player_wins += 1;
                "Player wins!"
            }
            (a, b) if a == b => "Draw!",
            _ => {
                self.computer_wins += 1;
                "Computer wins!"
            }
        };

        self.rounds += 1;
        result.to_string()
    }

    fn display_stats(&self) {
        println!("\n=== STATS ===");
        println!("Player: {}", self.player_wins);
        println!("Computer: {}", self.computer_wins);
        println!("Rounds: {}", self.rounds);
    }

    fn run(&mut self) {
        loop {
            println!("\n1) Rock  2) Paper  3) Scissors  4) Quit");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let player_choice = match input.trim() {
                "1" => Choice::Rock,
                "2" => Choice::Paper,
                "3" => Choice::Scissors,
                "4" => break,
                _ => {
                    println!("Invalid choice");
                    continue;
                }
            };

            let computer_choice = self.get_computer_choice();
            let result = self.determine_winner(&player_choice, &computer_choice);
            println!("{}", result);
        }

        self.display_stats();
    }
}

fn main() {
    let mut game = RPSGame::new();
    game.run();
}
```

### Example 3: Adventure Game with Inventory

```rust
struct AdventureGame {
    player_health: i32,
    inventory: Vec<String>,
    location: String,
    game_over: bool,
}

impl AdventureGame {
    fn new() -> Self {
        AdventureGame {
            player_health: 100,
            inventory: vec![],
            location: "Forest".to_string(),
            game_over: false,
        }
    }

    fn pickup_item(&mut self, item: &str) {
        self.inventory.push(item.to_string());
        println!("Picked up: {}", item);
    }

    fn use_item(&mut self, item: &str) -> bool {
        if let Some(index) = self.inventory.iter().position(|x| x == item) {
            self.inventory.remove(index);
            println!("Used: {}", item);
            true
        } else {
            println!("You don't have that item");
            false
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.player_health -= damage;
        if self.player_health <= 0 {
            self.game_over = true;
            println!("You died!");
        }
    }

    fn move_location(&mut self, location: &str) {
        self.location = location.to_string();
        println!("Moved to: {}", self.location);
    }

    fn check_win_condition(&self) -> bool {
        self.inventory.contains(&"treasure".to_string())
    }

    fn run(&mut self) {
        self.pickup_item("sword");

        loop {
            if self.game_over {
                println!("GAME OVER");
                break;
            }

            if self.check_win_condition() {
                println!("YOU WIN!");
                break;
            }

            println!("\nHealth: {} | Location: {} | Inventory: {:?}",
                     self.player_health, self.location, self.inventory);
            println!("1) Move  2) Use item  3) Take damage (for demo)");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "1" => self.move_location("Castle"),
                "2" => { self.use_item("sword"); }
                "3" => self.take_damage(20),
                _ => println!("Invalid"),
            }
        }
    }
}

fn main() {
    let mut game = AdventureGame::new();
    game.run();
}
```

## Related Concepts

### Prerequisites
- Module 01: Functions, Control Flow, Pattern Matching
- Module 02: Collections, Iterators
- Module 04: Basic Programs, Text Processing
- Module 05: Game Loops, Console I/O, Random Numbers

### Follow-ups
- Advanced Game Architecture (ECS systems, event systems)
- Network Programming (multiplayer game logic)
- Performance Optimization (game logic profiling)

## Best Practices

1. **Validate Early** - Check preconditions before processing
2. **Separate Concerns** - Input → Logic → Output
3. **Use Enums for States** - Better than string comparisons
4. **Check Conditions After Updates** - Not during processing
5. **Handle All Cases** - Use match exhaustively
6. **Clear Naming** - Intent obvious from function names
7. **Return Results** - Use Result for fallible operations
8. **Test Logic Independently** - Without UI/rendering

## Summary

Game logic is the heart of every game. Well-designed logic is clear, maintainable, and handles all edge cases. By separating validation, processing, and condition checking into distinct phases, you create games that are fun to play and easy to extend. Understanding these patterns enables building everything from simple text games to complex interactive experiences.

## Practice Exercise Ideas

1. Create a simple dungeon crawler with rooms and enemies
2. Build a turn-based strategy game with resource management
3. Implement a quiz game with scoring and difficulty levels
4. Create a simplified RPG with character stats and progression
5. Build a card game with deck management and hand evaluation

