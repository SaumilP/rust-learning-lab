# Ownership in Rust

Ownership in Rust is most important concept. <br />
It enables **memory safety without a garbage collector**.

If you understand ownership, **Rust starts to feel simple**.

---

## 🎯 Learning Goals

By the end of this module, you will:

- Understand move vs copy
- Know why Rust prevents data races
- Predict compiler errors related to ownership
- Write functions that handle ownership correctly

---

## 🧠 Core rules

- Each value has a single owner
- There can only be one owner at a time
- When the owner goes out-of-scope, the value is dropped

---

## 📂 Topics covered

| Folder | Concept |
|:-----|:-------|
| move_sementics | _Why value move by default_ |
| copy_types | Stack-only types |
| functions_and_ownership | Passing values to functions |
| returning_ownership | Getting values back |

---

## ▶️ Run Example

```bash
cd move_semantics
cargo run
```

## ⚠️ Common Mistakes

- Assuming variables are copied by default
- Fighting the borrow checker instead of understanding it
- Overusing `.clone()`

---

## 🧪 Exercises

Located in `/exercises`:

- Fix ownership errors
- Refactor code to avoid cloning
- Predict compiler errors before running

---

## 🔍 Mental model

Think of ownership as:
> "Who is responsible for cleaning up this value?"
If the answer is unclear, the compiler will complain.

---

### Example: `move_semantics/src/main.rs`

```rust
fn main() {
    let s1 = String::from("hello")
    let s2 = s1;

    // println!("{}", s1);  // ❌ compile error
    println!("{}", s2);     // ✅
}
```
