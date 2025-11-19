mod cli;
mod query;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow:: Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Query { db_path, sql } => {
            query::run_query(&db_path, &sql)?;
        }
        Commands::Analyze { db_path, table } => {
            print!("Analyze not implemented yet. Coming soon!");
        }
    }

    Ok(())
}
