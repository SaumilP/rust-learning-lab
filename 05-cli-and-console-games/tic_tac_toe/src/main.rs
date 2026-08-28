// Tic-Tac-Toe Game in Rust
// Demonstrates game logic, AI, state management, and user interaction

use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn other(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn symbol(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Taken(Player),
}

impl Cell {
    fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Taken(player) => write!(f, "{}", player.symbol()),
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [[Cell::Empty; 3]; 3],
        }
    }

    fn display(&self) {
        println!("\n     1   2   3");
        println!("   ╔═══╦═══╦═══╗");
        for (row_idx, row) in self.cells.iter().enumerate() {
            println!(" {} ║ {} ║ {} ║ {} ║", row_idx + 1, row[0], row[1], row[2]);
            if row_idx < 2 {
                println!("   ╠═══╬═══╬═══╣");
            }
        }
        println!("   ╚═══╩═══╩═══╝\n");
    }

    fn make_move(&mut self, row: usize, col: usize, player: Player) -> bool {
        if row < 3 && col < 3 && self.cells[row][col].is_empty() {
            self.cells[row][col] = Cell::Taken(player);
            true
        } else {
            false
        }
    }

    fn check_winner(&self) -> Option<Player> {
        // Check rows
        for row in &self.cells {
            if let Cell::Taken(player) = row[0] {
                if row[1] == row[0] && row[2] == row[0] {
                    return Some(player);
                }
            }
        }

        // Check columns
        for col in 0..3 {
            if let Cell::Taken(player) = self.cells[0][col] {
                if self.cells[1][col] == self.cells[0][col]
                    && self.cells[2][col] == self.cells[0][col]
                {
                    return Some(player);
                }
            }
        }

        // Check diagonals
        if let Cell::Taken(player) = self.cells[1][1] {
            // Top-left to bottom-right
            if self.cells[0][0] == self.cells[1][1] && self.cells[2][2] == self.cells[1][1] {
                return Some(player);
            }
            // Top-right to bottom-left
            if self.cells[0][2] == self.cells[1][1] && self.cells[2][0] == self.cells[1][1] {
                return Some(player);
            }
        }

        None
    }

    fn is_full(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|cell| !cell.is_empty()))
    }

    fn available_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for (row_idx, row) in self.cells.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if cell.is_empty() {
                    moves.push((row_idx, col_idx));
                }
            }
        }
        moves
    }
}

struct Game {
    board: Board,
    current_player: Player,
    mode: GameMode,
}

enum GameMode {
    HumanVsHuman,
    HumanVsComputer,
}

impl Game {
    fn new(mode: GameMode) -> Self {
        Game {
            board: Board::new(),
            current_player: Player::X,
            mode,
        }
    }

    fn run(&mut self) {
        println!("\n=== Tic-Tac-Toe ===\n");

        loop {
            self.board.display();

            // Check for winner or draw
            if let Some(winner) = self.board.check_winner() {
                println!("🎉 Player {} wins!", winner.symbol());
                break;
            }

            if self.board.is_full() {
                println!("🤝 It's a draw!");
                break;
            }

            // Current player's turn
            println!("Player {}'s turn", self.current_player.symbol());

            let (row, col) = match self.mode {
                GameMode::HumanVsHuman => self.get_human_move(),
                GameMode::HumanVsComputer => {
                    if self.current_player == Player::X {
                        self.get_human_move()
                    } else {
                        println!("Computer is thinking...");
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        self.get_computer_move()
                    }
                }
            };

            if self.board.make_move(row, col, self.current_player) {
                self.current_player = self.current_player.other();
            } else {
                println!("Invalid move! Try again.");
            }
        }

        println!("\nFinal board:");
        self.board.display();
    }

    fn get_human_move(&self) -> (usize, usize) {
        loop {
            print!("Enter row and column (1-3, e.g., '1 2'): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() != 2 {
                println!("Please enter two numbers separated by space.");
                continue;
            }

            let row: Result<usize, _> = parts[0].parse();
            let col: Result<usize, _> = parts[1].parse();

            match (row, col) {
                (Ok(r), Ok(c)) if (1..=3).contains(&r) && (1..=3).contains(&c) => {
                    return (r - 1, c - 1);
                }
                _ => {
                    println!("Please enter numbers between 1 and 3.");
                }
            }
        }
    }

    fn get_computer_move(&self) -> (usize, usize) {
        // Simple AI: Try to win, block opponent, or take center/corner

        // Try to win
        if let Some(mv) = self.find_winning_move(self.current_player) {
            println!("Computer plays: {} {}", mv.0 + 1, mv.1 + 1);
            return mv;
        }

        // Block opponent
        let opponent = self.current_player.other();
        if let Some(mv) = self.find_winning_move(opponent) {
            println!("Computer blocks: {} {}", mv.0 + 1, mv.1 + 1);
            return mv;
        }

        // Take center if available
        if self.board.cells[1][1].is_empty() {
            println!("Computer takes center: 2 2");
            return (1, 1);
        }

        // Take a corner
        let corners = [(0, 0), (0, 2), (2, 0), (2, 2)];
        for &(r, c) in &corners {
            if self.board.cells[r][c].is_empty() {
                println!("Computer takes corner: {} {}", r + 1, c + 1);
                return (r, c);
            }
        }

        // Take any available move
        let available = self.board.available_moves();
        let mv = available[0];
        println!("Computer plays: {} {}", mv.0 + 1, mv.1 + 1);
        mv
    }

    fn find_winning_move(&self, player: Player) -> Option<(usize, usize)> {
        for &(row, col) in &self.board.available_moves() {
            let mut test_board = self.board.clone();
            test_board.make_move(row, col, player);
            if test_board.check_winner() == Some(player) {
                return Some((row, col));
            }
        }
        None
    }
}

fn main() {
    println!("Welcome to Tic-Tac-Toe!");
    println!("\nSelect game mode:");
    println!("1. Human vs Human");
    println!("2. Human vs Computer");

    print!("\nEnter choice (1 or 2): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let mode = match choice.trim() {
        "1" => GameMode::HumanVsHuman,
        "2" => GameMode::HumanVsComputer,
        _ => {
            println!("Invalid choice. Defaulting to Human vs Computer.");
            GameMode::HumanVsComputer
        }
    };

    let mut game = Game::new(mode);
    game.run();

    println!("\nThanks for playing! 🎮");
}

/*
Tic-Tac-Toe Summary:
====================

FEATURES:
- Human vs Human mode
- Human vs Computer mode
- Simple AI opponent
- Win detection (rows, cols, diagonals)
- Draw detection
- Input validation
- Clean board display

CONCEPTS DEMONSTRATED:
- Enums (Player, Cell, GameMode)
- Structs (Board, Game)
- Pattern matching
- Game state management
- AI logic (minimax-lite)
- User input handling
- Clone trait

GAME LOGIC:
1. Display board
2. Check for winner/draw
3. Get current player's move
4. Validate and apply move
5. Switch players
6. Repeat

AI STRATEGY:
1. Try to win (find winning move)
2. Block opponent (find their winning move)
3. Take center if available
4. Take corner
5. Take any remaining space

IMPROVEMENTS:
- Minimax algorithm for perfect AI
- Alpha-beta pruning
- Difficulty levels
- Score tracking
- Undo move
- Save/load game
- Network multiplayer
- GUI version

RUN THIS:
    cargo run

PLAY:
    1. Choose game mode
    2. Enter row and column (1-3)
    3. Try to get three in a row!

EXAMPLE:
    Enter row and column: 1 1
    Enter row and column: 2 2
    Enter row and column: 1 2

KEY LEARNINGS:
- Game state management
- Win condition checking
- AI implementation
- User input validation
- Clean code structure

CODE STRUCTURE:
- Player:   Enum for X and O
- Cell:     Board cell state
- Board:    3x3 grid with game logic
- Game:     Main game controller
- GameMode: Human vs Human/Computer

COMPARISON TO OTHER LANGUAGES:
Python:  Lists for board, simpler syntax
Java:    Classes and objects
JavaScript: Objects and arrays
Rust:    Enums and pattern matching (safer!)

NEXT STEPS:
- Implement minimax algorithm
- Add difficulty levels
- Create graphical version
- Add network play
- Implement other games (Connect Four, Chess)
*/
