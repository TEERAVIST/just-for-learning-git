mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::Cli;
use utils::logger::init_logger;

fn main() -> anyhow::Result<()> {
    init_logger();
    let cli = Cli::parse();

    match &cli.command {
        Some(cli::Commands::Greet { name }) => {
            commands::greet::run(name)?;
        }
        Some(cli::Commands::Math { a, b }) => {
            commands::math::run(*a, *b)?;
        }
        None => {
            println!("No subcommand used. Try --help");
        }
    }

    Ok(())
}

// add this somewhere

// add this to main branch
