# GitHub Activity

A Rust CLI tool to fetch and display GitHub user events in a structured and readable format.

## Features

- Fetches GitHub events for a given username.
- Categorizes events by type (e.g., `CreateEvent`, `PushEvent`, `PullRequestEvent`, etc.).
- Displays a summary of events in a clean, human-readable format.

## Installation

1. Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/github-activity.git
   cd github-activity
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the tool with a GitHub username as an argument:

```bash
cargo run -- <username>
```

Example:

```bash
cargo run -- nobel-von-it
```

### Output Example

```
Created 4 repos: nobel-von-it/task-cli, nobel-von-it/weather-app, nobel-von-it/task-tracker
Pushed to repo: nobel-von-it/weather-app
```

## Acknowledgments

- [GitHub API](https://docs.github.com/en/rest) for providing event data.
- [Serde](https://serde.rs/) for JSON serialization/deserialization.
- [ureq](https://github.com/algesten/ureq) for making HTTP requests simple.
- This project was inspired by the [roadmap.sh](https://roadmap.sh/projects/github-user-activity) project.
