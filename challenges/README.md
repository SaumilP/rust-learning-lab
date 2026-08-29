# Rust Learning Lab - Challenge Problems

Welcome to the Rust Challenge Problems repository! This collection contains problems of varying difficulty designed to help you practice and master Rust programming at different levels.

## Structure

The challenges are organized by difficulty level, mirroring coding interview and competition sites like LeetCode, HackerRank, and Codewars.

```
challenges/
|-- kata/                    # Beginner problems (single concept)
|-- beginner/               # Beginner-Intermediate (2-3 concepts)
|-- interview/              # Technical interview questions
|   |-- 01_system_design.md
|   |-- 02_data_structures.md
|   |-- 03_algorithms.md
|   |-- 04_system_problems.md
|   `-- 05_behavioral.md
`-- README.md              # This file
```

## Difficulty Levels

### Kata (Beginner)
- **Scope**: Single function, ~5-15 lines
- **Concepts**: One core concept
- **Time**: 15-30 minutes
- **Categories**: 20 problems across 4 categories

**Categories**:
1. **String Manipulation** (5 problems)
   - Reverse String
   - Count Vowels
   - Capitalize Words
   - Remove Whitespace
   - Snake Case Conversion

2. **Number Operations** (5 problems)
   - Sum Array
   - Calculate Average
   - Check Prime
   - Calculate Factorial
   - Find Maximum

3. **Collections** (5 problems)
   - Find Duplicate
   - Remove Duplicates
   - Merge Sorted Arrays
   - Filter Type
   - Flatten One Level

4. **Logic Puzzles** (5 problems)
   - Pattern Generation
   - FizzBuzz
   - Number Sequence
   - Boolean Logic
   - Simple Sort

### Beginner Level (4-5 kyu equivalent)
- **Scope**: Function + helper, ~20-40 lines
- **Concepts**: 2-3 related concepts
- **Time**: 30-60 minutes
- **Count**: 12 problems across 3 categories

**Categories**:
1. **Intermediate Strings** (3 problems)
   - Anagram Checker
   - Palindrome Validator
   - Word Frequency Counter

2. **Array Algorithms** (4 problems)
   - Two Sum
   - Rotate Array
   - Group Elements
   - Find Pairs with Sum

3. **Number Theory** (3 problems)
   - GCD and LCM
   - Fibonacci Sequence
   - Roman Numeral Conversion

4. **Data Structures** (3 problems)
   - Implement Stack
   - Implement Queue
   - HashMap Usage

### Interview Questions
Comprehensive interview scenarios split into 5 categories:

1. **System Design** (5 questions)
   - URL Shortener
   - Rate Limiter
   - Cache Layer
   - Load Balancer
   - Logging System

2. **Data Structures** (5 questions)
   - Implement HashMap
   - Implement Vec
   - Implement LRU Cache
   - Implement Binary Search Tree
   - Implement Trie

3. **Algorithms & Optimization** (5 questions)
   - Optimize Slow Query
   - Fix Memory Issues
   - Handle Concurrency
   - Design Error Handling
   - Implement Testing

4. **System-Level Problems** (5 questions)
   - Debug Race Conditions
   - Optimize Performance
   - Handle Edge Cases
   - Design Scalability
   - Plan Infrastructure Migration

5. **Behavioral** (5 questions)
   - Learn New Crate
   - Handle Ambiguous Requirements
   - Make Architecture Trade-Offs
   - Conduct Code Review
   - Document Complex Solution

## Quiz items

The foundation quiz set lives in [quiz-items/](./quiz-items/) and uses structured, validated JSON rather than a site-specific format. It covers reasoning about variables, ownership, borrowing, `Option`, and `Result`; see [the quiz authoring contract](../docs/quiz-content.md) for the format and validation command.

The [Will It Compile?](./will-it-compile/) format is a focused subset of those items. It asks for a prediction about compiler behaviour before showing the answer and explanation.

## Getting Started

### Prerequisites

Before attempting problems, ensure you have:
- Rust and Cargo installed
- Basic understanding of the prerequisite modules
- Text editor or IDE with Rust support

### How to Use These Problems

1. **Read the Problem Statement**
   - Understand the requirements
   - Note any constraints
   - Review the function signature

2. **Study Examples**
   - Work through test cases
   - Understand input/output format
   - Identify edge cases

3. **Implement Solution**
   - Write your code
   - Test with provided examples
   - Handle edge cases

4. **Compare Approaches**
   - Think about time/space complexity
   - Review hints for alternative approaches
   - Optimize if needed

5. **Reflect and Learn**
   - What did you learn?
   - What was difficult?
   - Could you improve the solution?

## Problem-Solving Tips

### Kata Level Tips
- Start with the simplest test case
- Use standard library methods where appropriate
- Don't overthink - find straightforward solution
- After working example: think about edge cases

### Beginner Level Tips
- Break problem into smaller parts
- Use appropriate data structures
- Consider time/space trade-offs
- Test with edge cases systematically

### Interview Question Tips
- Ask clarifying questions
- Communicate your thinking
- Discuss trade-offs
- Consider scalability
- Be honest about what you don't know
- Show growth mindset

## Progression Path

**Recommended Order**:
1. Start with Kata level (20 problems)
2. Move to Beginner level (12 problems)
3. Practice Interview questions by category
4. Revisit difficult problems monthly

**Time Estimate**:
- Kata: ~1 hour per 5 problems
- Beginner: ~1 hour per 3 problems
- Interview: ~2-3 hours per question
- **Total**: ~30-40 hours for all categories

## Making the Most of These Problems

### Learning Goals
- Master core Rust concepts
- Build problem-solving skills
- Practice communication
- Prepare for interviews
- Gain confidence

### Best Practices
- Don't look at solutions immediately
- Try multiple approaches
- Write clean, readable code
- Add comments explaining logic
- Test thoroughly

### Tracking Progress
- Create a checklist of completed problems
- Note problem-solving strategies
- Track time spent on each
- Identify weak areas
- Revisit failed problems

## Related Resources

### Modules This Complements
- `01-core-fundamentals/` - Variables, data types, functions
- `02-standard-library/` - Collections, strings, iterators
- `03-tooling-and-quality/` - Testing and debugging
- `04-simple-programs/` - File I/O, CLI arguments
- `05-cli-and-console-games/` - Game loops, state

### External Resources
- [Rust Book](https://doc.rust-lang.org/book/) - Official guide
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
- [Exercism.org](https://exercism.org/tracks/rust) - Mentored exercises
- [LeetCode](https://leetcode.com/) - Algorithm practice
- [Interview.dev](https://interview.dev) - System design

## FAQ

### Q: How do I know which difficulty level to start with?
**A**: Start with Kata if you're new to Rust or problem-solving. If you're already comfortable with the language, try Beginner level. Interview questions are for comprehensive preparation.

### Q: Can I use external crates?
**A**: For Kata and Beginner problems, stick to standard library. For interview questions, discuss crate usage with your interviewer (if applicable).

### Q: How long should I spend on a problem before looking at hints?
**A**: Try for at least 15-20 minutes first. If truly stuck, look at the hint section. Avoid full solutions until you've attempted thoroughly.

### Q: Should I solve in a specific order?
**A**: The suggested order works well, but feel free to skip around. If a problem seems too hard, try others first and return later.

### Q: How do I improve from mistakes?
**A**: After solving or getting stuck:
1. Understand the correct approach
2. Reimplement from scratch
3. Note the key insight you missed
4. Practice similar problems

### Q: Can I suggest problems or improvements?
**A**: Yes! This is a living resource. Contributions are welcome.

## Contributing

Found an error? Have a better solution? Want to add more problems?

Contributing guidelines:
- Ensure problems have clear statements
- Test all code examples
- Include multiple test cases
- Add helpful hints (without spoilers)
- Document time complexity
- Be respectful and constructive

## License

These problems are part of the Rust Learning Lab project and are available for educational use.

## Acknowledgments

This problem set is inspired by:
- LeetCode and HackerRank problem formats
- Exercism's Rust track
- Coding interview best practices
- Community feedback and contributions

---

**Happy Coding!**

Start with [kata/](./kata/) for beginner problems and work your way up to [interview/](./interview/) for comprehensive interview preparation.

For questions or issues, refer to the main project README.
