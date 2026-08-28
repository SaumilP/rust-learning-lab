# CLI and Console Games

This module uses small games to practise console input, state transitions,
loops, randomness, collections, and decomposing a program into testable parts.

## Contents

| Topic or project | Main idea | Status |
|---|---|---|
| `console_io` | Prompt for and validate terminal input | Example available |
| `random_numbers` | Generate values and model dice rolls | Example available |
| `game_loop_basics` | Separate setup, update, and exit conditions | Example available |
| `simple_game_logic` | Represent turn-based state and actions | Example available |
| `hangman` | Build an interactive word game | Cargo project available |
| `tic_tac_toe` | Model a board, turns, and win conditions | Cargo project available |
| `minesweeper`, `snake`, `roguelike_ascii` | Larger console-game directions | Planned scaffolds |
| `exercises` | Repair guessing, dice, Hangman, and memory games | Exercises available |

## Run a project

From this directory:

```bash
cargo run -p hangman
cargo run -p tic_tac_toe
```

The randomness example is a separate Cargo package:

```bash
cargo run --manifest-path random_numbers/Cargo.toml --bin dice_roller
```

## Validate the module

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Interactive programs become easier to test when input/output is kept at the
edge and game rules are implemented as functions over plain state. Prefer that
shape as you extend the projects.
