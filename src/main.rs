use clap::{Parser, Subcommand};

mod sqlite;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[derive(Debug)]
struct Cli {
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

fn main() {
    let cli = Cli::parse();
    args_match(cli);
}

// Add error handling
fn args_match(args: Cli) {
    match args.command {
        Commands::Add { content } => {
            let task: String = content.unwrap().to_string();
            let result = sqlite::add(task);
            println!("{:?}", result);
        }
        Commands::Delete { id } => {
            let task_id: i32 = id.unwrap() as i32;
            println!("deleted task with id: {}", task_id);
        }
        Commands::List => {
            let todos = sqlite::list_todos();
            print!("{:?}", todos);
        }
        Commands::Update { id, new_data } => {
            let id = id.unwrap() as i32;
            let new_data = new_data.unwrap().to_string();
            println!("Updated task with id of {}. Updated task: {}", id, new_data);
        }
    }
}
