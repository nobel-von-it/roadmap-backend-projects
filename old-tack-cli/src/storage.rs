use std::io::Write;
use std::{fs, io};
use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::task::Status;
use crate::{error::TaskCliError, task::Task};

pub fn open() -> io::Result<File> {
    let path = PathBuf::from("./db.json");
    if path.exists() {
        return File::options().read(true).write(true).open(&path);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonStorage {
    tasks: Vec<Task>,
}

impl JsonStorage {
    pub fn load() -> Result<JsonStorage, TaskCliError> {
        let file = open()?;

        let mut reader = io::BufReader::new(file);
        let mut storage: JsonStorage = serde_json::from_reader(&mut reader)?;
        storage.update_indexes();
        Ok(storage)
    }
    pub fn save(&mut self) -> Result<(), TaskCliError> {
        self.update_indexes();
        let file = open()?;
        file.set_len(0)?;

        let mut writer = io::BufWriter::new(file);
        serde_json::to_writer(&mut writer, self)?;

        writer.flush()?;
        Ok(())
    }

    pub fn update_indexes(&mut self) {
        for (i, task) in self.tasks.iter_mut().enumerate() {
            task.id = i + 1;
        }
    }

    pub fn delete(&mut self, id: usize) {
        self.tasks.retain(|task| task.id != id);
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn update_text(&mut self, id: usize, text: String) {
        self.tasks
            .iter_mut()
            .find(|task| task.id == id)
            .unwrap()
            .description = text;
    }

    pub fn update_status(&mut self, id: usize, status: Status) {
        self.tasks
            .iter_mut()
            .find(|task| task.id == id)
            .unwrap()
            .status = status;
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn gets(&self) -> &[Task] {
        self.tasks.as_slice()
    }
}
