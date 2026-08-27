# Mini-Projects (Practical Application)

## Overview

Module 09 brings together concepts from previous modules into complete, runnable projects. Each project reinforces specific learning outcomes and demonstrates real-world scenarios.

## Projects

### 1. Todo CLI Application
**File**: `todo_cli/`

**Concepts Applied**:
- File I/O and persistence
- Enum-based state management
- Collections (Vec<T>, HashMap)
- Error handling with Result
- Command-line argument parsing
- Trait implementations

**Features**:
- Add, list, complete, delete todos
- Save/load from file
- Filter by status
- Priority levels
- Due dates

**Learning Outcomes**:
- ✓ Practical file operations
- ✓ Struct and enum design
- ✓ Error handling patterns
- ✓ Command processing

---

### 2. Simple Calculator
**File**: `calculator/`

**Concepts Applied**:
- Parsing and arithmetic operations
- Enum for operations (Add, Sub, Mul, Div)
- Pattern matching
- Error handling for division by zero
- Trait objects for expression evaluation
- Generic computation

**Features**:
- Basic arithmetic (+ - * /)
- Parentheses support
- Variables
- Function definitions
- Error recovery

**Learning Outcomes**:
- ✓ Expression parsing
- ✓ Recursive evaluation
- ✓ Type-safe operations
- ✓ Error handling at boundaries

---

### 3. Note-Taking Application
**File**: `note_taker/`

**Concepts Applied**:
- Struct composition
- Trait objects for different note types
- Ownership and borrowing
- Collections management
- Serialization/Deserialization (serde)
- Lifetimes in complex structures

**Features**:
- Create, read, update, delete notes
- Multiple note types (text, list, code)
- Tagging and categorization
- Search functionality
- Export to different formats
- Rich text support

**Learning Outcomes**:
- ✓ Complex data structures
- ✓ Trait-based polymorphism
- ✓ Serialization patterns
- ✓ Search algorithms

---

### 4. Weather CLI
**File**: `weather_cli/`

**Concepts Applied**:
- HTTP requests with reqwest
- JSON parsing with serde_json
- Error handling for network operations
- Async/await basics
- Command-line interfaces
- Configuration management
- Caching strategies

**Features**:
- Fetch current weather
- Multi-city support
- Temperature unit conversion
- Forecast viewing
- Weather alerts
- Local caching
- Configuration files

**Learning Outcomes**:
- ✓ Network programming
- ✓ Async Rust fundamentals
- ✓ External API integration
- ✓ Error handling for I/O

---

## Project Structure

Each project follows this structure:

```
project_name/
├── Cargo.toml              # Project dependencies
├── README.md               # Project-specific documentation
├── src/
│   ├── main.rs            # Entry point
│   ├── lib.rs             # Library code
│   ├── models.rs          # Data structures
│   ├── operations.rs      # Core logic
│   └── cli.rs             # Command-line interface
├── tests/                 # Integration tests
├── examples/              # Usage examples
└── data/                  # Sample data files
```

## Progressive Complexity

### Project 1: Todo CLI (Beginner)
- Single file I/O
- Simple data structures
- Basic trait implementation
- Linear program flow

### Project 2: Calculator (Beginner-Intermediate)
- Expression parsing
- Recursive structures
- Pattern matching depth
- Error handling complexity

### Project 3: Note Taker (Intermediate)
- Complex data composition
- Multiple trait implementations
- Serialization/deserialization
- Search algorithms

### Project 4: Weather CLI (Intermediate-Advanced)
- Network I/O
- Async/await fundamentals
- External dependency integration
- Configuration management

## Skills Developed

### Across All Projects
- ✓ Project organization
- ✓ Module structure
- ✓ Error handling
- ✓ Testing strategies
- ✓ Documentation
- ✓ CLI design patterns

### Domain-Specific Skills

**Todo CLI**:
- Persistent data storage
- State management
- File format design

**Calculator**:
- Parser implementation
- AST construction
- Expression evaluation

**Note Taker**:
- Type polymorphism
- Complex ownership patterns
- Serialization

**Weather CLI**:
- Network requests
- Async programming
- External API integration

## Development Approach

For each project:

1. **Plan** - Understand requirements and data structures
2. **Design** - Create module and trait architecture
3. **Implement** - Build core functionality
4. **Test** - Add unit and integration tests
5. **Polish** - Error messages, help text, configuration
6. **Document** - README, examples, comments

## Integration with Previous Modules

- **Modules 01-04**: Game logic, random generation, I/O
- **Module 05**: Game loop patterns applied to event handling
- **Module 06**: Traits, ownership, generics in practice
- **Module 07**: Design patterns (Factory for note types, Strategy for operations)
- **Module 08**: Design patterns (Facade for API, Builder for config)

## Extension Ideas

### Todo CLI Extensions
- SQLite backend instead of JSON
- Web API for sync
- Due date reminders
- Recurring todos
- Priority-based sorting
- Time tracking

### Calculator Extensions
- Scientific functions (sin, cos, sqrt)
- Variable assignment
- Function definitions
- Number bases (hex, binary)
- Matrix operations
- Symbolic computation

### Note Taker Extensions
- Full-text search
- Database persistence
- Web interface
- Mobile sync
- Encryption
- Collaboration

### Weather CLI Extensions
- Weather maps/visualization
- Severe weather alerts
- Historical weather data
- Environmental metrics
- Integration with smart home
- Custom notifications

## Best Practices Demonstrated

- Separation of concerns
- Error propagation patterns
- Configuration management
- Testing strategies
- Documentation standards
- User experience design
- Performance optimization

## Running the Projects

Each project can be run with:

```bash
cd 09-mini-projects/project_name
cargo run -- [arguments]
```

See individual project README for specific usage.

## Next Steps

After completing mini-projects:
1. Combine concepts from multiple projects
2. Add persistence (database, file storage)
3. Create web interfaces
4. Implement peer-to-peer features
5. Optimize for performance

