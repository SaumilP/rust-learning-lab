use data_pipeline::{User, UserPipeline, UserStats};
use std::collections::HashMap;

fn create_test_users() -> Vec<User> {
    vec![
        User {
            name: "Alice".to_string(),
            age: 30,
            email: "alice@gmail.com".to_string(),
            city: "NYC".to_string(),
        },
        User {
            name: "Bob".to_string(),
            age: 25,
            email: "bob@yahoo.com".to_string(),
            city: "LA".to_string(),
        },
        User {
            name: "Carol".to_string(),
            age: 35,
            email: "carol@gmail.com".to_string(),
            city: "NYC".to_string(),
        },
        User {
            name: "Dave".to_string(),
            age: 17,
            email: "dave@gmail.com".to_string(),
            city: "SF".to_string(),
        },
    ]
}

#[test]
fn test_filter_adults() {
    let pipeline = UserPipeline::from_users(create_test_users());
    let adults = pipeline.filter(|u| u.is_adult());

    assert_eq!(adults.count(), 3);
}

#[test]
fn test_filter_gmail_users() {
    let pipeline = UserPipeline::from_users(create_test_users());
    let gmail_users = pipeline.filter(|u| u.is_gmail_user());

    assert_eq!(gmail_users.count(), 3);
}

#[test]
fn test_chain_filters() {
    let pipeline = UserPipeline::from_users(create_test_users());
    let result = pipeline
        .filter(|u| u.is_adult())
        .filter(|u| u.is_gmail_user());

    assert_eq!(result.count(), 2); // Alice and Carol
}

#[test]
fn test_map_transformation() {
    let pipeline = UserPipeline::from_users(create_test_users());
    let transformed = pipeline.map(|mut u| {
        u.name = u.name.to_uppercase();
        u
    });

    assert_eq!(transformed.users()[0].name, "ALICE");
    assert_eq!(transformed.users()[1].name, "BOB");
}

#[test]
fn test_chain_filter_and_map() {
    let pipeline = UserPipeline::from_users(create_test_users());
    let result = pipeline
        .filter(|u| u.is_gmail_user())
        .map(|mut u| {
            u.name = u.name.to_uppercase();
            u
        });

    assert_eq!(result.count(), 3);
    assert_eq!(result.users()[0].name, "ALICE");
}

#[test]
fn test_take() {
    let pipeline = UserPipeline::from_users(create_test_users());
    let limited = pipeline.take(2);

    assert_eq!(limited.count(), 2);
}

#[test]
fn test_sort_by_age() {
    let pipeline = UserPipeline::from_users(create_test_users());
    let sorted = pipeline.sort_by(|u| u.age);

    assert_eq!(sorted.users()[0].age, 17); // Dave
    assert_eq!(sorted.users()[1].age, 25); // Bob
    assert_eq!(sorted.users()[2].age, 30); // Alice
    assert_eq!(sorted.users()[3].age, 35); // Carol
}

#[test]
fn test_group_by_city() {
    let pipeline = UserPipeline::from_users(create_test_users());
    let groups: HashMap<String, UserStats> = pipeline.group_by(|u| u.city.clone());

    assert_eq!(groups.len(), 3); // NYC, LA, SF
    assert!(groups.contains_key("NYC"));
    assert_eq!(groups["NYC"].count, 2); // Alice and Carol
}

#[test]
fn test_group_by_avg_age() {
    let pipeline = UserPipeline::from_users(create_test_users());
    let groups: HashMap<String, UserStats> = pipeline.group_by(|u| u.city.clone());

    let nyc_stats = &groups["NYC"];
    assert_eq!(nyc_stats.avg_age, 32.5); // (30 + 35) / 2
    assert_eq!(nyc_stats.min_age, 30);
    assert_eq!(nyc_stats.max_age, 35);
}

#[test]
fn test_parallel_map() {
    let pipeline = UserPipeline::from_users(create_test_users());
    let result = pipeline.parallel_map(|mut u| {
        u.name = u.name.to_uppercase();
        u
    });

    assert_eq!(result.count(), 4);
    assert!(result.users().iter().all(|u| u.name.chars().all(|c| c.is_uppercase() || c.is_whitespace())));
}

#[test]
fn test_csv_round_trip() {
    let test_file = "test_users.csv";
    let users = create_test_users();
    let pipeline = UserPipeline::from_users(users.clone());

    // Write to CSV
    pipeline.to_csv(test_file).unwrap();

    // Read back from CSV
    let loaded = UserPipeline::from_csv(test_file).unwrap();

    assert_eq!(loaded.count(), users.len());

    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_json_output() {
    let test_file = "test_users.json";
    let pipeline = UserPipeline::from_users(create_test_users());

    // Write to JSON
    pipeline.to_json(test_file).unwrap();

    // Verify file exists and is valid JSON
    let content = std::fs::read_to_string(test_file).unwrap();
    let parsed: Vec<User> = serde_json::from_str(&content).unwrap();

    assert_eq!(parsed.len(), 4);

    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_complex_pipeline() {
    // Test a realistic pipeline with multiple operations
    let pipeline = UserPipeline::from_users(create_test_users());

    let result = pipeline
        .filter(|u| u.age >= 25)           // Adults 25+
        .filter(|u| u.is_gmail_user())     // Gmail users only
        .map(|mut u| {                     // Uppercase names
            u.name = u.name.to_uppercase();
            u
        })
        .sort_by(|u| u.age)                // Sort by age
        .take(2);                          // Take first 2

    assert_eq!(result.count(), 2);
    assert_eq!(result.users()[0].name, "ALICE"); // age 30
    assert_eq!(result.users()[1].name, "CAROL"); // age 35
}

#[test]
fn test_user_validation() {
    let valid = User::new(
        "Test".to_string(),
        25,
        "test@example.com".to_string(),
        "NYC".to_string(),
    );
    assert!(valid.is_ok());

    let invalid_email = User::new(
        "Test".to_string(),
        25,
        "invalid".to_string(),
        "NYC".to_string(),
    );
    assert!(invalid_email.is_err());

    let invalid_age = User::new(
        "Test".to_string(),
        200,
        "test@example.com".to_string(),
        "NYC".to_string(),
    );
    assert!(invalid_age.is_err());
}

#[test]
fn test_user_stats_empty() {
    let stats = UserStats::from_users(&[]);
    assert!(stats.is_none());
}
