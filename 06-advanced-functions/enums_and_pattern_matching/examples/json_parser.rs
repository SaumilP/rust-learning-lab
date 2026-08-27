/// JSON Parser - Enums and Pattern Matching Example
///
/// Demonstrates:
/// - Enums with associated data
/// - Pattern matching on enums
/// - Recursive data structures
/// - Complex pattern matching with guards
///
/// Run with: cargo run --example json_parser

use std::collections::HashMap;

/// JSON value types
#[derive(Debug, Clone, PartialEq)]
enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    /// Convert JSON value to pretty string
    fn to_pretty_string(&self, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        let inner_indent_str = " ".repeat(indent + 2);

        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Boolean(b) => b.to_string(),
            JsonValue::Number(n) => {
                if n.fract() == 0.0 && *n >= 0.0 && *n <= 9_007_199_254_740_992.0 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            }
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Array(arr) => {
                if arr.is_empty() {
                    "[]".to_string()
                } else {
                    let items = arr
                        .iter()
                        .map(|v| format!("{}{}", inner_indent_str, v.to_pretty_string(indent + 2)))
                        .collect::<Vec<_>>()
                        .join(",\n");
                    format!("[\n{}\n{}]", items, indent_str)
                }
            }
            JsonValue::Object(obj) => {
                if obj.is_empty() {
                    "{}".to_string()
                } else {
                    let items = obj
                        .iter()
                        .map(|(k, v)| {
                            format!(
                                "{}\"{}\": {}",
                                inner_indent_str,
                                k,
                                v.to_pretty_string(indent + 2)
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",\n");
                    format!("{{\n{}\n{}}}", items, indent_str)
                }
            }
        }
    }

    /// Get value at path using pattern matching
    fn get_path(&self, path: &[&str]) -> Option<JsonValue> {
        match (self, path) {
            // Base case: empty path
            (value, []) => Some(value.clone()),

            // Array access with index
            (JsonValue::Array(arr), [index_str, rest @ ..]) => {
                if let Ok(index) = index_str.parse::<usize>() {
                    arr.get(index).and_then(|v| v.get_path(rest))
                } else {
                    None
                }
            }

            // Object access with key
            (JsonValue::Object(obj), [key, rest @ ..]) => {
                obj.get(*key).and_then(|v| v.get_path(rest))
            }

            _ => None,
        }
    }

    /// Count all values in JSON structure
    fn count_values(&self) -> usize {
        match self {
            JsonValue::Null | JsonValue::Boolean(_) | JsonValue::Number(_) | JsonValue::String(_) => 1,
            JsonValue::Array(arr) => 1 + arr.iter().map(|v| v.count_values()).sum::<usize>(),
            JsonValue::Object(obj) => {
                1 + obj
                    .values()
                    .map(|v| v.count_values())
                    .sum::<usize>()
            }
        }
    }

    /// Get type of value
    fn type_name(&self) -> &'static str {
        match self {
            JsonValue::Null => "null",
            JsonValue::Boolean(_) => "boolean",
            JsonValue::Number(_) => "number",
            JsonValue::String(_) => "string",
            JsonValue::Array(_) => "array",
            JsonValue::Object(_) => "object",
        }
    }
}

fn demo_basic_values() {
    println!("\n╔════════════════════════════════╗");
    println!("║ BASIC JSON VALUES              ║");
    println!("╚════════════════════════════════╝");

    let values = vec![
        JsonValue::Null,
        JsonValue::Boolean(true),
        JsonValue::Boolean(false),
        JsonValue::Number(42.0),
        JsonValue::Number(3.14),
        JsonValue::String("Hello".to_string()),
    ];

    for value in values {
        println!("Type: {:10} | Value: {}", value.type_name(), value.to_pretty_string(0));
    }
}

fn demo_arrays() {
    println!("\n╔════════════════════════════════╗");
    println!("║ JSON ARRAYS                    ║");
    println!("╚════════════════════════════════╝");

    let arr = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Number(3.0),
        JsonValue::String("test".to_string()),
    ]);

    println!("Array:\n{}", arr.to_pretty_string(0));
    println!("\nValue count: {}", arr.count_values());

    // Pattern match to extract values
    match &arr {
        JsonValue::Array(items) => {
            println!("\nFirst item:");
            match items.first() {
                Some(JsonValue::Number(n)) => println!("  Number: {}", n),
                Some(JsonValue::String(s)) => println!("  String: {}", s),
                Some(other) => println!("  Type: {}", other.type_name()),
                None => println!("  Empty array"),
            }
        }
        _ => println!("Not an array"),
    }
}

fn demo_objects() {
    println!("\n╔════════════════════════════════╗");
    println!("║ JSON OBJECTS                   ║");
    println!("╚════════════════════════════════╝");

    let mut obj = HashMap::new();
    obj.insert("name".to_string(), JsonValue::String("Alice".to_string()));
    obj.insert("age".to_string(), JsonValue::Number(30.0));
    obj.insert("active".to_string(), JsonValue::Boolean(true));

    let json = JsonValue::Object(obj);
    println!("Object:\n{}", json.to_pretty_string(0));

    // Pattern match to extract object fields
    match &json {
        JsonValue::Object(fields) if fields.len() == 3 => {
            println!("\nObject has exactly 3 fields:");
            for (key, value) in fields {
                println!("  {}: {}", key, value.type_name());
            }
        }
        _ => {}
    }
}

fn demo_nested_structures() {
    println!("\n╔════════════════════════════════╗");
    println!("║ NESTED STRUCTURES              ║");
    println!("╚════════════════════════════════╝");

    let mut users = Vec::new();

    let mut alice = HashMap::new();
    alice.insert("name".to_string(), JsonValue::String("Alice".to_string()));
    alice.insert("age".to_string(), JsonValue::Number(30.0));
    users.push(JsonValue::Object(alice));

    let mut bob = HashMap::new();
    bob.insert("name".to_string(), JsonValue::String("Bob".to_string()));
    bob.insert("age".to_string(), JsonValue::Number(25.0));
    users.push(JsonValue::Object(bob));

    let json = JsonValue::Array(users);
    println!("Users:\n{}", json.to_pretty_string(0));

    println!("\nTotal values: {}", json.count_values());

    // Get path
    if let Some(first_name) = json.get_path(&["0", "name"]) {
        println!("First user's name: {}", first_name.to_pretty_string(0));
    }
}

fn demo_pattern_matching() {
    println!("\n╔════════════════════════════════╗");
    println!("║ PATTERN MATCHING               ║");
    println!("╚════════════════════════════════╝");

    let values = vec![
        JsonValue::Null,
        JsonValue::Boolean(true),
        JsonValue::Number(42.5),
        JsonValue::String("test".to_string()),
        JsonValue::Array(vec![]),
        JsonValue::Object(HashMap::new()),
    ];

    for value in values {
        match value {
            JsonValue::Null => println!("Null value"),
            JsonValue::Boolean(b) => println!("Boolean: {}", b),
            JsonValue::Number(n) if n.fract() == 0.0 => println!("Integer: {:.0}", n),
            JsonValue::Number(n) => println!("Float: {}", n),
            JsonValue::String(s) => println!("String: {}", s),
            JsonValue::Array(arr) if arr.is_empty() => println!("Empty array"),
            JsonValue::Array(arr) => println!("Array with {} items", arr.len()),
            JsonValue::Object(obj) if obj.is_empty() => println!("Empty object"),
            JsonValue::Object(obj) => println!("Object with {} fields", obj.len()),
        }
    }
}

fn main() {
    println!("╔════════════════════════════════╗");
    println!("║ JSON PARSER DEMO               ║");
    println!("║ Enums & Pattern Matching       ║");
    println!("╚════════════════════════════════╝");

    demo_basic_values();
    demo_arrays();
    demo_objects();
    demo_nested_structures();
    demo_pattern_matching();

    println!("\n{}", "─".repeat(40));
    println!("\n╔════════════════════════════════╗");
    println!("║ PATTERN MATCHING BENEFITS      ║");
    println!("╠════════════════════════════════╣");
    println!("║ ✓ Exhaustive checking by       ║");
    println!("║   compiler (all cases handled) ║");
    println!("║                                ║");
    println!("║ ✓ Extract values safely        ║");
    println!("║   without unwrap() or panic    ║");
    println!("║                                ║");
    println!("║ ✓ Add conditions with guards   ║");
    println!("║   for specific patterns        ║");
    println!("║                                ║");
    println!("║ ✓ Recursive data structure     ║");
    println!("║   handling naturally           ║");
    println!("╚════════════════════════════════╝");
}
