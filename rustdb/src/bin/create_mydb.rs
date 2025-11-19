use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // Create.Open database
    let conn = Connection::open("my.db")?;

    // Create users table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            age INTEGER
        )",
        [],
    )?;

    // Create products table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price REAL
        )",
        [],
    )?;

    // Insert sample users
    conn.execute(
        "INSERT INTO users (name, email, age) VALUES
            ('Alice', 'alice@example.com', 30),
            ('Bob', 'bob@example.com', 25),
            ('Charlie', 'charlie@example.com', 40)",
        [],
    )?;

    // Insert sample products
    conn.execute(
        "INSERT INTO products (name, price) VALUES
            ('Laptop', 1200.50),
            ('Mouse', 25.99),
            ('Keyboard', 75.00)",
        [],
    )?;

    println!("my.db created with sample tables and data.");
    Ok(())
}