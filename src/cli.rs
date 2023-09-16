use clap::{Parser, Subcommand};
use rusqlite::Result;

mod helper;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[derive(Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        content: Option<String>,
    },
    Delete {
        id: Option<i32>,
    },
    List,
    Update {
        id: Option<i32>,
        new_data: Option<String>,
    },
}

pub fn args_match(args: Cli) {
    match args.command {
        Commands::Add { content } => {
            let task: String = content.unwrap().to_string();
            let result = helper::add(task);
            handle_error(result);
        }
        Commands::Delete { id } => {
            let _task_id: i32 = id.unwrap() as i32;
        }
        Commands::List => {
            let todos = helper::list_todos();
            handle_error(todos)
        }
        Commands::Update { id, new_data } => {
            let id = id.unwrap() as i32;
            let new_data = new_data.unwrap().to_string();
            println!("Updated task with id of {}. Updated task: {}", id, new_data);
        }
    }
}

fn handle_error(err: Result<()>) -> () {
    match err {
        Ok(_) => println!("Added new task!"),
        Err(value) => println!("{:?}", value),
    }
}
