# Proxy: Key Takeaways

- Proxy and real subject implement the same trait.
- The proxy controls when and how the real subject is called.
- `RefCell` permits a cache behind an immutable trait method in this example.
- Document caching, authorization, retries, and other non-obvious behaviour.
