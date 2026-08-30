# Send and thread boundaries

The function wants to move shared ownership of a number into a new thread. Predict why `broken.rs` fails before compiling it.

Hint: not every shared-pointer type is safe to transfer to another thread.

`Rc<T>` is single-threaded reference counting and does not implement `Send`, so the compiler reports E0277 when the closure is passed to `thread::spawn`. `fixed.rs` uses `Arc<T>`, whose atomic reference count is designed for cross-thread sharing.

An alternative is to keep `Rc<T>` when all ownership remains in one thread. `Arc` solves a thread-sharing requirement; it is not a drop-in performance upgrade for every `Rc`.
