# Unsafe Safety Contracts

`unsafe` is a promise made by the author, not a performance switch. Safe callers rely on the code at an unsafe boundary to preserve Rust's aliasing, lifetime, initialization, and thread-safety rules. Put the smallest possible unsafe operation behind a safe API whenever that API can enforce the preconditions.

## A contract before an unsafe block

Write down the contract immediately above the block or unsafe function. It should identify the invariants, who establishes them, how they are checked, and what could make the operation unsound. A useful contract is concrete: “the pointer was produced by this allocation, is aligned for `T`, names `len` initialized elements, and no mutable alias exists for the returned slice.” “Caller must be careful” is not a contract.

Use a safe abstraction when the standard library already provides one. `OnceLock` replaces most hand-written `static mut` initialization. `Mutex`, atomics, channels, and scoped threads cover common shared-state patterns without a custom safety proof. FFI belongs in a narrow wrapper that owns conversion, null checks, error translation, and resource cleanup.

## Review checklist

- State every precondition and whether the caller or wrapper verifies it.
- Keep raw pointers, `static mut`, inline assembly, and FFI calls out of ordinary business logic.
- Explain ownership across each FFI boundary, including which side frees each allocation.
- Validate nullability, lengths, alignment, UTF-8 or encoding assumptions, and integer conversions before dereferencing or constructing references.
- Document thread-affinity, reentrancy, and callback lifetime rules for foreign libraries.
- Test the safe wrapper’s error paths; use Miri or sanitizers when the project and platform support them.
- Prefer an `unsafe fn` only when its caller genuinely must uphold a precondition. Otherwise expose a safe function and contain the block inside it.

## Architecture boundary

This repository’s runnable advanced examples do not require FFI or raw-pointer implementations. The SIMD snippets in [performance optimization](../../07-advanced-concepts/performance_optimization/README.md) are illustrative platform-specific sketches, not complete implementations or a recommendation to bypass portable code. Any future runnable SIMD or FFI example must include its contract beside the unsafe operation, a portable fallback, target detection where applicable, and a command that checks the safe path.

## Singleton initialization

Do not use `static mut` plus `Once` for an ordinary singleton. The singleton guide now uses `OnceLock`, whose API expresses one-time initialization without exposing mutable global state. This removes an unnecessary unsafe boundary rather than trying to document around it.
