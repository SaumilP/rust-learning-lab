# Interview Questions: System-Level Problems

## Question 1: Debug Race Condition

**Difficulty**: Hard
**Concepts**: Concurrency, timing, debugging
**Time**: 30-40 minutes

### Problem Statement

Identify and fix a race condition in concurrent code that only manifests intermittently.

### Scenario: Counter Increment Bug

**Buggy Code**:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn count_with_bug() -> usize {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // BUG: What if lock is released between read and write?
            // Or if thread scheduling causes issues?
            for _ in 0..100 {
                let mut num = c.lock().unwrap();
                *num += 1;
                // Lock released here
                // Another thread might increment between our increments
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *counter.lock().unwrap()
}
```

### Analysis Questions

1. **Understanding the Bug**
   - Is there actually a race condition here?
   - Rust's type system handles this correctly
   - In other languages, this would be a problem

2. **Rust's Solutions**
   - Mutex ensures atomicity
   - Type system prevents unsynchronized access
   - RAII ensures lock release

3. **Variations of Real Bugs**
   - Lock ordering issues (deadlocks)
   - Holding lock too long
   - Using wrong synchronization primitive

### Real Bug Scenario: Deadlock

**Problematic Pattern**:
```rust
let lock1 = Arc::new(Mutex::new(0));
let lock2 = Arc::new(Mutex::new(0));

// Thread A acquires lock1 first
// Thread B acquires lock2 first
// Then A tries lock2, B tries lock1 -> DEADLOCK
```

### Detection Strategies

1. **Stress Testing**
   - Run many iterations
   - Multiple threads
   - Vary thread count

2. **Thread Sanitizer**
   - TSAN detects certain race conditions
   - Requires non-optimized builds
   - May have false positives

3. **Logic Analysis**
   - Lock ordering verification
   - Deadlock detection by inspection
   - Code review

4. **Testing**
   - Unit tests with known bad inputs
   - Parameterized tests with different thread counts
   - Fuzzing with random thread schedules

### Debugging Tools

**Using Mutex Guards Correctly**:
```rust
{
    let mut num = counter.lock().unwrap();
    *num += 1;
    // Lock automatically released at end of scope
}
```

**Preventing Deadlocks**:
```rust
// Always acquire locks in same order
let g1 = lock1.lock().unwrap();
let g2 = lock2.lock().unwrap();
// Use, then drop (scoped)
```

### Discussion Points

- Rust's compile-time guarantees
- When you still need careful design
- Testing concurrent code
- Performance vs safety trade-offs

---

## Question 2: Optimize Performance Bottleneck

**Difficulty**: Medium-Hard
**Concepts**: Profiling, performance analysis, optimization
**Time**: 30-40 minutes

### Problem Statement

Identify performance bottleneck in real code and optimize it.

### Scenario: Slow Data Processing

**Initial Implementation**:
```rust
pub fn process_data(data: &[u32]) -> Vec<u32> {
    let mut result = Vec::new();
    for &item in data {
        let processed = expensive_calculation(item);
        // String allocation inside loop
        let s = format!("Item: {}", processed);
        if is_valid(&s) {
            result.push(processed);
        }
    }
    result
}

fn expensive_calculation(n: u32) -> u32 {
    // Simulate expensive operation
    std::thread::sleep(std::time::Duration::from_millis(1));
    n * 2
}

fn is_valid(s: &str) -> bool {
    s.len() > 5
}
```

### Performance Analysis

1. **Identify Bottlenecks**
   - Allocations in tight loop?
   - Unnecessary operations?
   - Algorithm complexity?
   - I/O operations?

2. **Profiling Tools**
   - `perf` (Linux)
   - `cargo flamegraph`
   - `criterion` for benchmarks
   - Instrumentation

3. **Measurements**
   - Before optimization baseline
   - After optimization comparison
   - Statistical significance

### Optimization Strategies

**Strategy 1: Reduce Allocations**
```rust
pub fn process_data_optimized(data: &[u32]) -> Vec<u32> {
    data.iter()
        .map(|&item| expensive_calculation(item))
        .filter(|processed| {
            // Avoid string allocation
            processed.to_string().len() > 5
        })
        .collect()
}
```

**Strategy 2: Parallel Processing**
```rust
use rayon::prelude::*;

pub fn process_data_parallel(data: &[u32]) -> Vec<u32> {
    data.par_iter()
        .map(|&item| expensive_calculation(item))
        .filter(|processed| processed > &5)
        .collect()
}
```

**Strategy 3: Caching Results**
```rust
use std::collections::HashMap;

pub fn process_data_cached(data: &[u32]) -> Vec<u32> {
    let mut cache = HashMap::new();
    data.iter()
        .map(|&item| {
            *cache.entry(item)
                .or_insert_with(|| expensive_calculation(item))
        })
        .filter(|processed| processed > &5)
        .collect()
}
```

### Questions to Address

1. **Root Cause**
   - What's actually slow?
   - Is it CPU or I/O?
   - Algorithm or implementation?

2. **Trade-offs**
   - Memory vs speed?
   - Complexity vs performance?
   - Development time vs optimization?

3. **Measurement**
   - Benchmarking strategy?
   - Statistical confidence?
   - Real-world vs synthetic?

### Discussion Points

- Premature optimization dangers
- When to optimize
- Tools and techniques
- Parallelization overhead

---

## Question 3: Handle Edge Cases

**Difficulty**: Medium
**Concepts**: Robustness, error handling, testing
**Time**: 25-35 minutes

### Problem Statement

Consider edge cases in production code and handle them correctly.

### Scenario: Data Validation Function

**Function Purpose**: Validate user input for account creation

**Input Validation Needed**:
- Email format
- Password strength
- Name (empty, special characters)
- Age (valid range, type)
- Phone (format, length)

### Edge Cases to Consider

1. **Boundary Conditions**
   - Empty strings
   - Maximum length strings
   - Single character strings
   - Null/None values

2. **Special Characters**
   - Unicode characters
   - Control characters
   - Escaped characters
   - Combinations

3. **Type Conversions**
   - String to number edge cases
   - Overflow/underflow
   - Precision loss (float)

4. **Resource Limits**
   - Very large inputs
   - Performance timeouts
   - Memory constraints

5. **Encoding Issues**
   - UTF-8 vs other encodings
   - Byte vs character length
   - Combining characters

### Implementation Example

**Naive Approach**:
```rust
pub fn validate_email(email: &str) -> bool {
    email.contains("@")  // INCOMPLETE
}
```

**Robust Approach**:
```rust
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    // Check not empty
    if email.is_empty() {
        return Err(ValidationError::EmptyEmail);
    }

    // Check reasonable length
    if email.len() > 254 {
        return Err(ValidationError::EmailTooLong);
    }

    // Check has @ and domain
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(ValidationError::InvalidEmailFormat);
    }

    let (local, domain) = (parts[0], parts[1]);

    if local.is_empty() || domain.is_empty() {
        return Err(ValidationError::InvalidEmailFormat);
    }

    // Check domain has at least one dot
    if !domain.contains('.') {
        return Err(ValidationError::InvalidDomain);
    }

    Ok(())
}
```

### Testing Edge Cases

**Test Matrix**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_emails() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("a@b.co").is_ok());
    }

    #[test]
    fn test_edge_cases() {
        assert!(validate_email("").is_err());  // Empty
        assert!(validate_email("@").is_err());  // No local/domain
        assert!(validate_email("user@").is_err());  // No domain
        assert!(validate_email("@example.com").is_err());  // No local
        assert!(validate_email("user@nodomain").is_err());  // No TLD
        assert!(validate_email("user name@example.com").is_err());  // Space
    }

    #[test]
    fn test_unicode() {
        assert!(validate_email("üser@example.com").is_ok());  // Unicode in local
    }
}
```

### Discussion Points

- Finding edge cases systematically
- Property-based testing benefits
- Domain-specific validation rules
- Internationalization needs

---

## Question 4: Design Scalability

**Difficulty**: Hard
**Concepts**: Architecture, system design, growth planning
**Time**: 35-45 minutes

### Problem Statement

Design how a system scales as users/data grow from 100 to 1M users.

### Scenario: Social Network Feed

**Current System** (100 users):
- Single server
- SQL database
- All users connected to all users
- Feed loads all posts

### Scaling Challenges

1. **Database Scaling**
   - Database size grows
   - Query performance degrades
   - Replication/sharding needed

2. **Read/Write Split**
   - Many reads (viewing feed)
   - Fewer writes (posting)
   - Read replicas strategy

3. **Caching**
   - Cache popular feeds
   - Cache user data
   - Cache invalidation strategy

4. **Data Partitioning**
   - Shard by user ID
   - Shard by time (feeds)
   - Cross-shard queries

5. **Service Architecture**
   - Monolith vs microservices
   - Load balancing
   - Service discovery
   - API gateway

### Growth Phases

**Phase 1: 100 -> 1K Users**
- Single server sufficient
- Maybe add read replica
- Basic caching (Redis)

**Phase 2: 1K -> 10K Users**
- Multiple application servers
- Master-slave database replication
- Distributed caching

**Phase 3: 10K -> 100K Users**
- Database sharding
- Separate services (feed, user, messaging)
- Message queues for async operations

**Phase 4: 100K -> 1M Users**
- Multi-region deployment
- Cache-aside pattern
- Event streaming

### Architecture Decisions

1. **Consistency**
   - Strong vs eventual consistency?
   - Read-your-write guarantee?
   - Multi-region synchronization?

2. **Availability**
   - Redundancy requirements?
   - Failure handling?
   - Circuit breakers?

3. **Performance**
   - P99 latency targets?
   - Throughput requirements?
   - Cache hit rate goals?

### Discussion Points

- Trade-offs at each phase
- When to scale vs redesign
- Avoiding over-engineering
- Cost optimization

---

## Question 5: Plan Infrastructure Migration

**Difficulty**: Hard
**Concepts**: Operations, risk management, planning
**Time**: 35-45 minutes

### Problem Statement

Plan moving a production system from current infrastructure to new architecture with zero downtime.

### Scenario: Monolith to Microservices

**Current State**:
- Monolithic application
- PostgreSQL database
- 100 qps load
- 50,000 daily active users

**Target State**:
- User service
- Post service
- Feed service
- Search service
- Shared cache layer

### Migration Strategy

1. **Planning Phase**
   - Identify service boundaries
   - Plan data migration
   - Identify dependencies
   - Create rollback plan

2. **Parallel Running**
   - Run old and new systems side-by-side
   - Route subset of traffic to new
   - Gradually increase traffic percentage
   - Monitor both systems

3. **Data Migration**
   - Copy existing data to new format
   - Set up bidirectional sync (optional)
   - Verify data integrity
   - Handle incremental changes

4. **Service Decomposition**
   - Which service owns which data?
   - How to handle transactions across services?
   - Event bus for eventual consistency?
   - API contracts between services?

5. **Risk Mitigation**
   - Canary deployment
   - Feature flags for rollback
   - Health checks and monitoring
   - Incident response plan

### Example: User Service Migration

**Step 1: Set Up New Service**
- Build user-service
- Copy user data to new database
- Set up replication from old

**Step 2: Dual Writes**
- Monolith writes to both old and new
- Both systems see same updates
- Verify consistency

**Step 3: Gradual Traffic Shift**
- Route 5% of user requests to new service
- Monitor error rates and performance
- Increase to 10%, 25%, 50%, 100%

**Step 4: Cutover**
- Monolith stops writing to old user data
- All traffic goes to new service
- Keep old database for rollback

**Step 5: Cleanup**
- Monitor for period of time
- Decommission old user data
- Update dependent systems

### Monitoring and Alerting

**Key Metrics**:
- Request latency (p50, p95, p99)
- Error rates
- Service availability
- Database query times
- Cache hit rates

**Alerts**:
- Latency spike
- Error rate increase
- Service unavailability
- Data inconsistency

### Discussion Points

- Risk vs speed trade-offs
- Testing strategy during migration
- Handling failures mid-migration
- Communication with team

---

## Summary of System-Level Problems

These questions practice:
- Real-world production issues
- Debugging under uncertainty
- Performance optimization
- System design at scale
- Operational concerns

**Preparation**:
- Study real systems (Stripe, Uber, etc. blog posts)
- Practice root cause analysis
- Understand common failure modes
- Think about monitoring and observability
