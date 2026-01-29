use crate::{
    error::TaskCliError,
    storage::JsonStorage,
    task::{Status, Task},
};
use clap::{Arg, ArgMatches, Command};

pub enum Action {
    Add { text: String },
    Update { id: usize, text: String },
    Delete { id: usize },
    Mark { id: usize, status: String },
    List { status: Option<String> },
    Unknown { command: String },
}

impl Action {
    pub fn execute(&self, storage: &mut JsonStorage) -> Result<(), TaskCliError> {
        match self {
            Self::Add { text } => {
                if storage.gets().iter().any(|task| &task.description == text) {
                    return Err(TaskCliError::AlreadyExists(format!(
                        "Task with text '{}' already exists",
                        text
                    )));
                }
                if text.is_empty() || text.trim().is_empty() {
                    return Err(TaskCliError::IncorrectInput(
                        "Text cannot be empty".to_string(),
                    ));
                }
                storage.add(Task::new(storage.len() + 1, text.to_string()));
                println!("Task added");
            }
            Self::Update { id, text } => {
                if !storage.gets().iter().any(|task| task.id == *id) {
                    return Err(TaskCliError::NotFound(format!(
                        "Task with id {} not found",
                        id
                    )));
                }
                if text.is_empty() || text.trim().is_empty() {
                    return Err(TaskCliError::IncorrectInput(
                        "Text cannot be empty".to_string(),
                    ));
                }
                storage.update_text(*id, text.clone());
                println!("Task with id {} updated", id);
            }
            Self::Delete { id } => {
                if !storage.gets().iter().any(|task| task.id == *id) {
                    return Err(TaskCliError::NotFound(format!(
                        "Task with id {} not found",
                        id
                    )));
                }
                storage.delete(*id);
                println!("Task with id {} deleted", id);
            }
            Self::Mark { id, status } => {
                if !storage.gets().iter().any(|task| task.id == *id) {
                    return Err(TaskCliError::NotFound(format!(
                        "Task with id {} not found",
                        id
                    )));
                }
                let status = Status::from(status);
                storage.update_status(*id, status.clone());
                println!("Task with id {} marked as {}", id, status);
            }
            Self::List { status } => {
                let res: Vec<&Task> = if let Some(status) = status {
                    let status = Status::from(status);
                    storage
                        .gets()
                        .iter()
                        .filter(|task| task.status == status)
                        .collect()
                } else {
                    storage.gets().iter().collect()
                };
                if res.is_empty() {
                    println!("No tasks found");
                } else {
                    for task in res {
                        println!("{}", task);
                    }
                }
            }
            Self::Unknown { command } => {
                return Err(TaskCliError::IncorrectInput(format!(
                    "Unknown command: {}",
                    command
                )))
            }
        }
        Ok(())
    }
}

pub fn command() -> Command {
    Command::new("task-cli")
        .about("CLI for Taskwarrior")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommand(
            Command::new("add")
                .about("Add a task")
                .arg(Arg::new("text").required(true)),
        )
        .subcommand(
            Command::new("update")
                .about("Update a task")
                .arg(Arg::new("id").required(true))
                .arg(Arg::new("text").required(true)),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a task")
                .arg(Arg::new("id").required(true)),
        )
        .subcommand(
            Command::new("mark")
                .about("Mark a task as done or in-progress")
                .arg(Arg::new("id").required(true))
                .arg(Arg::new("status").required(true)),
        )
        .subcommand(
            Command::new("list")
                .about("List tasks")
                .arg(Arg::new("status").required(false)),
        )
}

pub fn parse(args: &ArgMatches) -> Action {
    match args.subcommand() {
        Some(("add", add_args)) => Action::Add {
            text: add_args.get_one::<String>("text").unwrap().clone(),
        },
        Some(("update", update_args)) => Action::Update {
            id: update_args
                .get_one::<String>("id")
                .unwrap()
                .parse()
                .unwrap(),
            text: update_args.get_one::<String>("text").unwrap().clone(),
        },
        Some(("delete", delete_args)) => Action::Delete {
            id: delete_args
                .get_one::<String>("id")
                .unwrap()
                .parse()
                .unwrap(),
        },
        Some(("mark", mark_args)) => Action::Mark {
            id: mark_args.get_one::<String>("id").unwrap().parse().unwrap(),
            status: mark_args.get_one::<String>("status").unwrap().clone(),
        },
        Some(("list", list_args)) => Action::List {
            status: list_args.get_one::<String>("status").cloned(),
        },
        Some((command, _)) => Action::Unknown {
            command: command.to_string(),
        },
        None => unreachable!(),
    }
}
