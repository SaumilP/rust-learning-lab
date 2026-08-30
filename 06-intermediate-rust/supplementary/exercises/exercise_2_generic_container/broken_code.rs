/// Broken Generic Container - Fix the bugs!

struct Box<T> {
    item: T,
}

impl<T> Box<T> {
    fn new(item: T) -> Self {
        Box { item }
    }

    fn get(&self) -> &T {
        &self.item
    }

    // ❌ BUG 1: Missing Clone bound
    fn get_copy(&self) -> T {
        self.item.clone()  // ERROR: T might not implement Clone
    }

    fn map<U, F>(self, f: F) -> Box<U>
    where
        // ❌ BUG 2: Wrong bound - should be FnOnce
        F: Fn(T) -> U,
    {
        Box {
            item: f(self.item),
        }
    }
}

// ❌ BUG 3: Missing where clause for Display bound
impl<T> Box<T> {
    fn display_item(&self) {
        println!("Item: {}", self.item);  // ERROR: T doesn't implement Display
    }
}

fn main() {
    let int_box = Box::new(42);
    println!("Integer: {}", int_box.get());

    let str_box = Box::new("Hello".to_string());
    println!("String: {}", str_box.get());

    let mapped = int_box.map(|x| x * 2);
    println!("Mapped: {}", mapped.get());
}
