// Exercise 4: Debug Output Statements to Find Bug (BROKEN CODE)
//
// This program compiles and runs but produces WRONG output.
// Your job: Use println! and dbg! to find the bug, then fix it.

fn main() {
    println!("Shopping Cart Calculator");
    println!("========================");

    // Item details
    let item_name = "Widget";
    let unit_price = 10.0_f64;
    let quantity = 5_u32;

    println!("Item: {} at ${:.2} x {}", item_name, unit_price, quantity);

    // Calculate the total
    let coupon_code = Some("SAVE15");
    let final_total = calculate_total(unit_price, quantity, coupon_code);

    println!("========================");
}

fn calculate_total(unit_price: f64, quantity: u32, coupon: Option<&str>) -> f64 {
    // Step 1: Calculate subtotal
    let subtotal = unit_price * quantity as f64;
    println!("Subtotal: ${:.2}", subtotal);

    // Step 2: Apply quantity discount (10% off for 3+ items)
    let after_quantity_discount = if quantity >= 3 {
        let discount = subtotal * 0.10;
        subtotal - discount
    } else {
        subtotal
    };
    println!("After quantity discount: ${:.2}", after_quantity_discount);

    // Step 3: Apply coupon discount
    let after_coupon = match coupon {
        Some("SAVE15") => {
            let discount = after_quantity_discount * 0.15;
            after_quantity_discount - discount
        }
        Some("SAVE10") => {
            let discount = after_quantity_discount * 0.10;
            after_quantity_discount - discount
        }
        _ => after_quantity_discount,
    };
    if coupon.is_some() {
        println!("After coupon ({}): ${:.2}", coupon.unwrap(), after_coupon);
    }

    // Step 4: Calculate tax (8%)
    let tax = after_coupon * 0.08;
    println!("Tax (8%): ${:.2}", tax);

    // BUG: This calculates tax but doesn't add it to the total!
    // The final_total should be after_coupon + tax
    let final_total = after_coupon;  // BUG: Should be after_coupon + tax

    println!("-----------");
    println!("TOTAL: ${:.2}", final_total);

    final_total
}

// DEBUGGING STEPS:
// 1. Run the program and see the wrong output
// 2. Add debug statements to trace values:
//    - dbg!(&subtotal);
//    - dbg!(&after_quantity_discount);
//    - dbg!(&after_coupon);
//    - dbg!(&tax);
//    - dbg!(&final_total);
// 3. Notice that tax is calculated but not added
// 4. Fix: Change `let final_total = after_coupon;`
//         to `let final_total = after_coupon + tax;`
// 5. Verify output is now correct

// EXPECTED OUTPUT AFTER FIX:
// Shopping Cart Calculator
// ========================
// Item: Widget at $10.00 x 5
// Subtotal: $50.00
// After quantity discount: $45.00
// After coupon (SAVE15): $38.25
// Tax (8%): $3.06
// -----------
// TOTAL: $41.31
// ========================

// THE BUG:
// Line 55: `let final_total = after_coupon;`
// Should be: `let final_total = after_coupon + tax;`
//
// The tax is calculated and printed, but never added to the final total.
// This is a classic "forgot to use the variable" bug.
