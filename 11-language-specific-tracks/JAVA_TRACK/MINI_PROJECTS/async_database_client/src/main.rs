use async_database_client::{DbConfig, NewUser, Pool, UpdateUser, UserRepository};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Async Database Client Demo\n");

    // Create configuration from environment
    let config = DbConfig::from_env()?;
    println!("✓ Configuration loaded");
    println!("  Database: {}", config.database_url);
    println!("  Max connections: {}", config.max_connections);

    // Create connection pool (like Java's HikariCP)
    let pool = Pool::new(&config).await?;
    println!("\n✓ Connection pool created");

    // Check pool health
    if pool.health_check().await? {
        println!("✓ Database connection is healthy");
    }

    // Run migrations to create tables
    pool.run_migrations().await?;
    println!("✓ Migrations completed");

    // Create repository (like Spring Data Repository)
    let repo = UserRepository::new(pool.clone());
    println!("\n--- CRUD Operations Demo ---\n");

    // CREATE: Insert new users
    println!("1. Creating users...");
    let alice = NewUser::new(
        "alice@example.com".to_string(),
        "Alice Smith".to_string(),
        30,
    );
    let bob = NewUser::new(
        "bob@example.com".to_string(),
        "Bob Johnson".to_string(),
        25,
    );
    let carol = NewUser::new(
        "carol@example.com".to_string(),
        "Carol Williams".to_string(),
        35,
    );

    // Demonstrate concurrent inserts using tokio::join!
    // Java equivalent: CompletableFuture.allOf(future1, future2, future3).join()
    let (user1, user2, user3) = tokio::join!(
        repo.create(&alice),
        repo.create(&bob),
        repo.create(&carol)
    );

    let user1 = user1?;
    let user2 = user2?;
    let user3 = user3?;

    println!("   ✓ Created user: {} (ID: {})", user1.name, user1.id);
    println!("   ✓ Created user: {} (ID: {})", user2.name, user2.id);
    println!("   ✓ Created user: {} (ID: {})", user3.name, user3.id);

    // READ: Find users
    println!("\n2. Finding users...");

    let found_user = repo.find_by_email("alice@example.com").await?;
    println!("   ✓ Found by email: {:?}", found_user);

    let found_user = repo.find_by_id(user2.id).await?;
    println!("   ✓ Found by ID: {:?}", found_user);

    let older_users = repo.find_by_age_greater_than(28).await?;
    println!("   ✓ Users older than 28: {} found", older_users.len());
    for user in &older_users {
        println!("     - {} (age: {})", user.name, user.age);
    }

    // UPDATE: Modify a user
    println!("\n3. Updating user...");
    let update = UpdateUser {
        email: None,
        name: Some("Alice Johnson".to_string()),
        age: Some(31),
    };
    let updated_user = repo.update(user1.id, &update).await?;
    println!("   ✓ Updated user: {:?}", updated_user);

    // DELETE: Remove a user
    println!("\n4. Deleting user...");
    repo.delete(user2.id).await?;
    println!("   ✓ Deleted user with ID: {}", user2.id);

    // LIST ALL: Show remaining users
    println!("\n5. Listing all users...");
    let all_users = repo.find_all().await?;
    println!("   ✓ Total users: {}", all_users.len());
    for user in &all_users {
        println!("     - {} <{}> (age: {})", user.name, user.email, user.age);
    }

    // COUNT: Get total count
    let count = repo.count().await?;
    println!("\n6. Total users in database: {}", count);

    // Pool statistics
    println!("\n--- Pool Statistics ---");
    let stats = pool.stats();
    println!("  Total connections: {}", stats.size);
    println!("  Idle connections: {}", stats.idle_connections);

    println!("\n--- Demonstrating Concurrent Queries ---");

    // Spawn multiple async queries concurrently
    // Java equivalent:
    // List<CompletableFuture<User>> futures = ids.stream()
    //     .map(id -> CompletableFuture.supplyAsync(() -> repo.findById(id)))
    //     .collect(Collectors.toList());
    let ids: Vec<i64> = all_users.iter().map(|u| u.id).collect();

    let futures: Vec<_> = ids.iter()
        .map(|&id| repo.find_by_id(id))
        .collect();

    let results = futures::future::join_all(futures).await;
    let successful = results.iter().filter(|r| r.is_ok()).count();
    println!("  ✓ Executed {} concurrent queries, {} successful", results.len(), successful);

    // Cleanup
    pool.close().await;
    println!("\n✓ Connection pool closed");
    println!("\n🎉 Demo completed successfully!");

    Ok(())
}
