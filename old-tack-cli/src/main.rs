use task_cli::{
    commands::{command, parse},
    error::TaskCliError,
    storage::JsonStorage,
};

fn main() -> Result<(), TaskCliError> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let action = parse(&command().get_matches_from(args));
    let mut storage = match JsonStorage::load() {
        Ok(storage) => storage,
        Err(e) => {
            eprintln!("Failed to load storage: {}", e);
            JsonStorage::default()
        }
    };
    if let Err(e) = action.execute(&mut storage) {
        eprintln!("Failed to execute action:: {}", e);
    }
    storage.save()
}
