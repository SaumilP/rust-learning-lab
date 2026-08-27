# Simple Calculator

## Overview

A command-line calculator supporting expressions, variables, and functions. Demonstrates parsing and evaluation patterns.

## Concepts Learned

- **Parsing**: Tokenization and expression parsing
- **Abstract Syntax Trees (AST)**: Expression representation
- **Recursive Evaluation**: Evaluating nested expressions
- **Enums**: Operation types (Add, Sub, Mul, Div)
- **Pattern Matching**: AST evaluation
- **Error Handling**: Division by zero, parse errors
- **Trait Objects**: Expression trait implementation

## Features

1. **Basic Arithmetic**: Addition, subtraction, multiplication, division
2. **Operator Precedence**: Correct evaluation order
3. **Parentheses**: Override precedence
4. **Variables**: Store and reuse values
5. **Functions**: Define and call basic functions
6. **Error Recovery**: Continue after errors

## Data Structures

```rust
pub enum Expr {
    Number(f64),
    Variable(String),
    BinOp {
        left: Box<Expr>,
        op: Operation,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
}

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
```

## Commands

```bash
# Simple arithmetic
cargo run
> 2 + 3 * 4
14

# Parentheses
> (2 + 3) * 4
20

# Variables
> x = 5
> x * 2 + 3
13

# Function definition
> def square(n) = n * n
> square(5)
25
```

## Implementation Details

### Parsing Pipeline

```
Input String
    ↓
Tokenizer → [Token]
    ↓
Parser → Expr (AST)
    ↓
Evaluator → Result<f64>
```

### Key Functions

```rust
pub fn tokenize(input: &str) -> Result<Vec<Token>, Error>
pub fn parse(tokens: &[Token]) -> Result<Expr, Error>
pub fn evaluate(expr: &Expr, ctx: &Context) -> Result<f64, Error>
```

### Error Handling

- Invalid token sequence
- Unclosed parentheses
- Division by zero
- Undefined variables
- Type mismatches

## Testing

```bash
# Run unit tests
cargo test

# Test specific expression
cargo test -- --nocapture

# Performance: large expressions
time cargo run < large_input.txt
```

## Extension Possibilities

1. **Scientific Functions**: sin, cos, sqrt, log
2. **Complex Numbers**: i notation support
3. **Matrix Operations**: [[1,2],[3,4]]
4. **Symbolic Computation**: Simplification
5. **History**: Command history and recall
6. **Visualization**: Plot functions

## Learning Outcomes

After completing this project:
- ✓ Lexical analysis and tokenization
- ✓ Parsing and AST construction
- ✓ Recursive evaluation
- ✓ Operator precedence handling
- ✓ Expression simplification

## Common Patterns Used

### Recursive Descent Parsing
```rust
fn parse_expression(tokens: &[Token]) -> Result<Expr, Error> {
    let mut left = parse_primary(tokens)?;
    while matches!(current_token, Token::Plus | Token::Minus) {
        let op = parse_op()?;
        let right = parse_primary(tokens)?;
        left = Expr::BinOp { left: Box::new(left), op, right: Box::new(right) };
    }
    Ok(left)
}
```

### Pattern Matching Evaluation
```rust
match expr {
    Expr::Number(n) => Ok(*n),
    Expr::BinOp { left, op, right } => {
        let l = evaluate(left)?;
        let r = evaluate(right)?;
        match op {
            Operation::Add => Ok(l + r),
            Operation::Divide if r == 0.0 => Err("Division by zero"),
            _ => Ok(perform_op(l, r, op))
        }
    }
}
```

## Estimated Development Time

- Beginner: 6-8 hours
- Intermediate: 3-4 hours
- Experienced Rust: 2-3 hours

## Related Modules

- Module 04: Recursion and algorithms
- Module 06: Trait implementations
- Module 08: Design patterns (Factory for operations)

