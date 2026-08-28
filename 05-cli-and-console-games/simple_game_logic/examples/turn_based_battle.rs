/// Turn-Based Battle - Game Logic Example
///
/// Demonstrates:
/// - Game state management with structs
/// - Turn-based game mechanics
/// - Win/lose condition checking
/// - Combat logic and validation
///
/// Run with: cargo run --example turn_based_battle
use std::io::{self, Write};

#[derive(Debug, PartialEq)]
enum BattleState {
    PlayerTurn,
    EnemyTurn,
    GameOver,
}

struct Character {
    name: String,
    health: i32,
    max_health: i32,
    damage: i32,
}

impl Character {
    fn new(name: &str, health: i32, damage: i32) -> Self {
        Character {
            name: name.to_string(),
            health,
            max_health: health,
            damage,
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn display_status(&self) {
        let bar_filled = (self.health as f64 / self.max_health as f64 * 20.0) as usize;
        let bar = "█".repeat(bar_filled) + &"░".repeat(20 - bar_filled);

        println!(
            "║ {:12} [{}] {:3}/{:3} HP ║",
            self.name, bar, self.health, self.max_health
        );
    }
}

struct Battle {
    player: Character,
    enemy: Character,
    turn: BattleState,
    round: u32,
}

impl Battle {
    fn new() -> Self {
        Battle {
            player: Character::new("Player", 30, 4),
            enemy: Character::new("Enemy", 25, 3),
            turn: BattleState::PlayerTurn,
            round: 1,
        }
    }

    fn display_status(&self) {
        println!("\n╔═══════════════════════════════════╗");
        println!("║ ROUND {}                          ║", self.round);
        println!("╠═══════════════════════════════════╣");
        self.player.display_status();
        println!("║ {:33} ║", " ");
        self.enemy.display_status();
        println!("╠═══════════════════════════════════╣");
        if self.turn == BattleState::PlayerTurn {
            println!("║ YOUR TURN                         ║");
        } else if self.turn == BattleState::EnemyTurn {
            println!("║ ENEMY TURN                        ║");
        }
        println!("╚═══════════════════════════════════╝");
    }

    fn player_attack(&mut self) {
        let damage = self.player.damage + rand::random() as i32 % 3 - 1;
        self.enemy.take_damage(damage);
        println!("\n⚔️  Player attacks for {} damage!", damage);

        if !self.enemy.is_alive() {
            println!("💀 Enemy defeated!");
            self.turn = BattleState::GameOver;
        } else {
            self.turn = BattleState::EnemyTurn;
        }
    }

    fn player_defend(&mut self) {
        println!("\n🛡️  Player takes defensive stance!");
        println!("   (Reduces incoming damage this turn)");
        self.turn = BattleState::EnemyTurn;
    }

    fn player_heal(&mut self) {
        let old_health = self.player.health;
        self.player.heal(8);
        let healed = self.player.health - old_health;
        println!("\n💚 Player heals for {} HP!", healed);
        self.turn = BattleState::EnemyTurn;
    }

    fn enemy_turn(&mut self) {
        // Simple AI: attack if enemy health > 50%, heal if low
        if self.enemy.health < self.enemy.max_health / 2 {
            let old_health = self.enemy.health;
            self.enemy.heal(6);
            let healed = self.enemy.health - old_health;
            println!("\n🩹 Enemy heals for {} HP!", healed);
        } else {
            let damage = self.enemy.damage + rand::random() as i32 % 2;
            self.player.take_damage(damage);
            println!("\n⚔️  Enemy attacks for {} damage!", damage);

            if !self.player.is_alive() {
                println!("💀 Player defeated!");
                self.turn = BattleState::GameOver;
            }
        }

        if self.turn != BattleState::GameOver {
            self.round += 1;
            self.turn = BattleState::PlayerTurn;
        }
    }

    fn get_player_action(&self) -> Option<char> {
        println!("\n╔════════════════════╗");
        println!("║ ACTIONS            ║");
        println!("║ a) Attack          ║");
        println!("║ d) Defend          ║");
        println!("║ h) Heal            ║");
        println!("║ q) Quit Battle     ║");
        println!("╚════════════════════╝");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().chars().next()
    }

    fn process_player_action(&mut self, action: char) {
        match action {
            'a' => self.player_attack(),
            'd' => self.player_defend(),
            'h' => self.player_heal(),
            'q' => {
                println!("\nBattle abandoned!");
                self.turn = BattleState::GameOver;
            }
            _ => println!("Invalid action!"),
        }
    }

    fn run(&mut self) {
        println!("╔═════════════════════════════════╗");
        println!("║ TURN-BASED BATTLE SYSTEM        ║");
        println!("╚═════════════════════════════════╝");

        while self.turn != BattleState::GameOver {
            self.display_status();

            if self.turn == BattleState::PlayerTurn {
                if let Some(action) = self.get_player_action() {
                    self.process_player_action(action);
                }
            } else if self.turn == BattleState::EnemyTurn {
                println!("\nEnemy is thinking...");
                std::thread::sleep(std::time::Duration::from_millis(800));
                self.enemy_turn();
            }
        }

        println!("\n╔═════════════════════════════════╗");
        println!("║ BATTLE RESULT                   ║");
        if self.player.is_alive() {
            println!("║ VICTORY! You defeated the enemy!║");
            println!("║ Final HP: {}                      ║", self.player.health);
        } else {
            println!("║ DEFEAT! You were defeated.      ║");
            println!("║ Rounds lasted: {}                ║", self.round);
        }
        println!("╚═════════════════════════════════╝");
    }
}

// Simple random number generation without external crate
mod rand {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn random() -> u32 {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        (duration.as_nanos() as u32) % 256
    }
}

fn main() {
    let mut battle = Battle::new();
    battle.run();
}
