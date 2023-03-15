use clap::Parser;

#[derive(Parser)]
#[command(name = "Handle Your Boss")]
#[command(version = "0.1")]
#[command(author = "IceRabbit")]
#[command(about = "Handle your boss by doing the minimum amount of work")]
pub struct Args {
    #[arg(long)]
    pub list: Option<String>
}