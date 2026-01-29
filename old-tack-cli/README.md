# Task CLI

Task CLI is a simple command-line interface (CLI) tool for managing tasks. It allows you to add, update, delete, mark, and list tasks with ease. The tasks are stored in a JSON file, making it easy to manage and persist your tasks across sessions.

## Features

- **Add a task**: Add a new task with a description.
- **Update a task**: Update the description of an existing task.
- **Delete a task**: Remove a task by its ID.
- **Mark a task**: Mark a task as "done" or "in-progress".
- **List tasks**: List all tasks, optionally filtered by status.

## Installation

To use Task CLI, you need to have Rust installed on your system. If you don't have Rust installed, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/).

Once Rust is installed, you can clone this repository and build the project:

```bash
git clone https://github.com/nobel-von-it/task-cli.git
cd task-cli
cargo build --release
```

The binary will be located in `target/release/task-cli`.

## Usage

### Add a Task

To add a new task, use the `add` command followed by the task description:

```bash
task-cli add "Buy groceries"
```

### Update a Task

To update an existing task, use the `update` command followed by the task ID and the new description:

```bash
task-cli update 1 "Buy groceries and cook dinner"
```

### Delete a Task

To delete a task, use the `delete` command followed by the task ID:

```bash
task-cli delete 1
```

### Mark a Task

To mark a task as "done" or "in-progress", use the `mark` command followed by the task ID and the status:

```bash
task-cli mark 1 done
```

### List Tasks

To list all tasks, use the `list` command. You can optionally filter tasks by status:

```bash
task-cli list
task-cli list done
```

## Commands Overview

| Command  | Description                          | Example Usage                          |
|----------|--------------------------------------|----------------------------------------|
| `add`    | Add a new task                       | `task-cli add "Buy groceries"`         |
| `update` | Update an existing task              | `task-cli update 1 "Buy groceries"`    |
| `delete` | Delete a task by ID                  | `task-cli delete 1`                    |
| `mark`   | Mark a task as done or in-progress   | `task-cli mark 1 done`                 |
| `list`   | List all tasks (optionally filtered) | `task-cli list` or `task-cli list done`|

## Error Handling

The CLI handles errors gracefully and will provide meaningful error messages if something goes wrong, such as attempting to update or delete a non-existent task.

## Contributing

Contributions are welcome! If you have any suggestions, bug reports, or feature requests, please open an issue or submit a pull request.

## Acknowledgments

- Thanks to the Rust community for providing excellent libraries and tools.
- Special thanks to the creators of `clap` for making command-line argument parsing easy and intuitive.
- This project was inspired by the [roadmap.sh](https://roadmap.sh/projects/task-tracker) project.

---

Enjoy managing your tasks with Task CLI!
