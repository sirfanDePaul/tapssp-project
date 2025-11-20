# **rustdb** - CLI Database Query and Analyzer Tool
### rustdb is a command-line tool written in Rust that allows users to connect to an SQLite database, run SQL queries, and analyze table structure and statistics. This project demonstrates Rust's strengths in systems programming, including ownership, borrowing, error handling, and performance measurement.

### This project highlights Rust's strengths in:
- ownership and borrowing
- error handling
- trait-based API design
- performance measurement
- safe systems-level tooling

## Overview
### Goal:
Create a lightweight, fast, and reliable command-line interface (CLI) for database querying and analysis, similar to a simplified sqlite3 client.

### Features:
- Run SQL queries directly from the terminal
- Display query results in a formatted table
- Analyze database tables (schema info, row counts, data types)
- Query plan explanation (--explain)
- Cross-platform
- Built using safe concurrency, ownership, and well-structured Rust APIs

## Usage
### Basic Query

cargo run -- query <database> "<SQL_QUERY>"

Example:

cargo run -- query my.db "SELECT * FROM users;"

## Export Query Results
### Export to CSV:

cargo run -- query my.db "SELECT * FROM users;" --csv users.csv

### Export to JSON:

cargo run -- query my.db "SELECT * FROM users;" --json users.json

### Export to both CSV/JSON:

cargo run -- query my.db "SELECT * FROM users;" --csv users.csv --json users.json

## Analyze a Table

cargo run -- analyze <database> <table>

### Example:

cargo run -- analyze my.db users

### Displays:
- Table schema
- Numeric column statistics (min, max, average)
- Total row count

## Explain Query Plan:

cargo run -- query <database> "<SQL_QUERY>" --explain

Dispays SQLite's query plan for the provided SQL statement.

## Installation
### 1. Clone the repository:

git clone <your-repo-url>
cd rustdb

### 2. Build the project

cargo build --release

### 3. Run commands via 'cargo run' as shown in the usage section

## Dependencies
- rusqlite: SQLite bindings for Rust
- comfy_table: Pretty table output
- serde and serde_json: JSON serialization
- csv: CSV export
- anyhow: Error handling

## Notes
- CSV and JSON export files are not tracked by Git (.gitignore should include *.csv and *.json).
- Ensure Rust and Cargo are installed and updated
- Compatible with SQLite databases only (at the moment).
