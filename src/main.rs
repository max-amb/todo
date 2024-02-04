use clap::{error::*,  Parser, Subcommand};
mod add;

#[derive(Parser)]
#[command(name = "todo")]
#[command(version = "v.alpha")]
#[command(author = "max-amb <max_a@e.email>")]
#[command(about = "Todo - A simple todo app written in rust :)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// To add a task - needs to be done in speech marks for now
    Add { task: String },
    /// To delete a task - use the task ID
    Delete { task_id: u32 },
    /// To list all tasks
    List {},
}

fn main() -> Result<(), ErrorKind> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { task } => {
            add::add(task.to_owned());
        }
        Commands::Delete { task_id } => {
            println!("Deleting task {task_id}")
        }
        Commands::List{} => {
            println!("Here are the tasks:")
        }
    }
    return Ok(());
}