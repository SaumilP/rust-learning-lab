# Weather CLI

## Overview

A command-line weather application using external APIs and async/await. Demonstrates network programming and real-world API integration.

## Concepts Learned

- **Async/Await**: Non-blocking network requests
- **HTTP Requests**: reqwest library
- **JSON Parsing**: serde_json for API responses
- **Error Handling**: Network errors and timeouts
- **Configuration**: Config files and environment variables
- **Caching**: Local cache for performance
- **Struct Deserialization**: API response parsing

## Features

1. **Current Weather**: Get live weather data
2. **Multi-City Support**: Check multiple locations
3. **Unit Conversion**: Celsius ↔ Fahrenheit
4. **Forecast**: 5-day forecast viewing
5. **Alerts**: Severe weather notifications
6. **Caching**: Avoid excessive API calls
7. **Configuration**: API keys, default city, units

## Data Structures

```rust
#[derive(Serialize, Deserialize)]
pub struct WeatherData {
    location: String,
    temperature: f64,
    condition: String,
    humidity: u32,
    wind_speed: f64,
    timestamp: DateTime,
}

#[derive(Serialize, Deserialize)]
pub struct Forecast {
    date: String,
    high_temp: f64,
    low_temp: f64,
    condition: String,
    precipitation: f64,
}

pub struct WeatherClient {
    api_key: String,
    base_url: String,
    cache: Cache,
}
```

## Commands

```bash
# Current weather
cargo run -- current "New York"

# Forecast
cargo run -- forecast "San Francisco" --days 7

# Multiple cities
cargo run -- current "London" "Tokyo" "Sydney"

# Unit conversion
cargo run -- current "Paris" --units celsius

# Alerts
cargo run -- alerts "Chicago" --severity high

# Cache management
cargo run -- clear-cache
cargo run -- cache-status
```

## Implementation Details

### Module Organization
```
src/
├── main.rs              # CLI entry point
├── client.rs            # HTTP client
├── models.rs            # Data structures
├── cache.rs             # Caching layer
├── config.rs            # Configuration
└── errors.rs            # Error types
```

### Async Request Example

```rust
pub async fn fetch_weather(location: &str) -> Result<WeatherData, Error> {
    let url = format!("https://api.weather.com/current?city={}", location);
    
    let response = reqwest::Client::new()
        .get(&url)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;
    
    let data: WeatherData = response.json().await?;
    Ok(data)
}
```

### Error Handling

```rust
pub enum WeatherError {
    NetworkError(reqwest::Error),
    ParseError(serde_json::Error),
    ApiError(String),
    NotFound(String),
    Timeout,
}

impl From<reqwest::Error> for WeatherError {
    fn from(err: reqwest::Error) -> Self {
        WeatherError::NetworkError(err)
    }
}
```

## Testing

```bash
# Unit tests
cargo test

# Mock API calls
cargo test -- --test-threads 1

# Integration tests (requires API key)
WEATHER_API_KEY=xxx cargo test --test integration

# Performance: parallel requests
cargo run -- current London Tokyo Sydney Paris
```

## Extension Possibilities

1. **Real-Time Alerts**: Webhook notifications
2. **Weather Maps**: ASCII visualization
3. **Historical Data**: Past weather analysis
4. **Machine Learning**: Weather prediction
5. **Mobile Sync**: Sync with phone apps
6. **Smart Home**: Integration with IoT devices

## Learning Outcomes

After completing this project:
- ✓ Async/await fundamentals
- ✓ HTTP client usage
- ✓ External API integration
- ✓ Error handling for network operations
- ✓ Caching strategies

## Common Patterns Used

### Async Function
```rust
pub async fn fetch_data(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    Ok(response.text().await?)
}
```

### Error Propagation
```rust
pub async fn get_weather(city: &str) -> Result<Weather, Error> {
    let data = fetch_api(city).await?; // Propagate errors
    Ok(parse_weather(data)?)
}
```

### Caching Pattern
```rust
pub fn get_weather_cached(city: &str) -> Result<Weather, Error> {
    if let Some(cached) = self.cache.get(city) {
        return Ok(cached);
    }
    
    let weather = self.fetch_weather(city)?;
    self.cache.put(city, weather.clone());
    Ok(weather)
}
```

## Dependencies

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
serde = { version = "1", features = ["derive"] }
```

## Estimated Development Time

- Beginner: 8-10 hours
- Intermediate: 4-5 hours
- Experienced Rust: 2-3 hours

## Related Modules

- Module 06: Error handling with Result
- Module 07 (Advanced): Async/await patterns
- Module 08: Facade pattern for API

