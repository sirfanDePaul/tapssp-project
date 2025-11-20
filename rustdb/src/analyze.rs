use rusqlite::{Connection, Result};
use comfy_table::Table;

pub fn analyze_table(db_path: &str, table: &str) -> Result<()> {
    let conn = Connection::open(db_path)?;

    // Get table schema
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({});", table))?;
    let columns = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?
    .collect::<Result<Vec<(String, String)>>>()?;

    println!("Schema for table '{}':", table);

    let mut schema_table = Table::new();
    schema_table.set_header(vec!["Column", "Type"]);
    for (name, col_type) in &columns {
        schema_table.add_row(vec![name, col_type]);
    }
    println!("{schema_table}");

    // Numeric column stats
    let numeric_columns: Vec<_> = columns
        .iter()
        .filter(|(_, t)| t.eq_ignore_ascii_case("INTEGER") || t.eq_ignore_ascii_case("REAL"))
        .collect();

    if !numeric_columns.is_empty() {
        println!("Column statistics:");
        let mut stats_table = Table::new();
        stats_table.set_header(vec!["Column", "Min", "Max", "Average"]);

        for (name, _) in numeric_columns {
            let query = format!(
                "SELECT MIN({0}), MAX({0}), AVG({0}) FROM {1}",
                name, table
            );
            let (min, max, avg): (Option<f64>, Option<f64>, Option<f64>) =
                conn.query_row(&query, [], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;

            stats_table.add_row(vec![
                name.to_string(),
                min.map(|v| v.to_string()).unwrap_or("NULL".to_string()),
                max.map(|v| v.to_string()).unwrap_or("NULL".to_string()),
                avg.map(|v| v.to_string()).unwrap_or("NULL".to_string()),
            ]);
        }
        println!("{stats_table}");
    }

    // Get row count
    let count: i32 = conn.query_row(
        &format!("SELECT COUNT (*) FROM {}", table),
        [],
        |r| r.get(0),
    )?;
    println!("Total rows: {}", count);

    Ok(())
}