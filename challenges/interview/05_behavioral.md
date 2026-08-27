# Interview Questions: Behavioral & Soft Skills

## Question 1: Approach to Learning a New Crate

**Difficulty**: Easy-Medium
**Concepts**: Learning strategy, documentation, problem-solving
**Time**: 15-20 minutes (discussion-focused)

### Problem Statement

You need to use a new Rust crate for your project (e.g., `serde`, `tokio`, `axum`) but have no prior experience. Walk through your approach.

### Sample Crate: Learning `tokio` for Async Runtime

### Your Approach Should Cover

1. **Initial Research**
   - Read crate documentation (README, lib.rs docs)
   - Check GitHub repository
   - Look at examples and tutorials
   - Understand primary use cases

2. **Conceptual Understanding**
   - What problem does it solve?
   - How does it fit your needs?
   - What are its limitations?
   - What are the alternatives?

3. **Hands-On Learning**
   - Write simple example program
   - Follow official tutorial
   - Modify examples to understand behavior
   - Experiment with different features

4. **Integration**
   - Small proof-of-concept
   - Understand error handling
   - Learn key APIs
   - Test with your code

5. **Going Deeper**
   - Read blog posts/articles
   - Study relevant examples
   - Understand performance implications
   - Learn best practices

### Evaluation Points

**Good Approach Shows**:
- Structured learning methodology
- Combination of reading and doing
- Community resource usage
- Practical experimentation
- Documentation reading skills

**Weak Approach**:
- Jumping straight to implementation
- Only reading others' code
- Not validating understanding
- Ignoring documentation

### Common Crates to Study

1. **`serde`** - Serialization/deserialization
2. **`tokio`** - Async runtime
3. **`axum`** - Web framework
4. **`sqlx`** - SQL toolkit
5. **`regex`** - Pattern matching
6. **`clap`** - CLI argument parsing

### Example Response Framework

"When I need to learn a new crate, I follow these steps:

1. **Understand the Purpose** - I read the crate documentation to understand what problem it solves and if it's the right tool.

2. **Assess Scope** - I check the main types, traits, and functions to understand the API surface.

3. **Study Examples** - I look at the examples in the repository and work through them to see how the crate is used.

4. **Experiment** - I create a simple project and try using the crate to solve a small problem. This helps me understand error messages and real usage.

5. **Integrate Gradually** - Rather than rewriting everything at once, I start with a small part of my project to validate my understanding.

6. **Deep Dive** - Once basic understanding is solid, I explore advanced features, performance considerations, and best practices through blog posts and community discussions."

### Discussion Questions

- How do you decide between similar crates?
- What makes good documentation?
- When to ask for help vs figure out yourself?
- How do you verify your understanding?

---

## Question 2: Handle Ambiguous Requirements

**Difficulty**: Medium
**Concepts**: Communication, clarification, scope management
**Time**: 15-20 minutes (discussion-focused)

### Problem Statement

You receive a vague requirement: "Make the system faster." Walk through how you handle this.

### Ambiguities to Clarify

1. **Current State**
   - What is "fast" for this system?
   - What is the baseline? (measurements?)
   - Are there existing performance issues?
   - What's the user-visible impact?

2. **Target**
   - What's the goal? (2x faster? 10x?)
   - Which operations matter most?
   - Are there specific metrics?
   - Deadline/priority?

3. **Constraints**
   - Budget for optimization?
   - Resource allocation?
   - Risk tolerance?
   - Backward compatibility needed?

### Your Approach Should Include

1. **Ask Clarifying Questions**
   - "What specific operations are slow?"
   - "What's the target performance?"
   - "Are there user complaints?"
   - "What's your timeline?"

2. **Gather Data**
   - Profile the current system
   - Identify bottlenecks
   - Measure baseline metrics
   - Understand usage patterns

3. **Propose Solution**
   - Identify top opportunities
   - Estimate improvement potential
   - Discuss trade-offs
   - Get buy-in on approach

4. **Implement Incrementally**
   - Tackle highest impact items first
   - Measure before and after
   - Get feedback
   - Plan next phase

5. **Document**
   - Record baseline metrics
   - Document changes made
   - Explain trade-offs
   - Guide future work

### Example Scenario

**Vague Requirement**: "Our API is too slow. Fix it."

**Good Response**:
"I'd like to understand better what we're optimizing for:
1. Are there specific endpoints that are slow, or is it system-wide?
2. What response times are we targeting? (e.g., < 100ms p95)
3. How many users/requests are we handling?
4. Are there user complaints or business metrics showing impact?
5. What's the timeline for this work?

Once I understand the constraints, I'll:
- Profile to identify bottlenecks
- Propose solutions with estimated improvements
- Implement and measure impact
- Focus on highest ROI changes first"

### Evaluation Points

**Strong Approach Shows**:
- Asks clarifying questions
- Gathers data before deciding
- Proposes multiple options
- Communicates trade-offs
- Has measurement plan

**Weak Approach**:
- Assumes what's needed
- Implements without data
- Optimizes wrong things
- Doesn't measure impact
- Poor communication

### Follow-Up Discussions

- How to handle pushback on needing more information?
- When to make reasonable assumptions?
- Escalation paths if requirements stay unclear?
- Documentation of decisions?

---

## Question 3: Make Architecture Trade-Offs

**Difficulty**: Medium
**Concepts**: Decision-making, trade-off analysis, communication
**Time**: 20-30 minutes

### Problem Statement

You need to choose between architectural options with different trade-offs. Walk through your decision process.

### Sample Scenario

**Choice**: Monolith vs Microservices

**Monolith Pros**:
- Simpler to deploy
- Easier to test
- Lower latency for inter-service calls
- Simpler transaction semantics
- Easier debugging

**Monolith Cons**:
- Harder to scale independent services
- Single point of failure
- Language/tech lock-in
- Larger blast radius for bugs
- Harder to parallelize teams

**Microservices Pros**:
- Independent scaling
- Team independence
- Technology flexibility
- Fault isolation
- Easier to replace components

**Microservices Cons**:
- Distributed system complexity
- Operational overhead
- Network latency
- Data consistency challenges
- Debugging difficulty

### Your Approach Should Address

1. **Define Success Criteria**
   - What matters for this project?
   - What are our constraints?
   - Timeline and resources?
   - Current team capability?

2. **Gather Data**
   - Current pain points?
   - Expected growth trajectory?
   - Team size and expertise?
   - Operational capability?

3. **Evaluate Options**
   - How does each align with criteria?
   - What are hidden costs?
   - Learning curves needed?
   - Migration path later?

4. **Recommend with Context**
   - Explain chosen option
   - Acknowledge trade-offs
   - Outline implementation plan
   - Plan future evolution

5. **Prepare for Change**
   - How easily can we pivot?
   - What signals should trigger re-evaluation?
   - When should we refactor?

### Example Decision Framework

**For Current Stage**:
"For a team of 5 with 10K daily users, I'd recommend starting with a monolith because:
- Simpler initial development and deployment
- Team can move quickly
- Operational overhead is manageable at this scale
- Clear migration path to microservices later if needed

**Milestones to Reconsider**:
- If we hit 1M users and scaling becomes bottleneck
- If team grows to 20+ engineers needing independent workflows
- If specific services need different technology stacks

**How to Prepare**:
- Keep services loosely coupled using interfaces
- Make data boundaries clear
- Use internal APIs between services
- This makes future extraction to microservices easier"

### Evaluation Points

**Strong Approach Shows**:
- Understands trade-offs deeply
- Considers current vs future state
- Proposes based on context
- Acknowledges assumptions
- Plans for evolution

**Weak Approach**:
- Favors one option always
- Ignores constraints
- Makes decision without context
- No contingency plan
- Overengineers

### Discussion Topics

- When to optimize for current need vs future flexibility?
- Recognizing when to re-evaluate decisions?
- Communicating technical trade-offs to non-technical stakeholders?
- Learning from past decisions?

---

## Question 4: Team Code Review

**Difficulty**: Medium
**Concepts**: Communication, feedback, collaboration
**Time**: 15-25 minutes

### Problem Statement

You're reviewing a colleague's code. How do you provide constructive feedback?

### Scenarios

**Scenario 1**: Colleague uses inefficient algorithm
**Scenario 2**: Code lacks error handling
**Scenario 3**: Style/convention violation
**Scenario 4**: Good solution, but you see alternative

### Your Approach Should Show

1. **Understanding**
   - Read code thoroughly
   - Understand the context
   - Recognize the constraints
   - Acknowledge what works well

2. **Positive First**
   - Compliment good parts
   - Recognize effort
   - Appreciate different approaches
   - Show respect

3. **Constructive Feedback**
   - Explain the issue (not just criticism)
   - Provide context/why it matters
   - Suggest solution (not just problems)
   - Ask questions rather than demand

4. **Collaborative Tone**
   - "What about..." instead of "You should..."
   - "We could consider..." vs "That's wrong"
   - Questions: "Did you consider...?"
   - Partner in solving problem

5. **Know When to Let It Go**
   - Different isn't wrong
   - Learning opportunity
   - Non-blocking vs blocking
   - Pick important battles

### Example Feedback Framework

**Situation**: Colleague wrote nested loops instead of using HashMap

**Good Feedback**:
"I noticed we're using nested loops here to find matching elements. This is O(n²) which could become a bottleneck if the dataset grows. Have you considered using a HashMap for O(n) lookup? Something like:
```rust
let map: HashMap<_, _> = first.iter().map(|x| (x.id, x)).collect();
for item in &second {
    if let Some(match) = map.get(&item.id) {
        // process match
    }
}
```
Would that approach work for our use case?"

**Why It Works**:
- Explains the concern (time complexity)
- Provides context (why it matters)
- Suggests solution with code
- Frames as question (collaborative)
- Respects their autonomy

### Evaluation Points

**Strong Review Shows**:
- Technical depth
- Constructive tone
- Respect for colleague
- Clear communication
- Collaborative spirit

**Weak Review**:
- Dismissive tone
- "Just rewrite this"
- No explanation
- Personal criticism
- Uncompromising stance

### Scenario Responses

**When Your Way Is Better**:
"I appreciate you thinking about this differently. We might also consider [approach] because [reason]. What do you think? Let's pair on this if you're open to it."

**When It's Personal Style**:
"This is a style choice and works fine. We tend to use [convention] for consistency. No need to change this time, but something to keep in mind."

**When It's A Blocker**:
"I found a potential issue: [specific concern]. This could cause [problem] in production. Let's discuss how to address this."

---

## Question 5: Document Complex Solution

**Difficulty**: Medium
**Concepts**: Communication, clarity, documentation
**Time**: 20-30 minutes

### Problem Statement

You've implemented a complex system component. How do you document it so others can understand and maintain it?

### Sample Scenario: LRU Cache Implementation

**Complexity Factors**:
- Combines HashMap with LinkedList
- Uses RefCell for interior mutability
- Tricky eviction logic
- Performance-critical

### Documentation Should Include

1. **High-Level Overview**
   - What does this component do?
   - Why is it needed?
   - What problem does it solve?

2. **Architecture**
   - How is it structured?
   - Key components and their roles
   - Design decisions and trade-offs
   - Why this approach over alternatives

3. **Algorithm Explanation**
   - How does it work?
   - Key operations (get, put, evict)
   - Time/space complexity
   - Why is it efficient?

4. **Data Structure Diagram**
   - Visual representation
   - Connections between parts
   - State transitions if applicable

5. **Usage Examples**
   - Simple cases first
   - Common patterns
   - Edge cases
   - Error handling

6. **Implementation Notes**
   - Tricky parts explained
   - Why certain choices were made
   - Potential pitfalls
   - Future improvements

### Example Documentation

**For LRU Cache**:

```
# LRU Cache

## Overview
An LRU (Least Recently Used) cache stores frequently accessed data
while maintaining a fixed memory footprint. When the cache is full,
the least recently used item is evicted.

## Architecture
The cache uses two key structures:
- HashMap: Maps keys to cache entries (O(1) lookup)
- DoublyLinkedList: Tracks access order (newest at front, oldest at back)

When an item is accessed, it's moved to the front. When eviction
is needed, the back item is removed.

## Operations
- get(key): O(1) - Lookup in HashMap, move to front in list
- put(key, value): O(1) - Insert in HashMap, add to front of list
- Eviction: O(1) - Remove item from back of list

## Why This Design?
We need O(1) access AND O(1) eviction. HashMap gives us fast access,
but doesn't track order. LinkedList tracks order but has O(n) lookup.
Combining them gives O(1) for both operations.

## Tricky Parts
The interior mutability (RefCell) is needed because we mutate
the list while only having references through the HashMap.

## Example
```rust
let mut cache = LruCache::new(2);
cache.put(1, "a");
cache.put(2, "b");
assert_eq!(cache.get(1), Some(&"a"));  // 1 becomes most recent
cache.put(3, "c");  // Evicts key 2
```
```

### Evaluation Points

**Strong Documentation Shows**:
- Clear explanation at multiple levels
- Diagrams for complex concepts
- Example usage
- Trade-off explanations
- Edge cases mentioned

**Weak Documentation**:
- Just code comments
- No high-level explanation
- Assumes reader knowledge
- No examples
- No context for decisions

### Types of Documentation

1. **README/Module docs** - High level overview
2. **Function docs** - What, why, examples
3. **Inline comments** - Non-obvious code
4. **Design document** - Architecture and rationale
5. **Examples** - Working code samples

---

## Summary of Behavioral Questions

These questions assess:
- Communication skills
- Problem-solving approach
- Team collaboration
- Decision-making under uncertainty
- Teaching and documentation abilities

**How to Prepare**:
- Reflect on real experiences
- Use STAR method (Situation, Task, Action, Result)
- Be honest about mistakes and learning
- Show growth mindset
- Demonstrate empathy and collaboration
