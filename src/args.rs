use clap::{Parser, Subcommand, ArgGroup};

#[derive(Parser)]
#[command(name = "Handle Your Boss")]
#[command(version = "0.1")]
#[command(author = "IceRabbit")]
#[command(about = "Handle your boss by doing the minimum amount of work")]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Find existing tasks
    List {
        #[command(subcommand)]
        arg: Option<SubList>,
        #[arg(long)]
        tag: Option<String>,
        #[arg(long)]
        status: Option<String>
    },
    /// Add tasks
    Add {
        #[arg(long, short)]
        /// Set your task content
        content: String,
        #[arg(long)]
        tag: String,
        #[arg(long)]
        status: String
    },
    #[command(group(
    ArgGroup::new("delete")
    .required(true)
    .args(["index"]),
    ))]
    Delete {
        #[arg(long)]
        index: Option<i32>,
        #[arg(long)]
        tag: Option<String>,
        #[arg(long)]
        status: Option<String>
    }
}

#[derive(Subcommand)]
pub enum SubList {
    /// List all existing tasks
    All,
}

#[derive(Subcommand)]
pub enum SubAdd {
    Content { content: Option<String>},
    Tag { tag: Option<String>},
    Status { status : Option<String>},
}