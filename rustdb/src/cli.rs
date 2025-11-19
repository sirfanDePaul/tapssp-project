use clap::{Parser, Subcommand};

/// CLI Interface
#[derive(Parser, Debug)]
#[command(name = "rustdb")]
#[command(version)]
#[command(about = "A lightweight command-line SQLite querty and analysis tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run an SQL query against a database
    Query {
        /// Path to database file
        db_path: String,

        /// SQL query to run
        sql: String,
    },

    /// Analyze a database table (schema, row count, etc.)
    Analyze {
        /// Path to database file
        db_path: String,

        /// Table name to analyze
        table: String,
    },
}