use async_database_client::{DbConfig, NewUser, Pool, UpdateUser, UserRepository};

async fn setup_test_repo() -> UserRepository {
    let config = DbConfig::new("sqlite::memory:".to_string())
        .with_max_connections(5)
        .with_min_connections(1);

    let pool = Pool::new(&config).await.expect("Failed to create pool");
    pool.run_migrations().await.expect("Failed to run migrations");

    UserRepository::new(pool)
}

#[tokio::test]
async fn test_full_crud_workflow() {
    let repo = setup_test_repo().await;

    // Create
    let new_user = NewUser::new(
        "crud@example.com".to_string(),
        "CRUD User".to_string(),
        25,
    );
    let created = repo.create(&new_user).await.expect("Failed to create user");
    assert_eq!(created.email, "crud@example.com");
    assert_eq!(created.name, "CRUD User");
    assert_eq!(created.age, 25);

    // Read
    let found = repo.find_by_id(created.id).await.expect("Failed to find user");
    assert_eq!(found.id, created.id);
    assert_eq!(found.email, created.email);

    // Update
    let update = UpdateUser {
        email: None,
        name: Some("Updated CRUD User".to_string()),
        age: Some(26),
    };
    let updated = repo.update(created.id, &update).await.expect("Failed to update user");
    assert_eq!(updated.name, "Updated CRUD User");
    assert_eq!(updated.age, 26);

    // Delete
    repo.delete(created.id).await.expect("Failed to delete user");
    let result = repo.find_by_id(created.id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_concurrent_inserts() {
    let repo = setup_test_repo().await;

    // Create multiple users concurrently
    let user1 = NewUser::new("user1@example.com".to_string(), "User 1".to_string(), 20);
    let user2 = NewUser::new("user2@example.com".to_string(), "User 2".to_string(), 30);
    let user3 = NewUser::new("user3@example.com".to_string(), "User 3".to_string(), 40);

    let (r1, r2, r3) = tokio::join!(
        repo.create(&user1),
        repo.create(&user2),
        repo.create(&user3)
    );

    assert!(r1.is_ok());
    assert!(r2.is_ok());
    assert!(r3.is_ok());

    let count = repo.count().await.expect("Failed to count users");
    assert_eq!(count, 3);
}

#[tokio::test]
async fn test_concurrent_reads() {
    let repo = setup_test_repo().await;

    // Create test data
    let user1 = repo.create(&NewUser::new("read1@example.com".to_string(), "Read 1".to_string(), 25)).await.unwrap();
    let user2 = repo.create(&NewUser::new("read2@example.com".to_string(), "Read 2".to_string(), 30)).await.unwrap();

    // Read concurrently using futures::join_all
    let futures = vec![
        repo.find_by_id(user1.id),
        repo.find_by_id(user2.id),
    ];

    let results = futures::future::join_all(futures).await;
    assert_eq!(results.len(), 2);
    assert!(results[0].is_ok());
    assert!(results[1].is_ok());
}

#[tokio::test]
async fn test_find_by_age_filter() {
    let repo = setup_test_repo().await;

    // Create users with different ages
    repo.create(&NewUser::new("young1@example.com".to_string(), "Young 1".to_string(), 20)).await.unwrap();
    repo.create(&NewUser::new("young2@example.com".to_string(), "Young 2".to_string(), 22)).await.unwrap();
    repo.create(&NewUser::new("old1@example.com".to_string(), "Old 1".to_string(), 35)).await.unwrap();
    repo.create(&NewUser::new("old2@example.com".to_string(), "Old 2".to_string(), 40)).await.unwrap();

    let older_users = repo.find_by_age_greater_than(30).await.expect("Failed to find users");
    assert_eq!(older_users.len(), 2);

    for user in &older_users {
        assert!(user.age > 30);
    }
}

#[tokio::test]
async fn test_duplicate_email_error() {
    let repo = setup_test_repo().await;

    let user1 = NewUser::new("duplicate@example.com".to_string(), "User 1".to_string(), 25);
    repo.create(&user1).await.expect("Failed to create first user");

    let user2 = NewUser::new("duplicate@example.com".to_string(), "User 2".to_string(), 30);
    let result = repo.create(&user2).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_user_not_found() {
    let repo = setup_test_repo().await;

    let result = repo.find_by_id(99999).await;
    assert!(result.is_err());

    let result = repo.find_by_email("nonexistent@example.com").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_user() {
    let repo = setup_test_repo().await;

    let result = repo.delete(99999).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_nonexistent_user() {
    let repo = setup_test_repo().await;

    let update = UpdateUser {
        email: None,
        name: Some("New Name".to_string()),
        age: None,
    };

    let result = repo.update(99999, &update).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_partial_update() {
    let repo = setup_test_repo().await;

    let new_user = NewUser::new(
        "partial@example.com".to_string(),
        "Original Name".to_string(),
        25,
    );
    let user = repo.create(&new_user).await.unwrap();

    // Update only name
    let update = UpdateUser {
        email: None,
        name: Some("New Name".to_string()),
        age: None,
    };
    let updated = repo.update(user.id, &update).await.unwrap();

    assert_eq!(updated.name, "New Name");
    assert_eq!(updated.email, "partial@example.com"); // Unchanged
    assert_eq!(updated.age, 25); // Unchanged
}

#[tokio::test]
async fn test_pool_health_check() {
    let config = DbConfig::new("sqlite::memory:".to_string());
    let pool = Pool::new(&config).await.expect("Failed to create pool");

    let is_healthy = pool.health_check().await.expect("Health check failed");
    assert!(is_healthy);
}

#[tokio::test]
async fn test_pool_stats() {
    let config = DbConfig::new("sqlite::memory:".to_string())
        .with_max_connections(3);

    let pool = Pool::new(&config).await.expect("Failed to create pool");
    let stats = pool.stats();

    assert!(stats.size > 0);
    assert!(stats.size <= 3);
}
