use clap::{Arg, ArgMatches, Command};

use crate::{error::ExpenseError, storage::JsonStorage, types::Expense};

fn month_number_to_name(month: u8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => unreachable!(),
    }
}

pub enum Action {
    Add {
        amount: f64,
        description: Option<String>,
    },
    Update {
        id: usize,
        amount: Option<f64>,
        description: Option<String>,
    },
    Delete {
        id: usize,
    },
    Summary {
        month: Option<u32>,
    },
    List,
}

impl Action {
    pub fn execute(&self, storage: &mut JsonStorage) -> Result<(), ExpenseError> {
        match self {
            Self::Add {
                amount,
                description,
            } => {
                if let Some(description) = description {
                    if description.trim().is_empty() {
                        return Err(ExpenseError::EmptyDescription);
                    }
                }
                if amount <= &0.0 {
                    return Err(ExpenseError::InvalidAmount);
                }
                let expense = Expense::new(*amount, description.clone());
                storage.add(expense);
            }
            Self::Update {
                id,
                amount,
                description,
            } => {
                if !storage.gets().iter().any(|e| e.id == *id) {
                    return Err(ExpenseError::ExpenseNotFound(*id));
                }
                if amount.is_some() && description.is_some() {
                    let amount = amount.unwrap();
                    let description = description.clone().unwrap();

                    if amount <= 0.0 {
                        return Err(ExpenseError::InvalidAmount);
                    }
                    if description.trim().is_empty() {
                        return Err(ExpenseError::EmptyDescription);
                    }

                    storage.update_amount(*id, amount);
                    storage.update_description(*id, description);

                    println!("Expense with id {} updated with amount and description", id);
                } else if let Some(amount) = amount {
                    if amount <= &0.0 {
                        return Err(ExpenseError::InvalidAmount);
                    }

                    storage.update_amount(*id, *amount);
                    println!("Expense with id {} updated with amount", id);
                } else if let Some(description) = description {
                    if description.trim().is_empty() {
                        return Err(ExpenseError::EmptyDescription);
                    }

                    storage.update_description(*id, description.clone());
                    println!("Expense with id {} updated with description", id);
                }
            }
            Self::Delete { id } => {
                if !storage.gets().iter().any(|e| e.id == *id) {
                    return Err(ExpenseError::ExpenseNotFound(*id));
                }
                storage.delete(*id);
                println!("Expense with id {} deleted", id);
            }
            Self::Summary { month } => {
                if let Some(month) = month {
                    if !(&1..=&12).contains(&month) {
                        return Err(ExpenseError::InvalidMonth(*month as u8));
                    }
                    let month_summary = storage.summary_by_month(*month);
                    let month_name = month_number_to_name(*month as u8);
                    if month_summary == 0.0 {
                        println!("No expenses for {}", month_name);
                    } else {
                        println!("Total amount of {}: {} rub", month_name, month_summary);
                    }
                } else {
                    println!("Total amount: {} rub", storage.summary());
                }
            }
            Self::List => {
                //ID  Date       Description  Amount
                //1   2024-08-06  Lunch        $20
                //2   2024-08-06  Dinner       $10
                if storage.gets().is_empty() {
                    println!("No expenses");
                } else {
                    for expense in storage.gets() {
                        println!(
                            "{:<2} {:<15} {:<15} {:<10}",
                            expense.id,
                            expense.created_at.format("%Y-%m-%d"),
                            expense
                                .description
                                .as_ref()
                                .unwrap_or(&String::from("None")),
                            expense.amount
                        );
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn command() -> Command {
    Command::new("expense-tracker")
        .version("0.1")
        .about("Expense tracker CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add an expense")
                .arg(Arg::new("amount").required(true).short('a').long("amount"))
                .arg(Arg::new("description").short('d').long("description")),
        )
        .subcommand(
            Command::new("update")
                .about("Update an expense")
                .arg(Arg::new("id").required(true).short('i').long("id"))
                .arg(Arg::new("amount").short('a').long("amount"))
                .arg(Arg::new("description").short('d').long("description")),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete an expense")
                .arg(Arg::new("id").required(true).short('i').long("id")),
        )
        .subcommand(
            Command::new("summary")
                .about("Get summary (for a month)")
                .arg(Arg::new("month").short('m').long("month")),
        )
        .subcommand(Command::new("list").about("List all expenses"))
}

pub fn parse(args: &ArgMatches) -> Result<Action, ExpenseError> {
    match args.subcommand() {
        Some(("add", args)) => Ok(Action::Add {
            amount: args.get_one::<String>("amount").unwrap().parse()?,
            description: args.get_one::<String>("description").cloned(),
        }),
        Some(("update", args)) => Ok(Action::Update {
            id: args.get_one::<String>("id").unwrap().parse()?,
            amount: args
                .get_one::<String>("amount")
                .map(|a| a.parse::<f64>().unwrap()),
            description: args.get_one::<String>("description").cloned(),
        }),
        Some(("delete", args)) => Ok(Action::Delete {
            id: args.get_one::<String>("id").unwrap().parse()?,
        }),
        Some(("summary", args)) => Ok(Action::Summary {
            month: args
                .get_one::<String>("month")
                .map(|m| m.parse::<u32>().unwrap()),
        }),
        Some(("list", _)) => Ok(Action::List),
        _ => Err(ExpenseError::UnknownCommand),
    }
}
