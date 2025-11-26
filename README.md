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
- Display query results in a formatted CSV table
- Analyze database tables (schema info, row counts, data types)
- Query plan explanation (--explain)
- Interactive TUI Mode (SQL Editor)

### Interactive TUI
- Navigate with keyboard in a clean terminal interface
- Input SQL queries and see results immediately
- Press 'q' to quit

### Query Execution
- Supports all standard SQL queries (SELECT, INSERT, UPDATE, DELETE, etc.)
- Displays results in a table-like format
- Handles NULLs, BLOBs, and empty result sets 

### Saved Queries
- Press 'F2' to view saved queries
- Type the number associated with a saved query to load it into the input field
- Save queries with 'Ctrl+S':
    - Prompted to enter a name for the query
    - Saved queries are sent to 'saved_queries.json'

### Autocomplete
- Dynamically suggests SQL keywords and saved query names as you type
- Press 'Tab' to autofill the first suggestion

### Navigating and Editing
- Use 'Backspace' to delete characters
- Use 'Esc' to cancel saving/selection screens
- Supports multi-case input for both SQL and saved query names

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

## Launch TUI

rustdb tui my.db

In the TUI:
- Type to enter SQL
- Enter -> Execute
- q -> Quit


## Saving a Query

- Type your query
    'SELECT * FROM users;'
- Press 'Ctrl+S'
- Enter a name for the query and press 'Enter'

## Loading a Saved Query:

- Press 'F2' to show saved queries
- Type the number corresponding to the saved query
- Press 'Enter' to autofill it into the input field


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
- crossterm: for terminal input/output
- ratatui: for TUI rendering

## Notes
- CSV and JSON export files are not tracked by Git (.gitignore should include *.csv and *.json).
- Ensure Rust and Cargo are installed and updated
- Compatible with SQLite databases only (at the moment).

## Future Features
### Some future features that could be added:
- Cycle through autocomplete suggestions with arrow keys
- Query history navigation
