use data_pipeline::{UserPipeline, UserStats};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Data Processing Pipeline Demo\n");

    // Create sample data directory if it doesn't exist
    std::fs::create_dir_all("data")?;

    // Create sample CSV file
    create_sample_data()?;
    println!("✓ Created sample data file\n");

    // Load users from CSV
    let pipeline = UserPipeline::from_csv("data/users.csv")?;
    println!("✓ Loaded {} users from CSV\n", pipeline.count());

    // Example 1: Filter adults
    println!("--- Example 1: Filter adults ---");
    let adults = pipeline.clone().filter(|u| u.is_adult());
    println!("Found {} adults:", adults.count());
    for user in adults.users().iter().take(3) {
        println!("  {}", user);
    }

    // Example 2: Filter Gmail users
    println!("\n--- Example 2: Gmail users ---");
    let gmail_users = UserPipeline::from_csv("data/users.csv")?
        .filter(|u| u.is_gmail_user());
    println!("Found {} Gmail users:", gmail_users.count());
    for user in gmail_users.users() {
        println!("  {}", user);
    }

    // Example 3: Chain transformations
    println!("\n--- Example 3: Chain transformations ---");
    let transformed = UserPipeline::from_csv("data/users.csv")?
        .filter(|u| u.age >= 25)
        .filter(|u| u.is_gmail_user())
        .map(|mut u| {
            u.name = u.name.to_uppercase();
            u
        })
        .sort_by(|u| u.age)
        .take(3);

    println!("Top 3 Gmail users (age >= 25):");
    for user in transformed.users() {
        println!("  {}", user);
    }

    // Example 4: Group by city
    println!("\n--- Example 4: Group by city ---");
    let pipeline = UserPipeline::from_csv("data/users.csv")?;
    let by_city: HashMap<String, UserStats> = pipeline.group_by(|u| u.city.clone());

    println!("Statistics by city:");
    for (city, stats) in &by_city {
        println!("  {}:", city);
        println!("    Count: {}", stats.count);
        println!("    Avg age: {:.1}", stats.avg_age);
        println!("    Age range: {}-{}", stats.min_age, stats.max_age);
    }

    // Example 5: Parallel processing
    println!("\n--- Example 5: Parallel processing ---");
    let start = std::time::Instant::now();

    let result = UserPipeline::from_csv("data/users.csv")?
        .parallel_map(|mut u| {
            // Simulate expensive computation
            u.name = u.name.to_uppercase();
            u
        });

    let elapsed = start.elapsed();
    println!(
        "Processed {} users in parallel: {:?}",
        result.count(),
        elapsed
    );

    // Example 6: Write to CSV
    println!("\n--- Example 6: Write output ---");
    UserPipeline::from_csv("data/users.csv")?
        .filter(|u| u.is_adult())
        .to_csv("data/output.csv")?;
    println!("✓ Wrote filtered users to data/output.csv");

    // Example 7: Write to JSON
    UserPipeline::from_csv("data/users.csv")?
        .filter(|u| u.is_gmail_user())
        .take(5)
        .to_json("data/output.json")?;
    println!("✓ Wrote Gmail users to data/output.json");

    println!("\n🎉 Demo completed successfully!");

    Ok(())
}

fn create_sample_data() -> Result<(), Box<dyn std::error::Error>> {
    use csv::Writer;

    let mut writer = Writer::from_path("data/users.csv")?;

    // Write header
    writer.write_record(&["name", "age", "email", "city"])?;

    // Write sample data
    let users = vec![
        ("Alice Smith", 30, "alice@gmail.com", "NYC"),
        ("Bob Johnson", 25, "bob@yahoo.com", "LA"),
        ("Carol Williams", 35, "carol@gmail.com", "NYC"),
        ("Dave Brown", 28, "dave@gmail.com", "SF"),
        ("Eve Davis", 22, "eve@hotmail.com", "LA"),
        ("Frank Miller", 40, "frank@gmail.com", "NYC"),
        ("Grace Wilson", 33, "grace@yahoo.com", "SF"),
        ("Henry Moore", 27, "henry@gmail.com", "LA"),
        ("Ivy Taylor", 31, "ivy@gmail.com", "NYC"),
        ("Jack Anderson", 29, "jack@hotmail.com", "SF"),
    ];

    for (name, age, email, city) in users {
        writer.write_record(&[name, &age.to_string(), email, city])?;
    }

    writer.flush()?;
    Ok(())
}
