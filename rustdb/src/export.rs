use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;
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

// Export benchmark.json when profile tag is called
#[derive(Serialize, Deserialize)]
struct BenchmarkEntry {
    query: String,
    milliseconds: f64,
    timestamp: String,
}

pub fn save_benchmark_log(sql: &str, elapsed: Duration) -> anyhow::Result<()> {
    let ms = elapsed.as_secs_f64() * 1000.0;
    let entry = BenchmarkEntry {
        query: sql.to_string(),
        milliseconds: ms,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    // Reads existing log or create an empty list
    let path = "benchmark.json";
    let mut data = Vec::<BenchmarkEntry>::new();

    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        data = serde_json::from_str(&contents).unwrap_or_default();
    }

    // Add new entry
    data.push(entry);

    // Write updated log
    let mut file = File::create(path)?;
    file.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;

    Ok(())
}