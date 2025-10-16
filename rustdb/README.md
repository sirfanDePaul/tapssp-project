# **rustdb** - CLI Database Query and Analyzer Tool
### rustdb is a command-line tool written in Rust that allows users to connect to an SQLite database, run SQL queries, and analyze table structure and statistics. This project demonstrates Rust's strengths in systems programming, including ownership, borrowing, error handling, and performance measurement.

## Overview
### Goal:
Create a lightweight, fast, and reliable command-line interface (CLI) for database querying and analysis, similar to a simplified sqlite3 client.

### Features:
- Run SQL queries directly from the terminal
- Display query results in a formatted table
- Analyze database tables (schema info, row counts, data types)
- Measure query performance (execution time in ms)
- Built using safe concurrency, ownership, and well-structured Rust APIs