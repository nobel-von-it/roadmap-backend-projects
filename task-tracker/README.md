# tt — simple CLI task manager

`tt` is a minimalistic command-line task (todo) manager written in Rust.  
Tasks are stored in a JSON file and managed via simple CLI commands.

## Features

- add tasks
- update task descriptions
- delete tasks
- change task status
- list tasks with optional status filtering
- JSON-based storage

## Task statuses

The following statuses are supported:

- `todo` — task to be done
- `in-progress` — task is in progress
- `done` — task is completed

## Installation

### Requirements

- Rust (stable)
- Cargo

### Build

```bash
git clone <repo-url>
cd tt
cargo build --release
```

After building, the binary will be located at:

```bash
target/release/tt
```

(or under the name specified in `Cargo.toml`)

## Usage

All data is stored in a `tasks.json` file in the current directory.

### Add a task

```bash
tt add "Write a course project"
```

### Update a task

```bash
tt update 0 "Write a course project about sorting algorithms"
```

### Delete a task

```bash
tt delete 0
```

### Change task status

```bash
tt mark done 1
tt mark in-progress 2
tt mark todo 3
```

### List tasks

List all tasks:

```bash
tt list
```

List tasks with a specific status:

```bash
tt list todo
tt list in-progress
tt list done
```

## Data format

Tasks are stored in `tasks.json` using the following structure:

```json
[
  {
    "id": 0,
    "desc": "Example task",
    "status": "todo",
    "created_at": "2026-01-29T12:00:00+03:00",
    "updated_at": "2026-01-29T12:00:00+03:00"
  }
]
```

## Dependencies

* `clap` — command-line argument parsing
* `miniserde` — JSON serialization/deserialization
* `chrono` — date and time handling
* `anyhow` — error handling
* `env_logger` / `log` — logging

## Notes and limitations

* task `id` values are assigned automatically (based on list length)
* deleted task IDs are not reused
* `tasks.json` is created automatically on first run

## License

Free to use. Fork it, modify it, and make it your own
