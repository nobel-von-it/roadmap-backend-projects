# 🚀 GitHub Trending CLI

A Command-Line Interface (CLI) application built in **Go** that interacts with the GitHub REST API to retrieve, filter, and display trending repositories.

This project is built as part of the [roadmap.sh/backend](https://roadmap.sh/backend) project roadmap to practice CLI application design, API integration, and clean Golang architecture.

---

## 🎯 Project Goals

- **Go & CLI Design**: Parse command-line flags, implement custom validations, and build robust structures using Go's standard libraries (`flag`, `text/tabwriter`).
- **API Integration**: Query the GitHub Search API dynamically, process response JSONs, and calculate rolling dates.
- **Modern Terminal Output**: Design a beautifully formatted terminal output with colored columns and dynamic table alignment.

---

## 🛠️ Features

- **Time Range Filtering**: Support filtering repositories created/active within a specific duration: `day`, `week`, `month`, or `year` (Defaults to `week`).
- **Custom Limits**: Control the number of repositories to display (Defaults to `10`).
- **Dynamic Star Sorting**: Fetches repositories sorted by star count in descending order.
- **Sleek Column Layout**: Displays `REPOSITORY`, `STARS`, `LANGUAGE`, and `URL` aligned dynamically with `text/tabwriter`.
- **Colored Terminals**: Employs ANSI colors (Bold, Yellow, Cyan, Blue) to enhance readability.

---

## ⚙️ Command-Line Arguments & Flags

The application parses arguments using Go's standard `flag` package, supporting both standard and shorthand flags:

| Flag | Long Flag | Description | Default | Allowed Values |
| :--- | :--- | :--- | :--- | :--- |
| `-d` | `--duration` | Time range for trending repositories | `week` | `day`, `week`, `month`, `year` |
| `-l` | `--limit` | Number of repositories to display | `10` | Any positive integer |

### Custom Help Output

If you pass `--help` or invalid arguments, the CLI displays a clean, custom usage output:

```text
Usage: trending-repos [flags]
Flags:
	 d 	 Specifies the time range (shorthand)
	 duration 	 Specifies the time range to query (day, week, month, year)
	 l 	 Specifies the number of repositories (shorthand)
	 limit 	 Specifies the number of repositories to display
```

---

## 💡 How it Works Under the Hood

The application dynamically calculates the date threshold relative to `time.Now()` using the requested `--duration`:

- `day`: `created:>YYYY-MM-DD` (1 day ago)
- `week`: `created:>YYYY-MM-DD` (7 days ago)
- `month`: `created:>YYYY-MM-DD` (1 month ago)
- `year`: `created:>YYYY-MM-DD` (1 year ago)

It constructs a single request to the **GitHub Search Repositories API**:
```http
GET https://api.github.com/search/repositories?q=created:>{DATE}&sort=stars&order=desc&per_page={LIMIT}&page=1
```

---

## 🖥️ Example Usage & Output

### Running the App

```bash
# Display 10 trending repositories of the last week (default)
./trending-repos

# Display 20 trending repositories of the last month
./trending-repos --duration month --limit 20

# Display 5 trending repositories of the last day using shorthand flags
./trending-repos -d day -l 5
```

### Colored Tabular Output

The application renders a perfectly aligned table where each column is colored for maximum visual style:

```text
REPOSITORY                          STARS    LANGUAGE     URL
openclaw/openclaw                   375564   TypeScript   https://github.com/openclaw/openclaw
affaan-m/ECC                        198826   JavaScript   https://github.com/affaan-m/ECC
ultraworkers/claw-code              192886   Rust         https://github.com/ultraworkers/claw-code
NousResearch/hermes-agent           173169   Python       https://github.com/NousResearch/hermes-agent
multica-ai/andrej-karpathy-skills   162025                https://github.com/multica-ai/andrej-karpathy-skills
```

---

## 🛠️ Build and Setup

### Prerequisites

- [Go](https://go.dev/) (version 1.26.3 or higher)
- Optional: [Just](https://github.com/casey/just) command runner

### Installation & Run

1. Clone the repository:
   ```bash
   git clone https://github.com/nimirus/roadmap-backend-projects.git
   cd roadmap-backend-projects/github-trending-cli
   ```

2. Build the executable manually:
   ```bash
   go build -o trending-repos
   ```

3. Or build and run using `just`:
   ```bash
   just run
   ```

---

## ⚠️ Error Handling

- **Invalid Limits**: Supplying a non-positive integer for `-l`/`--limit` will yield:
  > `limit must be a positive integer`
- **Invalid Durations**: Supplying an unknown value for `-d`/`--duration` will trigger a validation error:
  > `invalid duration "yearly" (must be: day, week, month, year)`
