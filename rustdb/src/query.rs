use rusqlite::{Connection, Result};
use comfy_table::{Table, presets::UTF8_FULL};
use crate::export::{export_to_csv, export_to_json, save_benchmark_log};
use serde::Serialize;
use anyhow::Result as AnyResult;


#[derive(Serialize)]
struct RowRecord {
    values: Vec<String>,
}

/// Runs a SQL query and prints results in a formatted table, optionally export CSV/JSON
pub fn run_query(db_path: &str, sql: &str, csv_file: Option<&str>, json_file: Option<&str>, explain: bool, profile: bool) -> AnyResult<()> {
    let conn = Connection::open(db_path)?;

    if explain {
        // Run explain query plan
        let mut stmt = conn.prepare(&format!("EXPLAIN QUERY PLAN {}", sql))?;
        let rows_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?, // id
                row.get::<_, i32>(1)?, // parent
                row.get::<_, i32>(2)?, // notused
                row.get::<_, String>(3)?, // detail
            ))
        })?;

        let mut table = Table::new();
        table.set_header(vec!["id", "parent", "notused", "detail"]);

        for row in rows_iter {
            let (id, parent, notused, detail) = row?;
            table.add_row(vec![id.to_string(), parent.to_string(), notused.to_string(), detail.to_string()]);
        }

        println!("{table}");
        return Ok(()); // Skip normal query execution
    }

    // Start profile query plan
    let start_time = if profile {
        Some(std::time::Instant::now())
    } else {
        None
    };

    let mut stmt = conn.prepare(sql)?;

    let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
    let column_count = stmt.column_count();

    // Map rows
    let rows_iter = stmt.query_map([], |row| {
        Ok(RowRecord {
            values: row_to_strings(row, column_count),
        })
    })?;

    // Collect rows into Vec
    let rows: Vec<RowRecord> = rows_iter.collect::<Result<_, rusqlite::Error>>()?;

    // Print to console
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(column_names.clone());

    for row in &rows {
        table.add_row(row.values.clone());
    }

    println!("{table}");

    // Export CSV if requested
    if let Some(file) = csv_file {
        let headers: Vec<&str> = column_names.iter().map(|s| s.as_str()).collect();
        let row_values: Vec<&[String]> = rows.iter().map(|r| r.values.as_slice()).collect();
        export_to_csv(file, &headers, &row_values)?;
        println!("Exported to CSV: {}", file);
    }

    // Export JSON if requested
    if let Some(file) = json_file {
        export_to_json(file, &rows)?;
        println!("Exported to JSON: {}", file);
    }

    // End profile query
    if let Some(start) = start_time {
        let elapsed = start.elapsed();
        println!(
            "Query executed in: {:.4} ms",
            elapsed.as_secs_f64() * 1000.0
        );
        // Writes timing to benchmark.json
        save_benchmark_log(sql, elapsed)?;
    }

    Ok(())
}

/// Converts all types to String properly
fn row_to_strings(row: &rusqlite::Row, column_count: usize) -> Vec<String> {
    (0..column_count)
        .map(|i| match row.get_ref(i) {
            Ok(rusqlite::types::ValueRef::Integer(v)) => v.to_string(),
            Ok(rusqlite::types::ValueRef::Real(v)) => v.to_string(),
            Ok(rusqlite::types::ValueRef::Text(t)) => String::from_utf8_lossy(t).to_string(),
            Ok(rusqlite::types::ValueRef::Blob(_)) => "<BLOB>".to_string(),
            Ok(rusqlite::types::ValueRef::Null) | Err(_) => "NULL".to_string(),
        })
        .collect()
}