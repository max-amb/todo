use std::{env, path::PathBuf};
use clap::{error::*,  Parser, Subcommand};
mod add; mod gui; mod list; mod delete;

#[derive(Parser)]
#[command(name = "todo")]
#[command(version = "v.alpha_gui")]
#[command(author = "max-amb <max_a@e.email>")]
#[command(about = "Todo - A simple todo app written in rust :)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// To open the gui - requires ncurses
    Gui {},
    /// To add a task - needs to be done in speech marks for now
    Add { task: String },
    /// To delete a task - use the task ID
    Delete { task_id: i32 },
    /// To list all tasks
    List {},
}

fn main() -> Result<(), ErrorKind> {
    // Parsing arguments
    let cli = Cli::parse();
    match &cli.command {
        Commands::Gui {} => {
            gui::run_gui();
        }
        Commands::Add { task } => {
            add::console_add(task.to_owned());
        }
        Commands::Delete { task_id } => {
            delete::console_delete(task_id.to_owned());
        }
        Commands::List{} => {
            list::console_list();
        }
    }
    return Ok(());
}

pub fn generate_path() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    path.pop();
    path.push("tasks");
    path
}