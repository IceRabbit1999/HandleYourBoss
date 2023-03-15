use std::fmt::{Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
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
        self.tasks = list;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    content: String,
    tag: Tag,
    status: Status,
    // create_time: String,
    // update_time: String
}

impl Task {
    pub fn new(content: String) -> Task {
        Task {
            content,
            tag: Tag::Annoying,
            status: Status::JustStarted
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
    Creative
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
        let mut list = Vec::new();
        for i in 0..10 {
            list.push(Task::new(i.to_string()))
        }
        let tasks = TaskList {
            tasks: list
        };

        let json = serde_json::to_string(&tasks).unwrap();
        println!("{}", json);

        tasks.save("../target/debug/save.hyb")
    }

    #[test]
    fn load() {
        let mut list = TaskList::new();
        list.load("./save.hyb".to_string()).unwrap();
        println!("{:?}", list.tasks)
    }
}