use rusqlite::{Connection, Row};
use comfy_table::{Table, presets::UTF8_FULL};
use anyhow::Result;

/// Runs a SQL query and prints results in a formatted table
pub fn run_query(db_path: &str, sql: &str) -> Result<()> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare(sql)?;

    let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
    let column_count = stmt.column_count();

    let rows = stmt.query_map([], move |row| {
        Ok(extract_row(row, column_count))
    })?;

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(column_names);

    for row in rows {
        table.add_row(row?);
    }

    println!("{table}");

    Ok(())
}

/// Extract a row into Vec<String> for table display
fn extract_row(row: &Row, column_count: usize) -> Vec<String> {
    (0..column_count)
        .map(|i| {
            match row.get_ref(i) {
                Ok(val) => match val {
                    rusqlite::types::ValueRef::Null => "NULL".to_string(),
                    rusqlite::types::ValueRef::Integer(v) => v.to_string(),
                    rusqlite::types::ValueRef::Real(v) => v.to_string(),
                    rusqlite::types::ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                    rusqlite::types::ValueRef::Blob(_) => "<BLOB>".to_string(),
                },
                Err(_) => "NULL".to_string(),
            }
        })
        .collect()
}