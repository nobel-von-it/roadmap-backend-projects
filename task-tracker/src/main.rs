use std::{fs, path::PathBuf, time::Duration};

use anyhow::Result;
use chrono::{DateTime, Local};
use clap::{Parser, ValueEnum};
use miniserde::json::{self, Array, Number, Object, Value};

#[derive(
    Debug,
    Clone,
    ValueEnum,
    miniserde::Serialize,
    miniserde::Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
enum TStatus {
    Done,
    Todo,
    InProgress,
}

impl ToString for TStatus {
    fn to_string(&self) -> String {
        match self {
            TStatus::Done => "done".to_string(),
            TStatus::Todo => "todo".to_string(),
            TStatus::InProgress => "in-progress".to_string(),
        }
    }
}

#[derive(Debug, Parser)]
enum TTCommands {
    Add { content: String },
    Update { id: u64, content: String },
    Delete { id: u64 },
    Mark { status: TStatus, id: u64 },
    List { status: Option<TStatus> },
}

#[derive(Debug, Clone, miniserde::Serialize, miniserde::Deserialize)]
struct TTask {
    id: u64,
    desc: String,
    status: TStatus,
    created_at: String,
    updated_at: String,
}

impl TTask {
    fn print(&self, tabbed: usize, mid: usize, mdesc: usize, mstatus: usize) {
        println!(
            "{}{:<mid$} {:<mdesc$} {:<mstatus$}",
            " ".repeat(tabbed),
            self.id,
            self.desc,
            self.status.to_string(),
        )
    }
    fn any_from(ttj: &Object) -> Result<Self> {
        let id = if let Some(Value::Number(Number::U64(id))) = ttj.get("id") {
            *id
        } else {
            return Err(anyhow::anyhow!("id not found"));
        };
        let desc = if let Some(Value::String(desc)) = ttj.get("desc") {
            desc.clone()
        } else {
            return Err(anyhow::anyhow!("desc not found"));
        };
        let status = if let Some(Value::String(status)) = ttj.get("status") {
            match status.as_str() {
                "done" => TStatus::Done,
                "todo" => TStatus::Todo,
                "in-progress" => TStatus::InProgress,
                _ => return Err(anyhow::anyhow!("invalid status")),
            }
        } else {
            return Err(anyhow::anyhow!("status not found"));
        };
        let created_at = if let Some(Value::String(created_at)) = ttj.get("created_at") {
            created_at.clone()
        } else {
            return Err(anyhow::anyhow!("created_at not found"));
        };
        let updated_at = if let Some(Value::String(updated_at)) = ttj.get("updated_at") {
            updated_at.clone()
        } else {
            return Err(anyhow::anyhow!("updated_at not found"));
        };

        Ok(TTask {
            id,
            desc,
            status,
            created_at,
            updated_at,
        })
    }
}

struct JsonStorage {
    path: PathBuf,

    content: Vec<TTask>,
}

impl JsonStorage {
    fn check(path: &PathBuf) -> Result<()> {
        if !path.exists() {
            log::info!("creating file {}", path.display());
            fs::create_dir_all(path.parent().ok_or(anyhow::anyhow!("parent not found"))?)?;
            fs::File::create(path)?;
        } else {
            log::info!("file {} exists", path.display());
        }

        Ok(())
    }
    fn load(path: PathBuf) -> Result<Self> {
        JsonStorage::check(&path)?;

        let content =
            json::from_str::<Vec<TTask>>(&fs::read_to_string(&path)?).unwrap_or_else(|e| {
                log::error!("failed to parse json: {}", e);
                Vec::new()
            });

        Ok(Self { path, content })
    }
    fn save(&self) -> Result<()> {
        let json = json::to_string(&self.content);
        fs::write(&self.path, json)?;
        Ok(())
    }
    fn add<S: AsRef<str>>(&mut self, content: S) -> TTask {
        let tt = TTask {
            id: self.content.len() as u64,
            desc: content.as_ref().to_string(),
            status: TStatus::Todo,
            created_at: Local::now().to_rfc3339(),
            updated_at: Local::now().to_rfc3339(),
        };
        self.content.push(tt.clone());

        tt
    }
    fn upadte<S: AsRef<str>>(&mut self, id: u64, content: S) -> Option<TTask> {
        let tt = self.content.iter_mut().find(|tt| tt.id == id)?;
        tt.updated_at = Local::now().to_rfc3339();
        tt.desc = content.as_ref().to_string();

        Some(tt.clone())
    }
    fn delete(&mut self, id: u64) -> Option<TTask> {
        let tt = self.content.iter().find(|tt| tt.id == id)?.clone();
        self.content.retain(|tt| tt.id != id);
        Some(tt)
    }
    fn mark(&mut self, status: TStatus, id: u64) -> Option<TTask> {
        let tt = self.content.iter_mut().find(|tt| tt.id == id)?;
        tt.status = status;
        tt.updated_at = Local::now().to_rfc3339();
        Some(tt.clone())
    }
    fn get_ms(&self) -> (usize, usize, usize, usize) {
        let mut mid = 0;
        let mut mdesc = 0;
        let mut mstatus = 0;
        let mut mcreated_at = 0;

        for tt in self.content.iter() {
            mid = mid.max(tt.id.to_string().len());
            mdesc = mdesc.max(tt.desc.len());
            mstatus = mstatus.max(tt.status.to_string().len());
            mcreated_at = mcreated_at.max(tt.created_at.len());
        }

        (mid, mdesc, mstatus, mcreated_at)
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let tt_command = TTCommands::parse();

    let mut storage = JsonStorage::load(PathBuf::from("tasks.json"))?;
    match tt_command {
        TTCommands::Add { content } => {
            println!("ADD:");

            let tt = storage.add(content);
            tt.print(2, 0, 0, 0);

            storage.save()?;
        }
        TTCommands::Update { id, content } => {
            println!("UPDATE:");
            let ntt = storage
                .upadte(id, content)
                .ok_or(anyhow::anyhow!("task not found"))?;

            ntt.print(2, 0, 0, 0);

            storage.save()?;
        }
        TTCommands::Delete { id } => {
            println!("DELETE:");
            let dtt = storage
                .delete(id)
                .ok_or(anyhow::anyhow!("task not found"))?;
            dtt.print(2, 0, 0, 0);

            storage.save()?;
        }
        TTCommands::Mark { status, id } => {
            println!("MARK:");
            let mtt = storage
                .mark(status, id)
                .ok_or(anyhow::anyhow!("task not found"))?;
            mtt.print(2, 0, 0, 0);

            storage.save()?;
        }
        TTCommands::List { status } => {
            let (mid, mdesc, mstatus, _) = storage.get_ms();

            println!("LIST:");

            for tt in storage.content {
                if let Some(status) = &status {
                    if &tt.status == status {
                        tt.print(2, mid, mdesc, mstatus)
                    }
                } else {
                    tt.print(2, mid, mdesc, mstatus)
                }
            }
        }
    }

    Ok(())
}
