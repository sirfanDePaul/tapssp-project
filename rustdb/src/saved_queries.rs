use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SavedQuery {
    pub name: String,
    pub sql: String,
}

const FILE_PATH: &str = "saved_queries.json";

/// Load all saved queries from file
pub fn load_saved_queries() -> Vec<SavedQuery> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();
    }
    let contents = fs::read_to_string(FILE_PATH).unwrap_or_default();
    serde_json::from_str(&contents).unwrap_or_default()
}

/// Save all queries to file\
pub fn save_new_query(name: &str, sql: &str) -> anyhow::Result<()> {
    let mut queries = load_saved_queries();
    queries.push(SavedQuery {
        name: name.to_string(),
        sql: sql.to_string(),
    });
    let contents = serde_json::to_string_pretty(&queries)?;
    fs::write(FILE_PATH, contents)?;
    Ok(())
}