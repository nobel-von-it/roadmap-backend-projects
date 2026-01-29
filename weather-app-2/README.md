
# Simple Weather App

This is a simple weather application written in Rust using **Axum**.  
It fetches current weather data from an external API and caches results in **Redis** to reduce API calls.

A minimal HTML + CSS + JavaScript frontend is included and served as static files.

## Features

- get current weather by city name
- external weather API integration (Visual Crossing)
- Redis-based caching with TTL
- simple REST API
- minimal frontend UI
- JSON-based communication

## Tech Stack

### Backend
- Rust
- Axum
- Tokio
- Reqwest (HTTP client)
- Redis
- deadpool-redis (connection pool)
- Serde
- dotenvy
- env_logger

### Frontend
- HTML
- CSS
- Vanilla JavaScript (Fetch API)

## Project Structure

```
.
├── src/
│   ├── main.rs        # Server entry point
│   ├── api.rs           # External weather API logic
│   ├── cache.rs         # Cache abstraction and Redis implementation
│   ├── handlers.rs      # HTTP handlers
│   └── models.rs        # Data models
├── static/
│   ├── index.html
│   ├── style.css
│   └── script.js
├── Cargo.toml
└── README.md

````

## Requirements

- Rust (stable)
- Cargo
- Redis (running locally)
- Visual Crossing API key

## Environment Variables

Before running the project, set the following variable:

```bash
WEATHER_API_KEY=your_api_key_here
````

You can put it into a `.env` file.

## Running the Project

### 1. Start Redis

```bash
redis-server
```

### 2. Run the application

```bash
cargo run
```

The server will start on:

```
http://localhost:3001
```

### 3. Open in browser

```
http://localhost:3001/static/index.html
```

## API Endpoint

### Get current weather

```
POST /api/weather
```

Request body:

```json
{
  "city": "London",
  "timestamp": 1710000000
}
```

Response example:

```json
{
  "temp": 12.3,
  "temp_max": 14.0,
  "temp_min": 8.5,
  "humidity": 65.0,
  "pressure": 1012.0,
  "wind_speed": 4.2
}
```

## Caching Logic

* weather data is cached per city
* cache entries are stored in Redis sorted sets
* cache TTL is aligned to the next full hour
* if cached data is fresh, the external API is not called

## Notes and Limitations

* no authentication
* no error handling on frontend
* only current weather is shown
* city name must be valid
* API limits depend on the external provider

## Purpose

This project is intended for:

* learning Axum and async Rust
* working with external APIs
* practicing Redis caching
* building small full-stack Rust projects

## License

Free to use for learning and personal projects.

