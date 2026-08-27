// Creational Patterns Examples

// ============= Builder Pattern Example =============
pub struct HttpRequestBuilder {
    method: String,
    url: String,
    headers: std::collections::HashMap<String, String>,
    body: Option<String>,
    timeout_secs: u64,
}

impl HttpRequestBuilder {
    pub fn new(method: &str, url: &str) -> Self {
        Self {
            method: method.to_string(),
            url: url.to_string(),
            headers: std::collections::HashMap::new(),
            body: None,
            timeout_secs: 30,
        }
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            headers: self.headers,
            body: self.body,
            timeout_secs: self.timeout_secs,
        }
    }
}

pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Option<String>,
    pub timeout_secs: u64,
}

impl HttpRequest {
    pub fn display(&self) {
        println!("{} {}", self.method, self.url);
        for (key, val) in &self.headers {
            println!("  {}: {}", key, val);
        }
        if let Some(body) = &self.body {
            println!("Body: {}", body);
        }
    }
}

// ============= Factory Pattern Example =============
pub trait Database {
    fn connect(&self) -> Result<String, String>;
    fn query(&self, sql: &str) -> Vec<String>;
}

pub struct PostgresDb {
    url: String,
}

impl Database for PostgresDb {
    fn connect(&self) -> Result<String, String> {
        Ok(format!("Connected to Postgres: {}", self.url))
    }

    fn query(&self, sql: &str) -> Vec<String> {
        vec![format!("Postgres result for: {}", sql)]
    }
}

pub struct SqliteDb {
    path: String,
}

impl Database for SqliteDb {
    fn connect(&self) -> Result<String, String> {
        Ok(format!("Connected to SQLite: {}", self.path))
    }

    fn query(&self, sql: &str) -> Vec<String> {
        vec![format!("SQLite result for: {}", sql)]
    }
}

pub fn create_database(db_type: &str, connection_info: &str) -> Result<Box<dyn Database>, String> {
    match db_type {
        "postgres" => Ok(Box::new(PostgresDb {
            url: connection_info.to_string(),
        })),
        "sqlite" => Ok(Box::new(SqliteDb {
            path: connection_info.to_string(),
        })),
        _ => Err("Unknown database type".to_string()),
    }
}

// ============= Usage Example =============
pub fn example_creational_patterns() {
    println!("=== Builder Pattern Example ===");
    let request = HttpRequestBuilder::new("POST", "https://api.example.com/users")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"name": "Alice", "age": 30}"#)
        .timeout(60)
        .build();

    request.display();

    println!("\n=== Factory Pattern Example ===");
    let postgres = create_database("postgres", "localhost:5432").unwrap();
    println!("{}", postgres.connect().unwrap());

    let sqlite = create_database("sqlite", "./data.db").unwrap();
    println!("{}", sqlite.connect().unwrap());

    let results = sqlite.query("SELECT * FROM users");
    println!("Results: {:?}", results);
}

fn main() {
    example_creational_patterns();
}
