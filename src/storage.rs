use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::info;

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl Display for TaskList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, task) in self.tasks.iter().enumerate() {
            writeln!(f, "{}: \t {}", index + 1, task).expect("to_string error")
        }
        write!(f, "")
    }
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::new(),
        }
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) -> Result<()>{
        let mut file = File::create(&path)?;
        let json = serde_json::to_string(&self.tasks)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load(&mut self, path: String) -> Result<()> {
        let save_path = Path::new(&path);
        let mut file = File::open(save_path)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;
        let list: Vec<Task> = serde_json::from_str(&json)?;
        info!("load data from json success");
        self.tasks = list;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub content: String,
    pub tag: String,
    pub status: String,
    // create_time: String,
    // update_time: String
}


impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Content: {}\t Tag: {}\t Status: {} ", self.content, self.tag, self.status)
    }
}

impl Task {
    pub fn new(content: String, tag: String, status: String) -> Task {
        Task {
            content,
            tag,
            status
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Tag {
    Annoying,
    StupidButHaveTo,
    TimeWasting,
    Funny,
    Easy,
    Creative,
    Worth
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Done,
    InProgress,
    JustStarted
}



#[cfg(test)]
mod tests {
    use crate::storage::{Task, TaskList};

    // use super::*;
    #[test]
    fn save_json() -> anyhow::Result<()> {
        let list = vec![Task::new("Task1".to_string(), "Worth".to_string(), "JustStarted".to_string()),
                        Task::new("Task2".to_string(), "TimeWasting".to_string(), "InProgress".to_string()),
                        Task::new("Handle your boss dev".to_string(), "Funny".to_string(), "InProgress".to_string()),
                        Task::new("win a dota2 game".to_string(), "Easy".to_string(), "Done".to_string())
        ];
        let tasks = TaskList {
            tasks: list
        };

        let json = serde_json::to_string(&tasks).unwrap();
        println!("{}", json);

        tasks.save("./target/debug/save.hyb")
    }

    #[test]
    fn load() {
        let mut list = TaskList::new();
        list.load("./save.hyb".to_string()).unwrap();
        println!("{:?}", list.tasks)
    }
}