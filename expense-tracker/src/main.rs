use expense_tracker::{
    command::{command, parse},
    error::ExpenseError,
    storage::JsonStorage,
};

fn main() -> Result<(), ExpenseError> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let action = parse(&command().get_matches_from(args))?;
    let mut storage = match JsonStorage::load() {
        Ok(storage) => storage,
        Err(e) => {
            eprintln!("Failed to load storage: {}", e);
            JsonStorage::default()
        }
    };
    action.execute(&mut storage)?;
    storage.save()
}

