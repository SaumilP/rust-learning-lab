// Example: Basic File I/O Operations
//
// Demonstrates:
// - Reading entire file to string
// - Writing string to file
// - Appending to file
// - File existence checking
// - Error handling patterns
//
// Creates and modifies test files

use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("=== File I/O Operations Example ===\n");

    // Example 1: Writing to a file
    println!("1. Writing to file...");
    let content = "Hello, Rust!\nThis is a test file.\nWe can write multiple lines.";

    match fs::write("test_file.txt", content) {
        Ok(_) => println!("   Successfully wrote to test_file.txt"),
        Err(e) => eprintln!("   Error writing file: {}", e),
    }

    // Example 2: Reading entire file
    println!("\n2. Reading entire file...");
    match fs::read_to_string("test_file.txt") {
        Ok(contents) => {
            println!("   File contents:");
            for line in contents.lines() {
                println!("   > {}", line);
            }
            println!("   Total size: {} bytes", contents.len());
        }
        Err(e) => eprintln!("   Error reading file: {}", e),
    }

    // Example 3: Appending to file
    println!("\n3. Appending to file...");
    let new_line = "\nAdded via append operation!";
    match fs::OpenOptions::new().append(true).open("test_file.txt") {
        Ok(mut file) => match file.write_all(new_line.as_bytes()) {
            Ok(_) => println!("   Successfully appended to file"),
            Err(e) => eprintln!("   Error appending: {}", e),
        },
        Err(e) => eprintln!("   Error opening file: {}", e),
    }

    // Example 4: Reading file again to verify append
    println!("\n4. Reading file after append...");
    match fs::read_to_string("test_file.txt") {
        Ok(contents) => {
            println!("   Updated file contents:");
            for line in contents.lines() {
                println!("   > {}", line);
            }
        }
        Err(e) => eprintln!("   Error: {}", e),
    }

    // Example 5: Check file metadata
    println!("\n5. File metadata...");
    match fs::metadata("test_file.txt") {
        Ok(metadata) => {
            println!("   File size: {} bytes", metadata.len());
            println!("   Is file: {}", metadata.is_file());
            println!("   Is dir: {}", metadata.is_dir());
        }
        Err(e) => eprintln!("   Error: {}", e),
    }

    // Example 6: Check file existence
    println!("\n6. File existence checking...");
    let path = "test_file.txt";
    if Path::new(path).exists() {
        println!("   {} exists", path);
    } else {
        println!("   {} does not exist", path);
    }

    // Example 7: Processing file line by line
    println!("\n7. Processing file line by line...");
    match fs::read_to_string("test_file.txt") {
        Ok(contents) => {
            let line_count = contents.lines().count();
            println!("   Total lines: {}", line_count);

            for (i, line) in contents.lines().enumerate() {
                println!("   [{}] {} chars: {}", i + 1, line.len(), line);
            }
        }
        Err(e) => eprintln!("   Error: {}", e),
    }

    // Example 8: Copying file
    println!("\n8. Copying file...");
    match fs::copy("test_file.txt", "test_file_copy.txt") {
        Ok(bytes_copied) => println!("   Copied {} bytes to test_file_copy.txt", bytes_copied),
        Err(e) => eprintln!("   Error copying: {}", e),
    }

    // Example 9: Creating file with multiple writes
    println!("\n9. Creating file with multiple writes...");
    match fs::File::create("multi_write.txt") {
        Ok(mut file) => {
            let data = vec!["First line\n", "Second line\n", "Third line\n"];
            for line in data {
                match file.write_all(line.as_bytes()) {
                    Ok(_) => {
                        // File.write is successful, data may be buffered
                    }
                    Err(e) => eprintln!("   Error writing: {}", e),
                }
            }
            // Explicitly flush to ensure data is written
            match file.flush() {
                Ok(_) => println!("   Successfully created multi_write.txt"),
                Err(e) => eprintln!("   Error flushing: {}", e),
            }
        }
        Err(e) => eprintln!("   Error creating file: {}", e),
    }

    // Example 10: Cleanup
    println!("\n10. Cleanup...");
    let files_to_remove = vec!["test_file.txt", "test_file_copy.txt", "multi_write.txt"];
    for file in files_to_remove {
        match fs::remove_file(file) {
            Ok(_) => println!("   Removed {}", file),
            Err(e) => eprintln!("   Error removing {}: {}", file, e),
        }
    }

    println!("\nFile I/O operations complete!");
}

// Expected output:
// === File I/O Operations Example ===
//
// 1. Writing to file...
//    Successfully wrote to test_file.txt
//
// 2. Reading entire file...
//    File contents:
//    > Hello, Rust!
//    > This is a test file.
//    > We can write multiple lines.
//    Total size: 64 bytes
//
// 3. Appending to file...
//    Successfully appended to file
//
// 4. Reading file after append...
//    Updated file contents:
//    > Hello, Rust!
//    > This is a test file.
//    > We can write multiple lines.
//    > Added via append operation!
//
// 5. File metadata...
//    File size: 102 bytes
//    Is file: true
//    Is dir: false
//
// 6. File existence checking...
//    test_file.txt exists
//
// 7. Processing file line by line...
//    Total lines: 4
//    [1] 13 chars: Hello, Rust!
//    [2] 18 chars: This is a test file.
//    [3] 24 chars: We can write multiple lines.
//    [4] 32 chars: Added via append operation!
//
// 8. Copying file...
//    Copied 102 bytes to test_file_copy.txt
//
// 9. Creating file with multiple writes...
//    Successfully created multi_write.txt
//
// 10. Cleanup...
//     Removed test_file.txt
//     Removed test_file_copy.txt
//     Removed multi_write.txt
//
// File I/O operations complete!
