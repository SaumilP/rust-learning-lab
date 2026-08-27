use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u64,
}

#[derive(Debug, Deserialize)]
struct GithubRepo {
    name: String,
    description: Option<String>,
    stargazers_count: u64,
    language: Option<String>,
}

async fn get_request_example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 1. GET Request Example ===");

    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = client.get(url).send().await?;

    println!("Status: {}", response.status());

    let post: Post = response.json().await?;
    println!("Post: {:?}", post);

    Ok(())
}

async fn post_request_example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 2. POST Request Example ===");

    let new_post = Post {
        id: None,
        title: "Rust HTTP Client".to_string(),
        body: "Learning reqwest with Rust".to_string(),
        user_id: 1,
    };

    let url = "https://jsonplaceholder.typicode.com/posts";
    let response = client
        .post(url)
        .json(&new_post)
        .send()
        .await?;

    println!("Status: {}", response.status());

    let created_post: Post = response.json().await?;
    println!("Created post: {:?}", created_post);

    Ok(())
}

async fn put_request_example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 3. PUT Request Example ===");

    let updated_post = Post {
        id: Some(1),
        title: "Updated Title".to_string(),
        body: "Updated body content".to_string(),
        user_id: 1,
    };

    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = client
        .put(url)
        .json(&updated_post)
        .send()
        .await?;

    println!("Status: {}", response.status());

    let result: Post = response.json().await?;
    println!("Updated post: {:?}", result);

    Ok(())
}

async fn delete_request_example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 4. DELETE Request Example ===");

    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = client.delete(url).send().await?;

    println!("Status: {}", response.status());
    println!("Deleted successfully");

    Ok(())
}

async fn custom_headers_example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 5. Custom Headers Example ===");

    let url = "https://api.github.com/repos/rust-lang/rust";
    let response = client
        .get(url)
        .header(header::USER_AGENT, "rust-http-client/1.0")
        .header(header::ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await?;

    println!("Status: {}", response.status());

    let repo: GithubRepo = response.json().await?;
    println!("Repo: {}", repo.name);
    println!("Stars: {}", repo.stargazers_count);
    println!("Language: {:?}", repo.language);

    Ok(())
}

async fn concurrent_requests_example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 6. Concurrent Requests Example ===");

    let urls = vec![
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://jsonplaceholder.typicode.com/posts/2",
        "https://jsonplaceholder.typicode.com/posts/3",
    ];

    let futures: Vec<_> = urls
        .iter()
        .map(|url| client.get(*url).send())
        .collect();

    let results = futures::future::join_all(futures).await;

    for (idx, result) in results.iter().enumerate() {
        match result {
            Ok(response) => println!("Request {} status: {}", idx + 1, response.status()),
            Err(e) => eprintln!("Request {} failed: {}", idx + 1, e),
        }
    }

    Ok(())
}

async fn error_handling_example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 7. Error Handling Example ===");

    let url = "https://jsonplaceholder.typicode.com/invalid";
    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!("Success: {}", response.status());
            } else {
                println!("HTTP Error: {}", response.status());
            }
        }
        Err(e) => {
            if e.is_timeout() {
                eprintln!("Request timeout");
            } else if e.is_connect() {
                eprintln!("Connection error");
            } else {
                eprintln!("Request failed: {}", e);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 HTTP Client Examples\n");

    // Create a client with connection pooling and timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(10)
        .build()?;

    // Run examples
    get_request_example(&client).await?;
    post_request_example(&client).await?;
    put_request_example(&client).await?;
    delete_request_example(&client).await?;
    custom_headers_example(&client).await?;
    concurrent_requests_example(&client).await?;
    error_handling_example(&client).await?;

    println!("\n✅ All HTTP client examples completed!");

    Ok(())
}
