mod cli;
mod query;
mod analyze;
mod export;
mod tui;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow:: Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Query { db_path, sql, csv, json, explain, profile} => {
            query::run_query(&db_path, &sql, csv.as_deref(), json.as_deref(), explain, profile)?;
        }
        Commands::Analyze { db_path, table } => {
            analyze::analyze_table(&db_path, &table)?;
        }
        Commands::Tui { db_path } => {
            tui::start_tui(&db_path)?;
        }
    }

    Ok(())
}
