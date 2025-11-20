use serde::Serialize;
use std::fs::File;
use anyhow::Result;

/// Export a vector of records (Vec<Vec<String>>) to CSV
pub fn export_to_csv(file_path: &str, headers: &[&str], rows: &[&[String]]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(file_path)?;
    wtr.write_record(headers)?;
    for row in rows {
        wtr.write_record(&row[..])?;
    }
    
    wtr.flush()?;
    Ok(())
}

/// Export a vector of serializable records to JSON
pub fn export_to_json<T: Serialize>(file_path: &str, data: &[T]) -> Result<()> {
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}