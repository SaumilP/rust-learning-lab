// Example: Clone and Copy Traits
//
// Demonstrates:
// - Copy trait for bitwise copying (stack-only types)
// - Clone trait for explicit duplication
// - When to use each trait
// - Implementing Clone for custom types

fn main() {
    println!("=== Copy Trait - Automatic Copying ===\n");

    // Primitive types implement Copy
    let x = 5;
    let y = x; // Copy happens automatically
    println!("x = {}, y = {} (both valid - x was copied)", x, y);

    let a = 3.14;
    let b = a; // f64 implements Copy
    println!("a = {}, b = {} (both valid - a was copied)", a, b);

    let flag = true;
    let flag2 = flag; // bool implements Copy
    println!("flag = {}, flag2 = {} (both valid)", flag, flag2);

    let c = 'R';
    let d = c; // char implements Copy
    println!("c = '{}', d = '{}' (both valid)", c, d);

    println!("\n=== Types That Don't Implement Copy ===\n");

    // String does NOT implement Copy (heap allocated)
    let s1 = String::from("hello");
    let s2 = s1; // Move, not copy
                 // println!("{}", s1);  // ERROR: s1 was moved
    println!("s2 = '{}' (s1 was moved, not copied)", s2);

    // Vec does NOT implement Copy
    let v1 = vec![1, 2, 3];
    let v2 = v1; // Move, not copy
                 // println!("{:?}", v1);  // ERROR: v1 was moved
    println!("v2 = {:?} (v1 was moved)", v2);

    println!("\n=== Clone Trait - Explicit Duplication ===\n");

    // Use clone() to explicitly duplicate
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Explicit clone
    println!("s1 = '{}', s2 = '{}' (both valid after clone)", s1, s2);

    let v1 = vec![1, 2, 3];
    let v2 = v1.clone(); // Explicit clone
    println!("v1 = {:?}, v2 = {:?} (both valid)", v1, v2);

    // Modifying clone doesn't affect original
    let mut v3 = v1.clone();
    v3.push(4);
    println!("\nOriginal v1: {:?}", v1);
    println!("Modified clone v3: {:?}", v3);

    println!("\n=== Copy vs Clone for Different Types ===\n");

    // Arrays of Copy types are Copy
    let arr1 = [1, 2, 3];
    let arr2 = arr1; // Copy (because i32 is Copy)
    println!(
        "arr1 = {:?}, arr2 = {:?} (both valid - array was copied)",
        arr1, arr2
    );

    // Tuples of Copy types are Copy
    let tup1 = (1, 2.0, 'a');
    let tup2 = tup1; // Copy
    println!("tup1 = {:?}, tup2 = {:?} (both valid)", tup1, tup2);

    // Tuples with non-Copy types need clone
    let tup3 = (1, String::from("hello"));
    let tup4 = tup3.clone();
    // let tup4 = tup3;  // Would move tup3
    println!("tup3 (cloned) = {:?}", tup4);

    println!("\n=== Custom Types with Copy ===\n");

    // Copy can only be derived for types where all fields are Copy
    #[allow(dead_code)]
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // Copy
    println!("p1 = {:?}, p2 = {:?} (both valid - Point is Copy)", p1, p2);

    // Can also still use clone
    let p3 = p1.clone();
    println!("p3 (cloned) = {:?}", p3);

    println!("\n=== Custom Types with Clone Only ===\n");

    // Types with heap data can't be Copy, but can be Clone
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let person2 = person1.clone(); // Must use clone
    println!("person1 = {:?}", person1);
    println!("person2 = {:?}", person2);

    // Modify clone
    let mut person3 = person1.clone();
    person3.name = String::from("Bob");
    person3.age = 25;
    println!("\nModified clone person3 = {:?}", person3);
    println!("Original person1 unchanged = {:?}", person1);

    println!("\n=== Manual Clone Implementation ===\n");

    #[derive(Debug)]
    struct ComplexData {
        id: u32,
        data: Vec<i32>,
    }

    impl Clone for ComplexData {
        fn clone(&self) -> Self {
            println!("  (Custom clone called for id {})", self.id);
            ComplexData {
                id: self.id,
                data: self.data.clone(),
            }
        }
    }

    let data1 = ComplexData {
        id: 1,
        data: vec![10, 20, 30],
    };

    println!("Cloning data1:");
    let data2 = data1.clone();
    println!("data1 = {:?}", data1);
    println!("data2 = {:?}", data2);

    println!("\n=== Clone in Collections ===\n");

    // Cloning vectors
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = v1.clone();
    println!("v1 = {:?}", v1);
    println!("v2 (clone) = {:?}", v2);

    // Cloning nested structures
    let nested = vec![vec![1, 2], vec![3, 4]];
    let nested_clone = nested.clone();
    println!("\nnested = {:?}", nested);
    println!("nested_clone = {:?}", nested_clone);

    // Cloning HashMaps
    use std::collections::HashMap;
    let mut map1: HashMap<&str, i32> = HashMap::new();
    map1.insert("a", 1);
    map1.insert("b", 2);

    let map2 = map1.clone();
    println!("\nmap1 = {:?}", map1);
    println!("map2 (clone) = {:?}", map2);

    println!("\n=== When to Use Clone vs Copy ===\n");

    // Use Copy for simple, stack-only data
    // - Faster (no function call, just memory copy)
    // - Implicit (no .clone() needed)
    #[allow(dead_code)]
    #[derive(Copy, Clone, Debug)]
    struct Color(u8, u8, u8); // RGB, all u8 (Copy)

    let red = Color(255, 0, 0);
    let red_copy = red; // Implicit copy
    println!("red = {:?}, red_copy = {:?}", red, red_copy);

    // Use Clone for complex data
    // - Explicit control over when duplication happens
    // - Required for heap-allocated data
    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    struct Document {
        title: String,
        content: String,
    }

    let doc = Document {
        title: String::from("Report"),
        content: String::from("...long content..."),
    };
    let doc_backup = doc.clone(); // Explicit clone
    println!("\ndoc.title = '{}'", doc.title);
    println!("doc_backup.title = '{}'", doc_backup.title);

    println!("\n=== Clone in Function Parameters ===\n");

    fn process_data(data: Vec<i32>) {
        println!("  Processing: {:?}", data);
    }

    let original = vec![1, 2, 3];
    println!("Before function call: {:?}", original);

    // Without clone - original would be moved
    process_data(original.clone()); // Clone to keep original
    println!("After function call: {:?}", original);

    // Or design function to take reference
    fn process_ref(data: &[i32]) {
        println!("  Processing ref: {:?}", data);
    }
    process_ref(&original); // No clone needed
    println!("Still have original: {:?}", original);
}
