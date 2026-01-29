# Simple Blog Service

This is a small blog service written in Rust using **Axum** and **SQLite**.  
It provides a REST API for managing blog posts and a minimal frontend served as static files.

The project is intended for learning and experimentation, not for production use.

## Features

- create blog posts
- edit existing posts
- delete posts
- list all posts
- view a single post
- SQLite database for storage
- simple HTML + CSS + JavaScript frontend
- JSON-based REST API

## Tech Stack

### Backend
- Rust
- Axum (HTTP server & routing)
- Tokio (async runtime)
- SQLite
- deadpool-sqlite (connection pool)
- Serde (JSON serialization)
- Chrono (date and time)
- Tower HTTP (static files)

### Frontend
- HTML
- CSS
- Vanilla JavaScript (Fetch API)

## Requirements

- Rust (stable)
- Cargo

## Running the Project

### 1. Build and run

```bash
cargo run
```

The server will start on:

```
http://localhost:3000
```

### 2. Open in browser

Open the frontend in your browser:

```
http://localhost:3000/static/index.html
```

## API Endpoints

### Create a post

```
POST /posts
```

Request body:

```json
{
  "title": "My first post",
  "content": "Hello world",
  "category": "general",
  "tags": ["rust", "axum"]
}
```

Response:

* `201 Created` with the created post
* `400 Bad Request` if validation fails

### Get all posts

```
GET /posts
```

### Get a single post

```
GET /posts/{id}
```

### Update a post

```
PUT /posts/{id}
```

Request body is the same as for creating a post.

### Delete a post

```
DELETE /posts/{id}
```

Response:

* `204 No Content` on success

## Data Validation

* all fields must be non-empty
* only ASCII characters are allowed
* tags must be non-empty strings

Invalid data results in a `400 Bad Request`.

## Notes and Limitations

* authentication is not implemented
* pagination is not implemented
* database schema is very simple
* `created_at` is currently updated on edit
* frontend is intentionally minimal

## Purpose

This project is meant to:

* practice Rust backend development
* learn Axum and async programming
* understand basic CRUD APIs
* combine backend and frontend in a single project

## License

Free to use for learning and personal projects.
