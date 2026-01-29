use std::io::Write;
use std::{fs, io};
use std::{fs::File, path::PathBuf};

use chrono::Datelike;
use serde::{Deserialize, Serialize};

use crate::error::ExpenseError;
use crate::types::Expense;

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
    expenses: Vec<Expense>,
}

impl JsonStorage {
    pub fn load() -> Result<JsonStorage, ExpenseError> {
        let file = open()?;

        let mut reader = io::BufReader::new(file);
        let mut storage: JsonStorage = serde_json::from_reader(&mut reader)?;
        storage.update_indexes();
        Ok(storage)
    }
    pub fn save(&mut self) -> Result<(), ExpenseError> {
        self.update_indexes();
        let file = open()?;
        file.set_len(0)?;

        let mut writer = io::BufWriter::new(file);
        serde_json::to_writer(&mut writer, self)?;

        writer.flush()?;
        Ok(())
    }

    pub fn update_indexes(&mut self) {
        for (i, expense) in self.expenses.iter_mut().enumerate() {
            expense.id = i + 1;
        }
    }

    pub fn delete(&mut self, id: usize) {
        self.expenses.retain(|expense| expense.id != id);
    }

    pub fn add(&mut self, expense: Expense) {
        self.expenses.push(expense);
    }

    pub fn update_description(&mut self, id: usize, text: String) {
        self.expenses
            .iter_mut()
            .find(|expense| expense.id == id)
            .unwrap()
            .description = Some(text);
    }

    pub fn update_amount(&mut self, id: usize, amount: f64) {
        self.expenses
            .iter_mut()
            .find(|expense| expense.id == id)
            .unwrap()
            .amount = amount;
    }

    pub fn len(&self) -> usize {
        self.expenses.len()
    }

    pub fn is_empty(&self) -> bool {
        self.expenses.is_empty()
    }

    pub fn gets(&self) -> &[Expense] {
        self.expenses.as_slice()
    }

    pub fn summary(&self) -> f64 {
        self.expenses.iter().map(|e| e.amount).sum()
    }

    pub fn summary_by_month(&self, month: u32) -> f64 {
        self.expenses
            .iter()
            .filter(|e| e.created_at.month() == month)
            .map(|e| e.amount)
            .sum()
    }
}
