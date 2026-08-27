// Exercise 4: Nested Loop Pattern Generation (BROKEN CODE)
//
// This code has 3 bugs related to nested loops, conditions, and string building
// Your job: Find and fix the bugs so it compiles and produces correct output

fn main() {
    let size = 5;

    println!("=== Pattern 1: Right Triangle (Size {}) ===", size);
    print_right_triangle(size);

    println!("\n=== Pattern 2: Hollow Square (Size {}) ===", size);
    print_hollow_square(size);

    println!("\n=== Pattern 3: Diamond (Size {}) ===", size);
    print_diamond(size);
}

fn print_right_triangle(size: i32) {
    // BUG 1: Loop condition is wrong - should be i < size
    for i in 1..size {  // This only goes to size-1, not size
        for j in 0..=i {
            print!("*");
        }
        println!();
    }
}

fn print_hollow_square(size: i32) {
    for i in 0..size {
        let mut row = String::new();
        for j in 0..size {
            // BUG 2: Logic condition is wrong - should check if it's NOT an edge
            // The condition should be: (i > 0 && i < size - 1) && (j > 0 && j < size - 1)
            if i == 0 || i == size - 1 || j == 0 || j == size - 1 {
                row.push_str("*");
            } else {
                row.push_str(" ");
            }
        }
        println!("{}", row);
    }
}

fn print_diamond(size: i32) {
    let half = size / 2;

    // Upper half of diamond
    for i in 0..size {
        let mut row = String::new();

        // BUG 3: Spacing calculation is wrong
        // The condition uses i directly, but should account for the center point
        // Should be: let spaces = if i <= half { half - i } else { i - half };
        let spaces = i;  // This gives incorrect spacing

        // Add leading spaces
        for _ in 0..spaces {
            row.push(' ');
        }

        // Calculate stars
        let stars = 2 * i + 1;
        for _ in 0..stars {
            row.push('*');
        }

        println!("{}", row);
    }

    // Lower half of diamond (should mirror upper half)
    for i in (0..half).rev() {
        let mut row = String::new();
        let spaces = half - i;

        // Add leading spaces
        for _ in 0..spaces {
            row.push(' ');
        }

        // Calculate stars
        let stars = 2 * i + 1;
        for _ in 0..stars {
            row.push('*');
        }

        println!("{}", row);
    }
}

// BUGS SUMMARY:
// 1. Right triangle loop: 1..size doesn't include size, should be 1..=size or 0..size with adjustment
// 2. Hollow square logic: condition prints asterisks at borders but doesn't handle the logic correctly
//    Actually the condition IS correct - it prints * at borders and spaces inside
//    The real issue might be the loop ranges or spacing
// 3. Diamond spacing: spaces = i doesn't create proper diamond - needs half-based calculation

// EXPECTED BEHAVIOR:
// === Pattern 1: Right Triangle (Size 5) ===
// *
// **
// ***
// ****
// *****
//
// === Pattern 2: Hollow Square (Size 5) ===
// *****
// *   *
// *   *
// *   *
// *****
//
// === Pattern 3: Diamond (Size 5) ===
//     *
//    ***
//   *****
//  *******
// *********
//  *******
//   *****
//    ***
//     *
