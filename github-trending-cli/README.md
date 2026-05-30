# 🚀 GitHub Trending CLI

A modern, user-friendly Command-Line Interface (CLI) application that interacts with the GitHub REST API to retrieve, filter, and display trending repositories. 

This project is designed as an excellent practical exercise to master CLI application design, third-party API integration, robust error handling, configuration management, and terminal output formatting.

---

## 🎯 Project Goals

- **API Integration**: Learn how to interact with the GitHub REST API, handle rate limiting, and parse JSON payloads.
- **CLI Design**: Design clean terminal interfaces, handle command-line arguments/flags, and print structured, formatted output.
- **Robust Error Handling**: Handle API errors, network issues, and invalid user input gracefully.
- **Clean Code Architecture**: Build a maintainable and modular backend structure in the language of your choice.

---

## 🛠️ Features

- [x] **Time Range Filtering**: Support filtering repositories created/active within a specific duration: `day`, `week`, `month`, or `year`.
- [x] **Custom Limits**: Control the number of results returned (default is 10, fully configurable).
- [x] **Smart Sorting**: Repositories are sorted dynamically by star count.
- [x] **Rich Terminal Output**: Present details (name, description, stars, primary language, and link) in an elegant, readable format.
- [x] **No Auth Required**: Works out of the box using public GitHub endpoints (rate limits apply).

---

## ⚙️ Command-Line Arguments & Flags

The CLI tool should support the following flags:

| Flag | Long Flag | Description | Default | Allowed Values |
| :--- | :--- | :--- | :--- | :--- |
| `-d` | `--duration` | Specifies the time range to search within | `week` | `day`, `week`, `month`, `year` |
| `-l` | `--limit` | Specifies the maximum number of repositories to display | `10` | Any positive integer (e.g., `1-100`) |
| `-h` | `--help` | Displays usage instructions and help info | - | - |

---

## 💡 How it Works Under the Hood

Since the official GitHub REST API does not have a dedicated "trending" endpoint, the best way to fetch trending repositories is by using the **Search Repositories API** with date queries:

### 1. Endpoint
```http
GET https://api.github.com/search/repositories
```

### 2. Query Parameters
To find trending repositories, query for those created after a specific date, sorted by stars:
- **`q`**: `created:>{YYYY-MM-DD}` (calculated dynamically based on `--duration`)
- **`sort`**: `stars`
- **`order`**: `desc`
- **`per_page`**: specified by `--limit`

> [!TIP]
> **Date Calculation Examples (assuming today is May 30, 2026):**
> - `--duration day`: `q=created:>2026-05-29`
> - `--duration week`: `q=created:>2026-05-23`
> - `--duration month`: `q=created:>2026-04-30`
> - `--duration year`: `q=created:>2025-05-30`

---

## 🖥️ Example Usage & Output

### Commands

```bash
# Get top 10 trending repositories of the week (default)
trending-repos

# Get top 20 trending repositories of the last month
trending-repos --duration month --limit 20

# Get top 5 trending repositories of the last day using shorthand flags
trending-repos -d day -l 5
```

### Expected Output Format

Your application should print a clean, visually appealing terminal table or list. For example:

```text
📊 GitHub Trending Repositories (Last month, Top 5)
=================================================================================
#   Repository                           ⭐ Stars   🔤 Language   🔗 URL
---------------------------------------------------------------------------------
1   awesome-selfhosted/awesome-self...  185,240    Markdown      github.com/awesome...
2   cool-org/next-gen-db                12,450     Go            github.com/cool-or...
3   dev-tools/fast-api-template         4,321      Python        github.com/dev-too...
4   ui-kit/glassmorphism-components     1,890      TypeScript    github.com/ui-kit/...
5   ai-labs/simple-agent                850        JavaScript    github.com/ai-labs...
=================================================================================
```

---

## ⚠️ Error Handling & Edge Cases

To ensure a premium user experience, the CLI must handle the following cases:

1. **GitHub API Rate Limiting**: GitHub limits unauthenticated requests to 60 requests per hour. If exceeded, show a helpful message:
   > 🚫 *API rate limit exceeded. Please try again later or provide a GitHub token.*
2. **Network Offline**: Gracefully report connection issues:
   > 🔌 *Error: Unable to connect to GitHub. Please check your internet connection.*
3. **Invalid Arguments**: If the user provides `--duration yearly` or `--limit -5`, show usage instructions:
   > ❌ *Error: Invalid duration "yearly". Allowed values: day, week, month, year.*

---

## 🚀 Getting Started (Installation template)

> [!NOTE]
> *Replace this section with specific setup instructions for your chosen language (Go, Node.js, Python, Rust, etc.).*

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/github-trending-cli.git
   cd github-trending-cli
   ```

2. Run local build / execute:
   ```bash
   # Example for Node.js
   npm install
   npm link
   trending-repos --help
   ```
