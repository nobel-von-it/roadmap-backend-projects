# TMDB CLI Tool

A minimalist command-line interface (CLI) written in Go to fetch and display movies from The Movie Database (TMDB) directly in your terminal.

## Features

- **Four categories**: Fetch Popular, Now Playing, Top Rated, and Upcoming movies.
- **Localization**: Supports both English (`en-US`) and Russian (`ru-RU`) metadata.
- **Clean output**: Beautiful terminal lists containing titles, release dates, and user ratings.

## Installation

Ensure you have Go installed (version 1.20+), then build the application:

```bash
# Build the binary
go build -o tmdb-app
```

## Setup

The tool authenticates with TMDB using a **Read Access Token (JWT)**. Save it to your `.env` file or export it as an environment variable:

```bash
export TMDB_JWT_KEY="your_tmdb_read_access_token"
```

## Usage

```bash
# Fetch now playing movies (default)
./tmdb-app

# Fetch top-rated movies
./tmdb-app -t top

# Fetch upcoming movies localized in Russian
./tmdb-app -t upcoming -l ru-RU
```

### Options

| Flag | Shorthand | Description | Supported Values | Default |
| :--- | :--- | :--- | :--- | :--- |
| `--type` | `-t` | Category of movies | `playing`, `popular`, `top`, `upcoming` | `playing` |
| `--language` | `-l` | Localization code | `en-US`, `ru-RU` | `en-US` |