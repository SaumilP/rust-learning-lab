/// Container Operations - Generics Example
///
/// Demonstrates:
/// - Generic structs and functions
/// - Generic trait bounds
/// - Generic implementations
/// - Monomorphization at compile time
///
/// Run with: cargo run --example container_operations

use std::fmt::Display;

/// Generic container that holds a single item
struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }

    fn get(&self) -> &T {
        &self.item
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.item
    }

    fn take(self) -> T {
        self.item
    }

    fn map<U, F>(self, f: F) -> Container<U>
    where
        F: FnOnce(T) -> U,
    {
        Container {
            item: f(self.item),
        }
    }
}

/// Implementation only for types that implement Clone
impl<T: Clone> Container<T> {
    fn clone_item(&self) -> T {
        self.item.clone()
    }
}

/// Implementation only for types that implement Display
impl<T: Display> Container<T> {
    fn print_item(&self) {
        println!("Container holds: {}", self.item);
    }
}

/// Generic pair of different types
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    fn first(&self) -> &T {
        &self.first
    }

    fn second(&self) -> &U {
        &self.second
    }

    fn swap<V, W>(self, other: Pair<V, W>) -> (Pair<V, W>, Pair<T, U>) {
        (other, self)
    }
}

/// Implementation for pairs where both types are the same
impl<T: Clone> Pair<T, T> {
    fn clone_both(&self) -> (T, T) {
        (self.first.clone(), self.second.clone())
    }
}

/// Implementation where both types implement Display
impl<T: Display, U: Display> Pair<T, U> {
    fn display_both(&self) {
        println!("Pair: ({}, {})", self.first, self.second);
    }
}

/// Generic function to find the largest element
fn find_largest<T: PartialOrd + Clone>(items: &[T]) -> Option<T> {
    if items.is_empty() {
        return None;
    }

    let mut largest = items[0].clone();
    for item in &items[1..] {
        if item > &largest {
            largest = item.clone();
        }
    }

    Some(largest)
}

/// Generic function to sum elements
fn sum_values<T>(items: &[T]) -> T
where
    T: std::ops::Add<Output = T> + Clone + Default,
{
    let mut total = T::default();
    for item in items {
        total = total + item.clone();
    }
    total
}

/// Generic function with where clause for complex bounds
fn process_and_display<T>(container: Container<T>)
where
    T: Display + Clone,
{
    println!("Processing: {}", container.get());
    let cloned = container.clone_item();
    println!("Cloned: {}", cloned);
}

fn demo_generic_container() {
    println!("\n╔════════════════════════════════╗");
    println!("║ GENERIC CONTAINER DEMO         ║");
    println!("╚════════════════════════════════╝");

    println!("\n--- Integer Container ---");
    let int_container = Container::new(42);
    println!("Integer: {}", int_container.get());

    let transformed = int_container.map(|x| x * 2);
    println!("After map (*2): {}", transformed.get());

    println!("\n--- String Container ---");
    let str_container = Container::new(String::from("Hello"));
    str_container.print_item();

    let cloned = str_container.clone_item();
    println!("Cloned: {}", cloned);

    println!("\n--- Float Container ---");
    let float_container = Container::new(3.14);
    let as_int = float_container.map(|x| x as i32);
    println!("Float mapped to int: {}", as_int.get());
}

fn demo_generic_pair() {
    println!("\n╔════════════════════════════════╗");
    println!("║ GENERIC PAIR DEMO              ║");
    println!("╚════════════════════════════════╝");

    println!("\n--- Different Types Pair ---");
    let pair = Pair::new(42, "Hello");
    println!("First: {}, Second: {}", pair.first(), pair.second());

    println!("\n--- Same Types Pair ---");
    let same_pair = Pair::new(5, 10);
    let (a, b) = same_pair.clone_both();
    println!("Cloned both: {} and {}", a, b);

    println!("\n--- Display Pair ---");
    let display_pair = Pair::new(100, "Test");
    display_pair.display_both();
}

fn demo_generic_functions() {
    println!("\n╔════════════════════════════════╗");
    println!("║ GENERIC FUNCTIONS DEMO         ║");
    println!("╚════════════════════════════════╝");

    println!("\n--- Find Largest ---");
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    if let Some(largest) = find_largest(&numbers) {
        println!("Largest number: {}", largest);
    }

    let chars = vec!['a', 'z', 'm', 'b'];
    if let Some(largest) = find_largest(&chars) {
        println!("Largest char: {}", largest);
    }

    println!("\n--- Sum Values ---");
    let numbers = vec![1, 2, 3, 4, 5];
    let total: i32 = sum_values(&numbers);
    println!("Sum of numbers: {}", total);

    let floats = vec![1.5, 2.5, 3.5];
    let total: f64 = sum_values(&floats);
    println!("Sum of floats: {}", total);
}

fn demo_monomorphization() {
    println!("\n╔════════════════════════════════╗");
    println!("║ MONOMORPHIZATION DEMO          ║");
    println!("╚════════════════════════════════╝");

    println!("\nGeneric code:");
    println!("fn largest<T: PartialOrd>(a: T, b: T) -> T {{ ... }}");

    println!("\nCompiler generates at compile time:");
    println!("fn largest_i32(a: i32, b: i32) -> i32 {{ ... }}");
    println!("fn largest_f64(a: f64, b: f64) -> f64 {{ ... }}");
    println!("fn largest_char(a: char, b: char) -> char {{ ... }}");

    println!("\nResult: Zero runtime overhead!");
    println!("Each type gets its own specialized version.");
}

fn main() {
    println!("╔════════════════════════════════╗");
    println!("║ GENERICS DEMO                  ║");
    println!("╚════════════════════════════════╝");

    println!("\nThis demonstrates generic structs and functions");
    println!("with various trait bounds and implementations.");

    demo_generic_container();
    demo_generic_pair();
    demo_generic_functions();

    println!("\n{}", "─".repeat(40));

    demo_monomorphization();

    println!("\n{}", "─".repeat(40));

    println!("\n╔════════════════════════════════╗");
    println!("║ GENERIC BENEFITS               ║");
    println!("╠════════════════════════════════╣");
    println!("║ ✓ Write once, use with many    ║");
    println!("║   different types              ║");
    println!("║                                ║");
    println!("║ ✓ Full type safety at compile  ║");
    println!("║   time (no runtime errors)     ║");
    println!("║                                ║");
    println!("║ ✓ Zero cost abstraction -      ║");
    println!("║   monomorphization handles it  ║");
    println!("║                                ║");
    println!("║ ✓ Combine with traits for      ║");
    println!("║   constrained polymorphism     ║");
    println!("╚════════════════════════════════╝");
}
