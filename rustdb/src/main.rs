mod cli;
mod query;
mod analyze;
mod export;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow:: Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Query { db_path, sql, csv, json, explain} => {
            query::run_query(&db_path, &sql, csv.as_deref(), json.as_deref(), explain)?;
        }
        Commands::Analyze { db_path, table } => {
            analyze::analyze_table(&db_path, &table)?;
        }
    }

    Ok(())
}
