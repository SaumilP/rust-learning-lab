# Simple Game Logic - Key Takeaways

## Core Pattern: Validate → Process → Update → Check

```rust
fn handle_action(&mut self, action: Action) -> Result<String, String> {
    // 1. Validate
    self.validate(&action)?;

    // 2. Process
    let outcome = self.process(action);

    // 3. Update
    self.update(&outcome);

    // 4. Check win/lose
    self.check_conditions();

    Ok(outcome)
}
```

## Three Game Phases

| Phase | Purpose | Example |
|-------|---------|---------|
| Input | Get player action | Read "attack" command |
| Update | Apply game logic | Deal damage, award points |
| Check | Verify win/lose | Is enemy dead? Is health 0? |

## Turn-Based Structure

```rust
loop {
    render_state();

    // Input
    let action = read_input();

    // Validate
    if !is_valid(&action) { continue; }

    // Process
    apply_action(action);

    // Check end condition
    if is_game_over() { break; }

    // Switch turns
    switch_turn();
}
```

## Win/Lose Conditions

```rust
enum GameResult {
    Ongoing,
    Won,
    Lost,
    Draw,
}

fn check_result(game: &Game) -> GameResult {
    // Loss first
    if game.player.health <= 0 {
        return GameResult::Lost;
    }

    // Then win
    if game.enemies.is_empty() {
        return GameResult::Won;
    }

    // Special conditions
    if game.score >= 1000 {
        return GameResult::Won;
    }

    GameResult::Ongoing
}
```

## Action Validation

```rust
fn is_valid_action(game: &Game, action: &Action) -> bool {
    match action {
        Action::Attack(target) => *target < game.enemies.len(),
        Action::UseItem(item) => game.inventory.contains(item),
        Action::Move(dir) => can_move(game, *dir),
        _ => true,  // Always valid
    }
}
```

## Scoring Pattern

```rust
struct Score {
    points: i32,
    combo: i32,
}

impl Score {
    fn award(&mut self, amount: i32) {
        self.points += amount * self.combo;
        self.combo += 1;
    }

    fn reset_combo(&mut self) {
        self.combo = 0;
    }
}
```

## State Management

```rust
struct GameState {
    player_health: i32,
    player_score: i32,
    enemies: Vec<Enemy>,
    turn_count: i32,
    is_running: bool,
}

impl GameState {
    fn apply_damage(&mut self, target: usize, damage: i32) {
        if target < self.enemies.len() {
            self.enemies[target].health -= damage;
        }
    }

    fn check_losses(&mut self) {
        // Remove dead enemies
        self.enemies.retain(|e| e.health > 0);
    }
}
```

## Resource Management

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

    fn cast_spell(&mut self, cost: i32) -> bool {
        if self.mana >= cost {
            self.mana -= cost;
            true
        } else {
            false
        }
    }
}
```

## Common Game Logic Patterns

| Pattern | Use Case |
|---------|----------|
| Validate → Process | Ensure legal actions |
| Check after update | Determine game end |
| State struct | Centralize game data |
| Enum for states | Track current phase |
| Early return | Reduce nesting |
| Result<T, E> | Handle failures |

## Event Handling

```rust
enum GameEvent {
    Attack { damage: i32 },
    Heal { amount: i32 },
    ItemPickup { name: String },
    LevelUp,
}

fn handle_event(game: &mut Game, event: GameEvent) {
    match event {
        GameEvent::Attack { damage } => {
            game.player.health -= damage;
        }
        GameEvent::Heal { amount } => {
            game.player.health += amount;
        }
        GameEvent::ItemPickup { name } => {
            game.inventory.push(name);
        }
        GameEvent::LevelUp => {
            game.player.level += 1;
        }
    }
}
```

## State Machines

```rust
#[derive(PartialEq)]
enum GamePhase {
    Menu,
    Playing,
    Paused,
    GameOver,
}

fn next_phase(phase: GamePhase, input: &str) -> GamePhase {
    match (phase, input) {
        (GamePhase::Menu, "start") => GamePhase::Playing,
        (GamePhase::Playing, "pause") => GamePhase::Paused,
        (GamePhase::Paused, "resume") => GamePhase::Playing,
        (_, "quit") => GamePhase::GameOver,
        (p, _) => p,
    }
}
```

## Important Notes

✓ Validate before processing (avoid crashes)
✓ Check win/lose after all updates complete
✓ Use Result for operations that can fail
✓ Maintain invariants (health >= 0, score >= 0)
✓ Keep state in centralized struct
✓ Use enums for game states
✓ Early return reduces nesting
✓ Test logic independently from rendering

## Common Mistakes to Avoid

1. ❌ No validation → crashes on invalid input
2. ❌ Checking win before all updates → misses conditions
3. ❌ State scattered across variables → inconsistent
4. ❌ Deep nesting → hard to follow
5. ❌ Ignoring edge cases → unexpected behavior

## Quick Reference

```rust
// Basic structure
struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    score: i32,
    running: bool,
}

// Process turn
fn process_turn(&mut self, action: Action) -> Result<String, String> {
    // Validate
    if !self.is_valid(&action) {
        return Err("Invalid".to_string());
    }

    // Apply
    match action {
        Action::Attack => self.do_attack(),
        Action::Defend => self.do_defend(),
        _ => {}
    }

    // Check
    self.check_game_over();

    Ok("Success".to_string())
}

// Check end conditions
fn is_game_over(&self) -> bool {
    self.player.health <= 0 || self.enemies.is_empty()
}

// Maintain invariants
fn apply_damage(&mut self, damage: i32) {
    self.health = (self.health - damage).max(0);
}
```

## Performance Tips

✓ Pre-validate once, not repeatedly
✓ Use remove_if for filtering collections
✓ Cache win condition checks
✓ Avoid cloning unless necessary

## Related Concepts

- Game loops (integrating with update loop)
- Random numbers (probability outcomes)
- Console I/O (displaying results)
- Collections (managing entities)
- Pattern matching (action handling)

