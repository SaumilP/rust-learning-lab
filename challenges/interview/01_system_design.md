# Interview Questions: System Design

## Question 1: Design a URL Shortener

**Difficulty**: Medium
**Concepts**: Distributed systems, database design, encoding
**Time**: 20-30 minutes

### Problem Statement

Design a URL shortening service (like bit.ly or tinyurl) that converts long URLs into short, unique identifiers.

### Requirements

**Functional**:
- Store mappings from short URL → long URL
- Given a long URL, generate a unique short identifier
- Redirect from short URL to original long URL
- Track creation time and access statistics (optional)

**Non-functional**:
- Support millions of shortened URLs
- High read traffic (more reads than writes)
- Low latency for redirects
- Unique identifiers should be as short as possible

### Design Questions to Answer

1. **Data Model**
   - What data do we store? (ID, long URL, creation time, expiry?)
   - Should URLs expire?
   - Do we track analytics?

2. **Encoding Strategy**
   - How do we generate short codes? (sequential, hash-based, random?)
   - How long should codes be to ensure uniqueness?
   - Character set for codes (alphanumeric, case-sensitive?)

3. **Database Design**
   - SQL vs NoSQL?
   - Primary key strategy?
   - Indexes needed?
   - Sharding strategy if distributed?

4. **API Design**
   - Endpoints: POST /shorten, GET /<short_code>
   - Request/response format?
   - Error handling?

5. **Scale Considerations**
   - Handling collisions?
   - Horizontal scaling approach?
   - Cache layer (Redis)?
   - Database replication?

6. **Security**
   - Prevention of malicious URLs?
   - Rate limiting?
   - Authentication for API?

### Rust-Specific Implementation Hints

- Use HashMap or BTreeMap for in-memory storage
- Implement trait for ID generation (trait-based strategy pattern)
- Consider async/await with tokio for concurrent requests
- Use Result<T, E> for error handling

### Discussion Points

- Trade-offs between hash-based vs sequential IDs
- Read-heavy vs write-heavy optimization
- Cache invalidation strategy
- Geographic distribution

---

## Question 2: Design a Rate Limiter

**Difficulty**: Medium
**Concepts**: Concurrency, state management, algorithms
**Time**: 20-30 minutes

### Problem Statement

Design a rate limiter that restricts the number of requests a client can make in a given time window.

### Requirements

**Functional**:
- Allow configurable requests per time window (e.g., 100 per minute)
- Track requests per user/IP
- Return success/failure for each request
- Support multiple rate limit strategies

**Non-functional**:
- Low latency decision (sub-millisecond)
- Support millions of concurrent users
- Minimal memory overhead
- Accurate limiting

### Design Questions to Answer

1. **Strategies**
   - Token bucket algorithm?
   - Sliding window?
   - Fixed window?
   - Leaky bucket?

2. **State Management**
   - How to track request counts per user?
   - In-memory vs distributed state?
   - Expiration of old entries?

3. **Distributed System**
   - Multiple servers/processes?
   - Shared state (Redis, database)?
   - Consistency guarantees?

4. **API Design**
   - `allow_request(user_id, timestamp) -> bool`
   - Configuration structure?
   - Return additional info (remaining quota)?

5. **Edge Cases**
   - Clock skew/synchronization?
   - Burst handling?
   - User cleanup (old entries)?

### Rust-Specific Implementation Hints

- Use HashMap<UserId, RequestTracker> for state
- Implement Arc<Mutex<>> for thread-safe shared state
- Consider using channels for distributed version
- Implement custom algorithms as trait impls

### Discussion Points

- Algorithm trade-offs (accuracy vs efficiency)
- Memory usage with millions of users
- Handling clock skew in distributed systems
- Testing different strategies

---

## Question 3: Design a Cache Layer

**Difficulty**: Medium-Hard
**Concepts**: Cache policies, performance, data structures
**Time**: 25-35 minutes

### Problem Statement

Design a caching layer for a web application that stores frequently accessed data to reduce latency.

### Requirements

**Functional**:
- Store key-value pairs
- Set/get operations
- Configure cache size limit
- Eviction policy when full

**Non-functional**:
- Fast get/set operations (O(1) ideal)
- Memory efficient
- Thread-safe
- Support different eviction policies

### Design Questions to Answer

1. **Data Structure**
   - HashMap for fast lookups?
   - Additional structures for eviction tracking?
   - Ordering requirements?

2. **Eviction Policies**
   - LRU (Least Recently Used)?
   - LFU (Least Frequently Used)?
   - FIFO?
   - TTL (Time To Live)?

3. **Concurrency**
   - Multiple readers/writers?
   - Lock granularity?
   - Read-write locks?

4. **Invalidation**
   - Manual invalidation?
   - TTL-based expiration?
   - Version tracking?

5. **Stats & Monitoring**
   - Hit/miss ratio?
   - Size tracking?
   - Eviction statistics?

### Rust-Specific Implementation Hints

- HashMap + LinkedList for LRU
- Arc<RwLock<>> for concurrent access
- Use enums for different eviction policies
- Implement trait for policy switching

### Discussion Points

- LRU vs LFU trade-offs
- Single-threaded vs multi-threaded performance
- Memory overhead of tracking
- Hit ratio vs memory size trade-off

---

## Question 4: Design a Load Balancer

**Difficulty**: Hard
**Concepts**: Networking, distributed systems, algorithms
**Time**: 30-40 minutes

### Problem Statement

Design a load balancer that distributes incoming requests across multiple backend servers.

### Requirements

**Functional**:
- Route requests to available servers
- Support multiple load balancing algorithms
- Health checking of backend servers
- Handle server failures

**Non-functional**:
- Low latency routing
- High throughput
- Scalable to many servers
- Reliable/fault-tolerant

### Design Questions to Answer

1. **Routing Algorithms**
   - Round-robin?
   - Least connections?
   - Weighted distribution?
   - Consistent hashing?

2. **Health Checking**
   - How often to check?
   - Failure detection?
   - Automatic removal/recovery?
   - Graceful degradation?

3. **Session Persistence**
   - Sticky sessions needed?
   - Session state management?
   - Failover behavior?

4. **Architecture**
   - Single vs multiple load balancers?
   - Heartbeat protocol?
   - Configuration management?

5. **Metrics**
   - Request counts per server?
   - Response time tracking?
   - Error rate monitoring?

### Rust-Specific Implementation Hints

- Use Vec/HashMap of server connections
- Implement algorithms as traits
- Use channels for inter-component communication
- Consider tokio for async I/O

### Discussion Points

- Algorithm selection for different use cases
- Consistency in distributed load balancing
- Failure scenarios and recovery
- Performance optimization

---

## Question 5: Design a Logging System

**Difficulty**: Medium
**Concepts**: I/O, buffering, formatting
**Time**: 20-30 minutes

### Problem Statement

Design a logging system for an application that efficiently records and stores log events.

### Requirements

**Functional**:
- Log events with different severity levels
- Format logs consistently
- Store to multiple destinations (file, stdout, remote)
- Configurable filtering

**Non-functional**:
- Low overhead (shouldn't impact app performance)
- Thread-safe
- Async logging support
- Buffering for efficiency

### Design Questions to Answer

1. **Architecture**
   - Global singleton vs injected logger?
   - Async vs synchronous?
   - Separate thread for I/O?

2. **Log Levels**
   - DEBUG, INFO, WARN, ERROR, FATAL?
   - Runtime filtering?
   - Per-module levels?

3. **Formatting**
   - Timestamp format?
   - Contextual info (module, line number)?
   - Structured logging?

4. **Destinations**
   - File rotation?
   - Multiple outputs simultaneously?
   - Remote logging?

5. **Performance**
   - Buffering strategy?
   - Batch writes?
   - Lock contention?

### Rust-Specific Implementation Hints

- Use Arc<Mutex<>> for shared logger
- Implement writer trait for pluggability
- Consider crossbeam::channel for async
- Use macros for log! calls

### Discussion Points

- Synchronous vs asynchronous logging trade-offs
- Buffer size selection
- File rotation strategies
- Structured vs unstructured logging

---

## Summary of System Design Questions

These questions practice:
- Large-scale system thinking
- Trade-off analysis
- API design
- Scalability considerations
- Implementation strategies in Rust

**Preparation**:
- Draw diagrams during design
- Think aloud about trade-offs
- Ask clarifying questions
- Discuss alternative approaches

**For Each Question**:
1. Clarify requirements
2. High-level design
3. Detailed design
4. Consider scale/edge cases
5. Discuss improvements
