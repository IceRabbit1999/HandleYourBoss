use std::io;
use std::string::ToString;
use chrono::Local;
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use crate::args::Args;
use crate::storage::TaskList;
mod args;
mod storage;

const SAVE_PATH: &str = "./target/debug/save.hyb";
const LOG_PATH: &str = "/data/repo/HandleYourBoss/log";

fn main() {
    // init log system
    let file_appender = tracing_appender::rolling::daily(LOG_PATH, "dyb.log");
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
    let list = init_list();

    if arg.list.is_none() {
        print!("{:?}", list.tasks);
    }
}

fn init_list() -> TaskList {
    let mut list = TaskList::new();
    if list.load(SAVE_PATH.to_string()).is_ok() {
        println!("load data success");
    }else {
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