use clap::Parser;
mod cli;
fn main() {
    let cli = cli::Cli::parse();
    cli::args_match(cli);
}

// Add error handling
