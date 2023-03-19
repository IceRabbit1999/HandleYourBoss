use std::io;
use std::os::linux::raw::stat;
use std::string::ToString;
use chrono::Local;
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use crate::args::{Args, Command, SubList};
use crate::storage::{Task, TaskList};

mod args;
mod storage;

const SAVE_PATH: &str = "./save.hyb";
const LOG_PATH: &str = "/data/repo/HandleYourBoss/log";

fn main() {
    // init log system
    let file_appender = tracing_appender::rolling::daily(LOG_PATH, "hyb.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(io::stdout)
        .with_writer(non_blocking)
        .with_ansi(false)
        .event_format(format)
        .init();

    let arg = Args::parse();
    let mut list = init_list();

    match &arg.cmd {
        Command::List { arg, tag, status } => {
            // hyb list --tag <tag> --status <status>
            // hyb list --tag <tag>
            // hyb list --status <status>
            // hyb list all
            match (arg, tag, status) {
                (Some(_), None, None) => { println!("{}", list) }
                (_, Some(tag), None) => {
                    let tag_list = list.tasks.iter()
                        .filter(|&task| task.tag == *tag)
                        .collect::<Vec<&Task>>();

                    if !tag_list.is_empty() {
                        for &task in &tag_list {
                            println!("{}", task);
                        }
                    }else {
                        println!("No such tasks found")
                    }
                }
                (_, None, Some(status)) => {
                    let status_list = list.tasks.iter()
                        .filter(|&task| task.status == *status)
                        .collect::<Vec<&Task>>();

                    if !status_list.is_empty() {
                        for &task in &status_list {
                            println!("{}", task);
                        }
                    }else {
                        println!("No such tasks found")
                    }
                }
                (_, Some(tag), Some(status)) => {
                    let l = list.tasks.iter().filter(|&task| task.tag == *tag && task.status == *status).collect::<Vec<&Task>>();
                    if !l.is_empty() {
                        for &task in &l {
                            println!("{}", task);
                        }
                    }else {
                        println!("No such tasks found")
                    }
                }
                _ => { println!("{}", list) }
            }
        }
        Command::Add { content, tag, status } => {
            list.tasks.push(Task::new(content.clone(), tag.clone(), status.clone()));
            list.save(SAVE_PATH).expect("Unable to save tasks, your history may be lost");
        }
        Command::Delete {index, tag, status} => {
            match (index, tag, status) {
                (Some(index), _, _) => {
                    list.tasks.remove((*index - 1) as usize);
                    // list.save(SAVE_PATH).expect("Unable to save tasks, your history may be lost");
                },
                (None, Some(tag), None) => {
                    list.tasks.retain(|task| task.tag != *tag);
                    list.save(SAVE_PATH).expect("Unable to save tasks, your history may be lost");
                },
                (None, Some(tag), Some(status)) => {
                    list.tasks.retain(|task| task.tag != *tag && task.status != *status);
                    list.save(SAVE_PATH).expect("Unable to save tasks, your history may be lost");
                },
                (None, None, Some(status)) => {
                    list.tasks.retain(|task| task.status != *status);
                    list.save(SAVE_PATH).expect("Unable to save tasks, your history may be lost");
                },
                _ => {}
            }
        }
    };

    list.save(SAVE_PATH).expect("Unable to save tasks, your history may be lost");
}

fn init_list() -> TaskList {
    let mut list = TaskList::new();
    if list.load(SAVE_PATH.to_string()).is_ok() {
        println!("load data success");
    } else {
        println!("load data fail");
    }
    list
}

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S.%3f"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn string() {
        let a = String::from("a");
        let v = vec!["1".to_string(), "a".to_string(), "3".to_string()];
        let v2 = v.iter()
            .filter(|&s| *s == a)
            .map(|s| *s)
            .collect::<Vec<String>>();
    }

    #[test]
    fn number() {
        let a = 1;
        let v = vec![1, 2, 3, 1];
        let v2 = v.iter()
            //.map(|v| v * 2)
            .filter(|&s| *s == a)
            .collect::<Vec<i32>>();
    }
}