# Hints for Exercise 2: Generic Container

## Bug 1: Missing Clone Bound

**Issue**: `get_copy()` calls clone() without requiring Clone trait

**Fix**:
```rust
impl<T: Clone> Box<T> {
    fn get_copy(&self) -> T {
        self.item.clone()
    }
}
```

## Bug 2: Wrong Function Bound

**Issue**: Uses `Fn` but should use `FnOnce` since we take ownership

**Fix**:
```rust
fn map<U, F>(self, f: F) -> Box<U>
where
    F: FnOnce(T) -> U,  // Takes ownership once
```

## Bug 3: Missing Display Bound

**Issue**: Tries to format T without requiring Display

**Fix**:
```rust
impl<T: Display> Box<T> {
    fn display_item(&self) {
        println!("Item: {}", self.item);
    }
}

use std::fmt::Display;
```

## Testing

- Create Box with different types
- Use clone/copy operations
- Apply map transformations
- Display only works for Display types
