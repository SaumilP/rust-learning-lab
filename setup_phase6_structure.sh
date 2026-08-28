#!/bin/bash

# Phase 6 Implementation Script
# Rust Learning Lab - Modules 04-11
# Creates directory structures and placeholder documentation

set -e  # Exit on error

BASE_PATH="/home/admin-and/code/personal/rust-learning-lab"

echo "=========================================="
echo "Phase 6: Module Structure Setup"
echo "=========================================="
echo ""

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to create directory
create_dir() {
    local dir="$1"
    if [ ! -d "$dir" ]; then
        mkdir -p "$dir"
        echo -e "${GREEN}✓${NC} Created: $dir"
    else
        echo -e "${YELLOW}⊳${NC} Exists: $dir"
    fi
}

# Function to create file if it doesn't exist
create_file() {
    local file="$1"
    local content="$2"
    if [ ! -f "$file" ]; then
        echo "$content" > "$file"
        echo -e "${GREEN}✓${NC} Created: $file"
    else
        echo -e "${YELLOW}⊳${NC} Exists: $file"
    fi
}

# ============================================================================
# MODULE 04: SIMPLE PROGRAMS
# ============================================================================

echo -e "${BLUE}Module 04: Simple Programs${NC}"
echo "----------------------------"

# Concept directories (already exist, but ensuring completeness)
create_dir "$BASE_PATH/04-simple-programs/cli_arguments/examples"
create_dir "$BASE_PATH/04-simple-programs/file_io_basics/examples"
create_dir "$BASE_PATH/04-simple-programs/simple_algorithms/examples"
create_dir "$BASE_PATH/04-simple-programs/text_processing/examples"

# .gitkeep files
touch "$BASE_PATH/04-simple-programs/cli_arguments/examples/.gitkeep"
touch "$BASE_PATH/04-simple-programs/file_io_basics/examples/.gitkeep"
touch "$BASE_PATH/04-simple-programs/simple_algorithms/examples/.gitkeep"
touch "$BASE_PATH/04-simple-programs/text_processing/examples/.gitkeep"

# file_io_basics documentation
create_file "$BASE_PATH/04-simple-programs/file_io_basics/README.md" "# File I/O Basics

## Overview
Learn fundamental file operations in Rust using the \`std::fs\` module.

## Topics Covered
- Reading files: \`read_to_string()\`, \`read()\`
- Writing files: \`write()\`, \`write_all()\`
- Buffered I/O: \`BufReader\`, \`BufWriter\`
- Error handling with \`Result<T, E>\`
- File paths and path manipulation

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/04-simple-programs/file_io_basics/key_takeaways.md" "# Key Takeaways: File I/O Basics

## Essential Concepts
1. **Always handle errors** - File operations return \`Result<T, E>\`
2. **Use buffered I/O** - More efficient for large files
3. **Close files automatically** - RAII pattern handles cleanup
4. **Path manipulation** - Use \`std::path::Path\` for cross-platform paths

## Common Patterns
- \`read_to_string()\` for text files
- \`BufReader\` for line-by-line reading
- \`BufWriter\` for efficient writing
- Error propagation with \`?\` operator

## Best Practices
- Use \`?:\` for error handling
- Prefer buffered I/O for large files
- Check file existence before operations
- Handle different error types appropriately
"

# simple_algorithms documentation
create_file "$BASE_PATH/04-simple-programs/simple_algorithms/README.md" "# Simple Algorithms

## Overview
Fundamental algorithms for sorting, searching, and data manipulation in Rust.

## Topics Covered
- Sorting: bubble sort, insertion sort, selection sort
- Searching: linear search, binary search
- Array manipulation: merging, filtering, transforming
- Algorithm complexity basics (Big O)

## Learning Objectives
- Implement classic algorithms in Rust
- Understand time and space complexity
- Use Rust's iterator methods effectively
- Compare manual implementations vs standard library

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/04-simple-programs/simple_algorithms/key_takeaways.md" "# Key Takeaways: Simple Algorithms

## Essential Concepts
1. **Time complexity matters** - Choose algorithms based on data size
2. **Rust's iterators are powerful** - Often better than manual loops
3. **Binary search requires sorted data** - O(log n) vs O(n)
4. **In-place vs copying** - Consider memory usage

## Common Patterns
- Iterators for data processing
- Slices for array sections
- Generics for reusable algorithms
- Pattern matching for algorithm logic

## Best Practices
- Use standard library when possible
- Benchmark before optimizing
- Document time/space complexity
- Write tests for edge cases
"

# text_processing documentation
create_file "$BASE_PATH/04-simple-programs/text_processing/README.md" "# Text Processing

## Overview
Techniques for manipulating and analyzing text data in Rust.

## Topics Covered
- String vs &str
- Line-by-line processing
- Word counting and frequency analysis
- Text filtering and pattern matching
- CSV and structured text parsing
- Regular expressions basics

## Learning Objectives
- Master string handling in Rust
- Process large text files efficiently
- Implement text analysis algorithms
- Use regex for pattern matching

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/04-simple-programs/text_processing/key_takeaways.md" "# Key Takeaways: Text Processing

## Essential Concepts
1. **String ownership** - Understand String vs &str trade-offs
2. **Line processing** - Use \`lines()\` iterator for efficiency
3. **Whitespace handling** - \`trim()\`, \`split_whitespace()\`
4. **Unicode awareness** - Rust strings are UTF-8

## Common Patterns
- \`lines()\` for line-by-line processing
- \`split()\` and \`split_whitespace()\` for tokenization
- \`collect()\` to build strings/vectors
- HashMap for frequency counting

## Best Practices
- Avoid unnecessary allocations
- Use iterators for lazy evaluation
- Handle edge cases (empty lines, special chars)
- Consider regex crate for complex patterns
"

# Exercise directories
for i in {1..4}; do
    case $i in
        1) name="word_counter" ;;
        2) name="simple_search" ;;
        3) name="text_filter" ;;
        4) name="merge_sorted_files" ;;
    esac

    create_dir "$BASE_PATH/04-simple-programs/exercises/exercise_${i}_${name}/src"
    create_dir "$BASE_PATH/04-simple-programs/exercises/exercise_${i}_${name}/tests"
    create_dir "$BASE_PATH/04-simple-programs/exercises/exercise_${i}_${name}/solution"

    create_file "$BASE_PATH/04-simple-programs/exercises/exercise_${i}_${name}/README.md" "# Exercise $i: ${name//_/ }

## Objective
[Detailed exercise description]

## Requirements
- [Requirement 1]
- [Requirement 2]
- [Requirement 3]

## Skills Practiced
- [Skill 1]
- [Skill 2]

## Bonus Challenges
- [Challenge 1]
- [Challenge 2]
"

    create_file "$BASE_PATH/04-simple-programs/exercises/exercise_${i}_${name}/Cargo.toml" "[package]
name = \"exercise_${i}_${name}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
"
done

echo -e "${GREEN}✓ Module 04 structure complete${NC}"
echo ""

# ============================================================================
# MODULE 05: CLI AND CONSOLE GAMES
# ============================================================================

echo -e "${BLUE}Module 05: CLI and Console Games${NC}"
echo "--------------------------------"

# Concept directories
create_dir "$BASE_PATH/05-cli-and-console-games/game_loop_basics/examples"
create_dir "$BASE_PATH/05-cli-and-console-games/random_numbers/examples"
create_dir "$BASE_PATH/05-cli-and-console-games/console_io/examples"
create_dir "$BASE_PATH/05-cli-and-console-games/simple_game_logic/examples"

# .gitkeep files
touch "$BASE_PATH/05-cli-and-console-games/game_loop_basics/examples/.gitkeep"
touch "$BASE_PATH/05-cli-and-console-games/random_numbers/examples/.gitkeep"
touch "$BASE_PATH/05-cli-and-console-games/console_io/examples/.gitkeep"
touch "$BASE_PATH/05-cli-and-console-games/simple_game_logic/examples/.gitkeep"

# game_loop_basics documentation
create_file "$BASE_PATH/05-cli-and-console-games/game_loop_basics/README.md" "# Game Loop Basics

## Overview
The game loop is the heart of any game, controlling the flow of initialization, update, and rendering.

## Topics Covered
- Game loop pattern: init → update → render → repeat
- Event-driven vs continuous loops
- Frame rate control and timing
- State management in loops
- Input processing

## Learning Objectives
- Implement basic game loops
- Manage game state effectively
- Control timing and frame rates
- Handle user input in loops

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/05-cli-and-console-games/game_loop_basics/key_takeaways.md" "# Key Takeaways: Game Loop Basics

## Essential Concepts
1. **Loop structure** - Init, update, render cycle
2. **State management** - Track game state between frames
3. **Timing** - Control frame rate and delta time
4. **Exit conditions** - Clean loop termination

## Common Patterns
- \`loop\` for infinite game loops
- State enums for game phases
- Input polling vs event handling
- Delta time for frame-independent updates

## Best Practices
- Separate update and render logic
- Use delta time for smooth animations
- Handle exit gracefully
- Keep state immutable where possible
"

# random_numbers documentation
create_file "$BASE_PATH/05-cli-and-console-games/random_numbers/README.md" "# Random Numbers

## Overview
Generating random numbers and making random choices using the \`rand\` crate.

## Topics Covered
- Using \`rand\` crate
- Generating random integers: \`gen()\`, \`gen_range()\`
- Random choices: \`choose()\`, \`shuffle()\`
- RNG seeding for reproducibility
- Weighted random selection
- Distribution types

## Learning Objectives
- Generate random numbers in various ranges
- Make random choices from collections
- Control randomness with seeding
- Implement probability-based logic

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/05-cli-and-console-games/random_numbers/key_takeaways.md" "# Key Takeaways: Random Numbers

## Essential Concepts
1. **rand crate** - Standard for random number generation
2. **gen_range()** - Most common for bounded random values
3. **Seeding** - Control randomness for testing/replay
4. **Distributions** - Uniform, weighted, normal, etc.

## Common Patterns
- \`thread_rng()\` for quick random generation
- \`gen_range(min..max)\` for bounded values
- \`choose()\` for random element selection
- Seeding for deterministic behavior

## Best Practices
- Use \`thread_rng()\` for simple cases
- Seed RNG for reproducible results
- Consider distributions for realistic randomness
- Test edge cases (min/max values)
"

# console_io documentation
create_file "$BASE_PATH/05-cli-and-console-games/console_io/README.md" "# Console I/O

## Overview
Techniques for reading input and displaying output in terminal-based games.

## Topics Covered
- Reading user input: \`stdin().read_line()\`
- Formatted output: \`print!()\`, \`println!()\`, \`format!()\`
- ANSI color codes
- Clear screen techniques
- Menu system patterns
- Input validation

## Learning Objectives
- Read and validate user input
- Create colorful, formatted output
- Build interactive menus
- Clear and refresh console

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/05-cli-and-console-games/console_io/key_takeaways.md" "# Key Takeaways: Console I/O

## Essential Concepts
1. **Input reading** - \`stdin().read_line()\` requires mutable String
2. **Output formatting** - Use \`println!()\` with format specifiers
3. **ANSI codes** - Color and style terminal text
4. **Input validation** - Always validate user input

## Common Patterns
- Read-trim-parse for input processing
- Menu loops with match expressions
- ANSI escape codes for colors
- Clear screen with ANSI or \`print!(\"\\x1B[2J\")\`

## Best Practices
- Trim input before parsing
- Provide clear prompts
- Handle invalid input gracefully
- Use crates like \`crossterm\` for advanced features
"

# simple_game_logic documentation
create_file "$BASE_PATH/05-cli-and-console-games/simple_game_logic/README.md" "# Simple Game Logic

## Overview
Implementing game rules, state management, and basic AI for console games.

## Topics Covered
- Game state representation (structs, enums)
- Turn-based game flow
- Scoring and statistics tracking
- Win/loss condition checking
- Simple AI strategies
- Game rules implementation

## Learning Objectives
- Design game state structures
- Implement game rules
- Check win/loss conditions
- Create simple AI opponents

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/05-cli-and-console-games/simple_game_logic/key_takeaways.md" "# Key Takeaways: Simple Game Logic

## Essential Concepts
1. **State representation** - Use structs and enums
2. **Rule enforcement** - Validate moves before applying
3. **Win conditions** - Check after each move
4. **AI strategies** - Random, heuristic, minimax

## Common Patterns
- Enum for game states (Menu, Playing, GameOver)
- Struct for game data (board, score, players)
- Methods for game operations (move, check_win)
- Pattern matching for move validation

## Best Practices
- Keep state immutable where possible
- Validate all inputs
- Separate game logic from I/O
- Test win conditions thoroughly
"

# Exercise directories
for i in {1..4}; do
    case $i in
        1) name="higher_lower" ;;
        2) name="dice_roller" ;;
        3) name="hangman" ;;
        4) name="memory_game" ;;
    esac

    create_dir "$BASE_PATH/05-cli-and-console-games/exercises/exercise_${i}_${name}/src"
    create_dir "$BASE_PATH/05-cli-and-console-games/exercises/exercise_${i}_${name}/tests"
    create_dir "$BASE_PATH/05-cli-and-console-games/exercises/exercise_${i}_${name}/solution"

    create_file "$BASE_PATH/05-cli-and-console-games/exercises/exercise_${i}_${name}/README.md" "# Exercise $i: ${name//_/ }

## Objective
[Detailed exercise description]

## Requirements
- [Requirement 1]
- [Requirement 2]
- [Requirement 3]

## Skills Practiced
- [Skill 1]
- [Skill 2]

## Bonus Challenges
- [Challenge 1]
- [Challenge 2]
"

    create_file "$BASE_PATH/05-cli-and-console-games/exercises/exercise_${i}_${name}/Cargo.toml" "[package]
name = \"exercise_${i}_${name}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
rand = \"0.8\"
"
done

echo -e "${GREEN}✓ Module 05 structure complete${NC}"
echo ""

# ============================================================================
# MODULE 06: INTERMEDIATE RUST
# ============================================================================

echo -e "${BLUE}Module 06: Intermediate Rust${NC}"
echo "-----------------------------"

# Concept directories (new ones)
create_dir "$BASE_PATH/06-intermediate-rust/ownership_and_borrowing/examples"
create_dir "$BASE_PATH/06-intermediate-rust/traits_and_polymorphism/examples"
create_dir "$BASE_PATH/06-intermediate-rust/generics/examples"
create_dir "$BASE_PATH/06-intermediate-rust/enums_and_pattern_matching/examples"
create_dir "$BASE_PATH/06-intermediate-rust/lifetimes/examples"

# .gitkeep files
touch "$BASE_PATH/06-intermediate-rust/ownership_and_borrowing/examples/.gitkeep"
touch "$BASE_PATH/06-intermediate-rust/traits_and_polymorphism/examples/.gitkeep"
touch "$BASE_PATH/06-intermediate-rust/generics/examples/.gitkeep"
touch "$BASE_PATH/06-intermediate-rust/enums_and_pattern_matching/examples/.gitkeep"
touch "$BASE_PATH/06-intermediate-rust/lifetimes/examples/.gitkeep"

# ownership_and_borrowing documentation
create_file "$BASE_PATH/06-intermediate-rust/ownership_and_borrowing/README.md" "# Ownership and Borrowing

## Overview
Rust's ownership system is its most distinctive feature, enabling memory safety without garbage collection.

## Topics Covered
- Three ownership rules
- Move semantics and copying
- Stack vs heap allocation
- Borrowing rules (immutable and mutable)
- The borrow checker
- Reference scope and lifetimes
- Slice references

## Learning Objectives
- Understand ownership transfer
- Use references correctly
- Avoid borrowing errors
- Master the borrow checker

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/06-intermediate-rust/ownership_and_borrowing/key_takeaways.md" "# Key Takeaways: Ownership and Borrowing

## Essential Concepts
1. **Each value has one owner** - Ownership is unique
2. **Move by default** - Non-Copy types move on assignment
3. **Borrowing rules** - Many immutable OR one mutable reference
4. **References have scope** - Can't outlive the data they reference

## Common Patterns
- \`&T\` for immutable borrows
- \`&mut T\` for mutable borrows
- \`.clone()\` for explicit copying
- Slices (\`&[T]\`) for portions of collections

## Best Practices
- Prefer borrowing over ownership transfer
- Use immutable references by default
- Clone only when necessary
- Understand when types implement Copy
"

# traits_and_polymorphism documentation
create_file "$BASE_PATH/06-intermediate-rust/traits_and_polymorphism/README.md" "# Traits and Polymorphism

## Overview
Traits define shared behavior across types, enabling polymorphism in Rust.

## Topics Covered
- Defining traits
- Implementing traits for types
- Trait bounds and constraints
- Trait objects (\`dyn Trait\`)
- Default implementations
- Associated types and functions
- Orphan rule
- Deriving traits

## Learning Objectives
- Define custom traits
- Implement traits for various types
- Use trait bounds in generic code
- Understand trait objects vs static dispatch

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/06-intermediate-rust/traits_and_polymorphism/key_takeaways.md" "# Key Takeaways: Traits and Polymorphism

## Essential Concepts
1. **Traits define behavior** - Like interfaces in other languages
2. **Static dispatch** - Trait bounds, zero-cost abstraction
3. **Dynamic dispatch** - Trait objects (\`dyn Trait\`), runtime cost
4. **Orphan rule** - Implement trait on type only if you own one

## Common Patterns
- \`impl Trait\` for trait bounds in parameters
- \`&dyn Trait\` for trait objects
- \`#[derive(...)]\` for automatic implementations
- Associated types for type relationships

## Best Practices
- Prefer static dispatch when possible
- Use trait objects for heterogeneous collections
- Provide default implementations when appropriate
- Document trait contracts clearly
"

# generics documentation
create_file "$BASE_PATH/06-intermediate-rust/generics/README.md" "# Generics

## Overview
Generic programming allows writing flexible, reusable code that works with multiple types.

## Topics Covered
- Generic functions
- Generic structs and enums
- Type parameters and constraints
- Multiple type parameters
- Const generics (array sizes)
- Where clauses for complex bounds
- Monomorphization and zero-cost abstractions

## Learning Objectives
- Write generic functions and types
- Apply trait bounds to generics
- Use const generics for array sizes
- Understand monomorphization

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/06-intermediate-rust/generics/key_takeaways.md" "# Key Takeaways: Generics

## Essential Concepts
1. **Type parameters** - \`<T>\` represents generic types
2. **Trait bounds** - \`<T: Trait>\` constrains generic types
3. **Monomorphization** - Compiler generates code for each type
4. **Zero cost** - Generics have no runtime overhead

## Common Patterns
- \`<T>\` for single type parameter
- \`<T: Trait>\` for trait bounds
- \`<T, U>\` for multiple type parameters
- \`where T: Trait\` for complex bounds

## Best Practices
- Use meaningful type parameter names
- Apply minimum necessary trait bounds
- Use where clauses for readability
- Leverage type inference when possible
"

# enums_and_pattern_matching documentation
create_file "$BASE_PATH/06-intermediate-rust/enums_and_pattern_matching/README.md" "# Enums and Pattern Matching

## Overview
Enums represent data that can be one of several variants, and pattern matching provides exhaustive handling.

## Topics Covered
- Enum variants with and without data
- Option<T> and Result<T, E> in depth
- Match expressions and exhaustiveness
- Pattern matching syntax
- If let and while let
- Destructuring patterns
- Match guards

## Learning Objectives
- Define and use enums effectively
- Master pattern matching
- Use Option and Result idiomatically
- Apply destructuring patterns

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/06-intermediate-rust/enums_and_pattern_matching/key_takeaways.md" "# Key Takeaways: Enums and Pattern Matching

## Essential Concepts
1. **Enums for variants** - Model data with multiple possibilities
2. **Match is exhaustive** - Compiler ensures all cases handled
3. **Option for nullable** - Safer than null pointers
4. **Result for errors** - Explicit error handling

## Common Patterns
- \`match\` for exhaustive handling
- \`if let\` for single pattern match
- \`while let\` for iterating with patterns
- \`_\` wildcard for catch-all

## Best Practices
- Prefer Option over nullable patterns
- Use Result for recoverable errors
- Match all variants explicitly when possible
- Use guards for additional conditions
"

# lifetimes documentation
create_file "$BASE_PATH/06-intermediate-rust/lifetimes/README.md" "# Lifetimes

## Overview
Lifetime annotations ensure references are always valid, preventing dangling pointers at compile time.

## Topics Covered
- Lifetime annotations syntax
- Lifetime in function signatures
- Lifetime in struct definitions
- Lifetime elision rules
- Static lifetime
- Multiple lifetimes

## Learning Objectives
- Understand lifetime annotations
- Apply lifetimes in functions and structs
- Recognize lifetime elision
- Debug lifetime errors

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/06-intermediate-rust/lifetimes/key_takeaways.md" "# Key Takeaways: Lifetimes

## Essential Concepts
1. **Lifetimes are compile-time** - No runtime overhead
2. **Prevent dangling references** - References can't outlive data
3. **Elision rules** - Compiler infers lifetimes in common cases
4. **'static lifetime** - Lives for entire program

## Common Patterns
- \`'a\` for lifetime parameters
- \`&'a T\` for references with lifetime
- \`&'static\` for program-duration references
- Multiple lifetimes when needed

## Best Practices
- Let compiler infer when possible
- Use descriptive lifetime names
- Start with simple cases
- Understand error messages
"

# Exercise directories
for i in {1..4}; do
    case $i in
        1) name="custom_traits" ;;
        2) name="generic_functions" ;;
        3) name="complex_patterns" ;;
        4) name="borrow_checker" ;;
    esac

    create_dir "$BASE_PATH/06-intermediate-rust/exercises/exercise_${i}_${name}/src"
    create_dir "$BASE_PATH/06-intermediate-rust/exercises/exercise_${i}_${name}/tests"
    create_dir "$BASE_PATH/06-intermediate-rust/exercises/exercise_${i}_${name}/solution"

    create_file "$BASE_PATH/06-intermediate-rust/exercises/exercise_${i}_${name}/README.md" "# Exercise $i: ${name//_/ }

## Objective
[Detailed exercise description]

## Requirements
- [Requirement 1]
- [Requirement 2]
- [Requirement 3]

## Skills Practiced
- [Skill 1]
- [Skill 2]

## Bonus Challenges
- [Challenge 1]
- [Challenge 2]
"

    create_file "$BASE_PATH/06-intermediate-rust/exercises/exercise_${i}_${name}/Cargo.toml" "[package]
name = \"exercise_${i}_${name}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
"
done

echo -e "${GREEN}✓ Module 06 structure complete${NC}"
echo ""

# ============================================================================
# MODULE 08: DESIGN PATTERNS
# ============================================================================

echo -e "${BLUE}Module 08: Design Patterns${NC}"
echo "---------------------------"

# Category READMEs
create_file "$BASE_PATH/08-design-patterns/creational_patterns/README.md" "# Creational Patterns

## Overview
Creational patterns deal with object creation mechanisms, providing flexibility in how objects are created.

## Patterns in This Category

### Builder Pattern
Construct complex objects step by step.
- Separate construction from representation
- Same construction process can create different representations
- Useful for objects with many optional parameters

### Factory Pattern
Create objects without specifying exact classes.
- Encapsulate object creation
- Return trait objects for polymorphism
- Useful when type is determined at runtime

### Singleton Pattern
Ensure a class has only one instance.
- Global access point
- Thread-safe initialization
- Use lazy_static or once_cell in Rust

## See Also
- Individual pattern directories
- Examples in each pattern's examples/ folder
"

create_file "$BASE_PATH/08-design-patterns/structural_patterns/README.md" "# Structural Patterns

## Overview
Structural patterns deal with object composition, creating relationships between entities.

## Patterns in This Category

### Adapter Pattern
Make incompatible interfaces work together.
- Convert one interface to another
- Use traits for interface definition
- Wrapper types for adaptation

### Decorator Pattern
Dynamically add behavior to objects.
- Attach additional responsibilities
- Flexible alternative to subclassing
- Use composition over inheritance

### Facade Pattern
Provide simplified interface to complex subsystem.
- Hide complexity behind simple interface
- Reduce dependencies on internal implementation
- Single entry point for multiple operations

## See Also
- Individual pattern directories
- Examples in each pattern's examples/ folder
"

create_file "$BASE_PATH/08-design-patterns/behavioral_patterns/README.md" "# Behavioral Patterns

## Overview
Behavioral patterns deal with object collaboration and communication.

## Patterns in This Category

### Observer Pattern
Notify multiple objects about state changes.
- Define one-to-many dependency
- Use channels or callbacks
- Decouple event sources from handlers

### Strategy Pattern
Encapsulate interchangeable algorithms.
- Define family of algorithms
- Make them interchangeable
- Use traits or enums in Rust

### Command Pattern
Encapsulate requests as objects.
- Parameterize clients with different requests
- Queue or log requests
- Support undo operations

### State Pattern
Alter behavior based on internal state.
- State-specific behavior
- Type-state pattern in Rust
- Compile-time state guarantees

## See Also
- Individual pattern directories
- Examples in each pattern's examples/ folder
"

# Exercise directories
for i in {1..4}; do
    case $i in
        1) name="builder_pattern" ;;
        2) name="adapter_pattern" ;;
        3) name="observer_pattern" ;;
        4) name="state_machine" ;;
    esac

    create_dir "$BASE_PATH/08-design-patterns/exercises/exercise_${i}_${name}/src"
    create_dir "$BASE_PATH/08-design-patterns/exercises/exercise_${i}_${name}/tests"
    create_dir "$BASE_PATH/08-design-patterns/exercises/exercise_${i}_${name}/solution"

    create_file "$BASE_PATH/08-design-patterns/exercises/exercise_${i}_${name}/README.md" "# Exercise $i: ${name//_/ }

## Objective
[Detailed exercise description]

## Requirements
- [Requirement 1]
- [Requirement 2]
- [Requirement 3]

## Skills Practiced
- [Skill 1]
- [Skill 2]

## Bonus Challenges
- [Challenge 1]
- [Challenge 2]
"

    create_file "$BASE_PATH/08-design-patterns/exercises/exercise_${i}_${name}/Cargo.toml" "[package]
name = \"exercise_${i}_${name}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
"
done

echo -e "${GREEN}✓ Module 08 structure complete${NC}"
echo ""

# ============================================================================
# MODULE 09: MINI-PROJECTS
# ============================================================================

echo -e "${BLUE}Module 09: Mini-Projects${NC}"
echo "-------------------------"

# Create standardized structure for each project
for project in todo_cli calculator note_taker weather_cli; do
    create_dir "$BASE_PATH/09-mini-projects/$project/examples"
    create_dir "$BASE_PATH/09-mini-projects/$project/exercises"
    create_dir "$BASE_PATH/09-mini-projects/$project/tests"
    create_dir "$BASE_PATH/09-mini-projects/$project/data"

    # Requirements file
    create_file "$BASE_PATH/09-mini-projects/$project/requirements.md" "# Requirements: ${project//_/ }

## Functional Requirements
1. [Core functionality 1]
2. [Core functionality 2]
3. [Core functionality 3]

## Non-Functional Requirements
- Performance: [Requirements]
- Usability: [Requirements]
- Reliability: [Requirements]

## User Stories
- As a user, I want to [action] so that [benefit]
- As a user, I want to [action] so that [benefit]

## Technical Requirements
- Rust version: [Minimum version]
- Dependencies: [List key dependencies]
- Platform: [Target platforms]

## Acceptance Criteria
- [ ] [Criterion 1]
- [ ] [Criterion 2]
- [ ] [Criterion 3]
"
done

echo -e "${GREEN}✓ Module 09 structure complete${NC}"
echo ""

# ============================================================================
# MODULE 10: REAL-WORLD RUST
# ============================================================================

echo -e "${BLUE}Module 10: Real-World Rust${NC}"
echo "---------------------------"

# Concept directories
create_dir "$BASE_PATH/10-real-world-rust/project_organization/examples"
create_dir "$BASE_PATH/10-real-world-rust/performance_optimization/examples"
create_dir "$BASE_PATH/10-real-world-rust/deployment/examples"
create_dir "$BASE_PATH/10-real-world-rust/concurrency_intro/examples"
create_dir "$BASE_PATH/10-real-world-rust/error_handling_advanced/examples"

# .gitkeep files
touch "$BASE_PATH/10-real-world-rust/project_organization/examples/.gitkeep"
touch "$BASE_PATH/10-real-world-rust/performance_optimization/examples/.gitkeep"
touch "$BASE_PATH/10-real-world-rust/deployment/examples/.gitkeep"
touch "$BASE_PATH/10-real-world-rust/concurrency_intro/examples/.gitkeep"
touch "$BASE_PATH/10-real-world-rust/error_handling_advanced/examples/.gitkeep"

# project_organization documentation
create_file "$BASE_PATH/10-real-world-rust/project_organization/README.md" "# Project Organization

## Overview
Best practices for structuring Rust projects, workspaces, and public APIs.

## Topics Covered
- Workspace setup and management
- Module hierarchy and visibility
- Public API design
- Feature flags and conditional compilation
- Versioning strategies (SemVer)
- Documentation structure

## Learning Objectives
- Create multi-crate workspaces
- Design clean module hierarchies
- Use visibility modifiers effectively
- Implement feature flags

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/10-real-world-rust/project_organization/key_takeaways.md" "# Key Takeaways: Project Organization

## Essential Concepts
1. **Workspaces** - Manage multiple related crates
2. **Module hierarchy** - Organize code logically
3. **Visibility** - Control what's public vs private
4. **Feature flags** - Optional functionality

## Common Patterns
- Binary + library crate structure
- Re-exports for clean API
- Prelude module for common imports
- Conditional compilation with features

## Best Practices
- Keep public API minimal
- Use workspaces for multi-crate projects
- Document public items thoroughly
- Follow SemVer for versioning
"

# performance_optimization documentation
create_file "$BASE_PATH/10-real-world-rust/performance_optimization/README.md" "# Performance Optimization

## Overview
Techniques for making Rust code faster and more efficient.

## Topics Covered
- Profiling techniques
- Allocation reduction strategies
- String optimization
- Zero-cost abstractions
- Inlining and const functions
- SIMD basics
- Lazy evaluation patterns
- Benchmarking with criterion

## Learning Objectives
- Profile code to find bottlenecks
- Optimize memory allocations
- Use compiler optimizations effectively
- Benchmark performance improvements

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/10-real-world-rust/performance_optimization/key_takeaways.md" "# Key Takeaways: Performance Optimization

## Essential Concepts
1. **Measure first** - Profile before optimizing
2. **Avoid allocations** - Use &str, slices, iterators
3. **Zero-cost abstractions** - Generics, iterators are free
4. **Compiler optimization** - Release builds are much faster

## Common Patterns
- \`&str\` over \`String\` when possible
- Iterator chains over loops
- \`Vec::with_capacity()\` to preallocate
- \`#[inline]\` for small functions

## Best Practices
- Benchmark with criterion
- Profile with perf, flamegraph
- Optimize hot paths only
- Document performance assumptions
"

# deployment documentation
create_file "$BASE_PATH/10-real-world-rust/deployment/README.md" "# Deployment

## Overview
Building, packaging, and deploying Rust applications.

## Topics Covered
- Release builds and optimizations
- Cross-compilation
- Static vs dynamic linking
- Docker containerization
- CI/CD integration
- Distribution strategies

## Learning Objectives
- Create optimized release builds
- Cross-compile for different platforms
- Containerize Rust applications
- Set up CI/CD pipelines

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/10-real-world-rust/deployment/key_takeaways.md" "# Key Takeaways: Deployment

## Essential Concepts
1. **Release mode** - \`cargo build --release\` for production
2. **Static linking** - Single binary, no dependencies
3. **Cross-compilation** - Build for different platforms
4. **Containerization** - Docker for consistent deployment

## Common Patterns
- Multi-stage Docker builds
- Static linking with musl
- Cross for cross-compilation
- GitHub Actions for CI/CD

## Best Practices
- Always test release builds
- Minimize Docker image size
- Use semantic versioning
- Automate with CI/CD
"

# concurrency_intro documentation
create_file "$BASE_PATH/10-real-world-rust/concurrency_intro/README.md" "# Concurrency Introduction

## Overview
Thread-based concurrency in Rust, focusing on safety and performance.

## Topics Covered
- Thread creation and joining
- Message passing with channels
- Shared state with Arc and Mutex
- Thread safety guarantees
- Data race prevention
- Thread pools
- Parallel iterators with rayon

## Learning Objectives
- Create and manage threads
- Communicate between threads safely
- Share data between threads
- Use parallel processing effectively

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/10-real-world-rust/concurrency_intro/key_takeaways.md" "# Key Takeaways: Concurrency

## Essential Concepts
1. **Thread safety** - Compiler prevents data races
2. **Message passing** - Channels for communication
3. **Shared state** - Arc + Mutex for shared data
4. **Send/Sync traits** - Compiler-checked thread safety

## Common Patterns
- \`thread::spawn()\` for creating threads
- \`mpsc::channel()\` for message passing
- \`Arc<Mutex<T>>\` for shared mutable state
- Rayon for parallel iterators

## Best Practices
- Prefer message passing over shared state
- Use scoped threads when possible
- Minimize locked regions
- Avoid deadlocks with lock ordering
"

# error_handling_advanced documentation
create_file "$BASE_PATH/10-real-world-rust/error_handling_advanced/README.md" "# Advanced Error Handling

## Overview
Sophisticated error handling strategies for production Rust code.

## Topics Covered
- Custom error types
- Error context and chains
- Using thiserror crate
- Using anyhow crate
- Error propagation patterns
- Recovery strategies

## Learning Objectives
- Define custom error types
- Add context to errors
- Use error handling crates effectively
- Implement error recovery

## See Also
- Examples in \`examples/\` directory
- Exercises in \`../exercises/\`
"

create_file "$BASE_PATH/10-real-world-rust/error_handling_advanced/key_takeaways.md" "# Key Takeaways: Advanced Error Handling

## Essential Concepts
1. **Custom errors** - Define domain-specific error types
2. **Error context** - Add information to errors
3. **thiserror** - Derive macro for custom errors
4. **anyhow** - Convenient error handling for applications

## Common Patterns
- \`#[derive(Error)]\` with thiserror
- \`.context()\` to add error context
- \`Result<T, anyhow::Error>\` for applications
- Error conversion with \`From\` trait

## Best Practices
- Use thiserror for libraries
- Use anyhow for applications
- Add context at error boundaries
- Document error conditions
"

# Exercise directories
for i in {1..4}; do
    case $i in
        1) name="modular_restructure" ;;
        2) name="code_optimization" ;;
        3) name="build_deploy" ;;
        4) name="thread_safety" ;;
    esac

    create_dir "$BASE_PATH/10-real-world-rust/exercises/exercise_${i}_${name}/src"
    create_dir "$BASE_PATH/10-real-world-rust/exercises/exercise_${i}_${name}/tests"
    create_dir "$BASE_PATH/10-real-world-rust/exercises/exercise_${i}_${name}/solution"

    create_file "$BASE_PATH/10-real-world-rust/exercises/exercise_${i}_${name}/README.md" "# Exercise $i: ${name//_/ }

## Objective
[Detailed exercise description]

## Requirements
- [Requirement 1]
- [Requirement 2]
- [Requirement 3]

## Skills Practiced
- [Skill 1]
- [Skill 2]

## Bonus Challenges
- [Challenge 1]
- [Challenge 2]
"

    create_file "$BASE_PATH/10-real-world-rust/exercises/exercise_${i}_${name}/Cargo.toml" "[package]
name = \"exercise_${i}_${name}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
"
done

echo -e "${GREEN}✓ Module 10 structure complete${NC}"
echo ""

# ============================================================================
# MODULE 11: LANGUAGE-SPECIFIC TRACKS
# ============================================================================

echo -e "${BLUE}Module 11: Language-Specific Tracks${NC}"
echo "------------------------------------"

# Concept mapping directories for each track
for track in JAVA_TRACK PYTHON_TRACK GO_TRACK CPP_TRACK; do
    create_dir "$BASE_PATH/11-language-specific-tracks/$track/concept_mapping"

    case $track in
        JAVA_TRACK)
            create_file "$BASE_PATH/11-language-specific-tracks/$track/concept_mapping/java_to_rust_ownership.md" "# Java to Rust: Ownership

## Java Approach
- Garbage collection manages memory
- Objects are always references
- No explicit memory management

## Rust Approach
- Ownership system manages memory
- Values can be on stack or heap
- Explicit ownership transfer (move semantics)

## Key Differences
[Details here]

## Migration Tips
[Tips here]
"
            ;;
        PYTHON_TRACK)
            create_file "$BASE_PATH/11-language-specific-tracks/$track/concept_mapping/python_to_rust_ownership.md" "# Python to Rust: Ownership

## Python Approach
- Reference counting + garbage collection
- Everything is an object reference
- Automatic memory management

## Rust Approach
- Ownership system manages memory
- Compile-time memory safety
- No garbage collection

## Key Differences
[Details here]

## Migration Tips
[Tips here]
"
            ;;
        GO_TRACK)
            create_file "$BASE_PATH/11-language-specific-tracks/$track/concept_mapping/go_to_rust_ownership.md" "# Go to Rust: Ownership

## Go Approach
- Garbage collection
- Pointers with GC
- Automatic memory management

## Rust Approach
- Ownership system
- No garbage collection
- Compile-time memory safety

## Key Differences
[Details here]

## Migration Tips
[Tips here]
"
            ;;
        CPP_TRACK)
            create_file "$BASE_PATH/11-language-specific-tracks/$track/concept_mapping/cpp_to_rust_ownership.md" "# C++ to Rust: Ownership

## C++ Approach
- Manual memory management (new/delete)
- RAII with destructors
- Smart pointers (shared_ptr, unique_ptr)

## Rust Approach
- Ownership system (similar to unique_ptr)
- RAII with Drop trait
- Smart pointers (Box, Rc, Arc)

## Key Differences
[Details here]

## Migration Tips
[Tips here]
"
            ;;
    esac
done

echo -e "${GREEN}✓ Module 11 structure complete${NC}"
echo ""

# ============================================================================
# SUMMARY
# ============================================================================

echo ""
echo "=========================================="
echo "Phase 6 Setup Complete!"
echo "=========================================="
echo ""
echo "Created structure for:"
echo "  - Module 04: Simple Programs"
echo "  - Module 05: CLI and Console Games"
echo "  - Module 06: Intermediate Rust"
echo "  - Module 08: Design Patterns"
echo "  - Module 09: Mini-Projects"
echo "  - Module 10: Real-World Rust"
echo "  - Module 11: Language-Specific Tracks"
echo ""
echo "Next steps:"
echo "  1. Fill in code examples"
echo "  2. Complete exercise implementations"
echo "  3. Test all examples and exercises"
echo "  4. Review and enhance documentation"
echo ""
echo "Happy coding!"
