use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers}, 
    execute, 
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    cursor,
};
use ratatui::{
    Terminal, 
    backend::{CrosstermBackend}, 
    layout::{Constraint, Direction, Layout}, 
    widgets::{Block, Borders, Paragraph},
    text::{Text, Line},
};
use rusqlite::{
    Connection,
    types::ValueRef,
};
use std::{io};

use crate::saved_queries::{SavedQuery, load_saved_queries, save_new_query};

enum InputMode {
    SQL,
    SaveName(String), // Holds current query to be named
    SelectSaved(Vec<SavedQuery>), // Show saved queries for selection
}

const SQL_KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE", "CREATE", "DROP", "ALTER",
    "TABLE", "INDEX", "VIEW", "TRIGGER", "JOIN", "INNER", "LEFT", "RIGHT", "FULL", "ON",
    "GROUP BY", "ORDER BY", "HAVING", "LIMIT", "OFFSET", "VALUES", "SET", "AND", "OR", "NOT",
];

pub fn start_tui(db_path: &str) -> anyhow::Result<()> {
    let conn = Connection::open(db_path)?;

    // Enable raw mode and disable echo
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    // Initialize Terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut output: Vec<String> = vec!["Enter SQL query and press Enter.\nPress q to quit.".into()];
    let mut input_mode = InputMode::SQL;
    let mut suggestions: Vec<String> = Vec::new();
    let mut number_buffer = String::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Input area
                    Constraint::Length(5), // Suggestions area
                    Constraint::Min(1) // Output area
                ])
                .split(f.size());
            
            // Input block
            let input_block = Paragraph::new(input.as_str())
                    .block(Block::default().title("SQL Input").borders(Borders::ALL));
            
            // Suggestion block
            let suggestion_text = Text::from(
                suggestions.iter().take(5).map(|line| Line::from(line.as_str())).collect::<Vec<Line>>()
            );
            let suggestion_block = Paragraph::new(suggestion_text)
                .block(Block::default().title("Suggestions").borders(Borders::ALL));

            // Output block
            let output_text = Text::from(
                output.iter().map(|line| Line::from(line.as_str())).collect::<Vec<Line>>()
            );

            let output_block = Paragraph::new(output_text)
                    .block(Block::default().title("Query Output").borders(Borders::ALL));

            f.render_widget(input_block, chunks[0]);
            f.render_widget(suggestion_block, chunks[1]);
            f.render_widget(output_block, chunks[2]);
        })?;

        // Input handling
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, kind: KeyEventKind::Press, modifiers, state: _ }) = event::read()? {
                match &mut input_mode {
                    InputMode::SQL => {
                        match (code, modifiers) {
                            (KeyCode::Char('q'), KeyModifiers::NONE) => break,
                            (KeyCode::F(2), KeyModifiers::NONE) => {
                                // Show saved queries
                                let saved_queries = load_saved_queries();
                                if saved_queries.is_empty() {
                                    output = vec!["No saved queries.".into()];
                                    input_mode = InputMode::SQL;
                                } else {
                                    output = saved_queries.iter().enumerate().map(|(i, q)| format!("{}: {}", i + 1, q.name)).collect();
                                    number_buffer.clear();
                                    input_mode = InputMode::SelectSaved(saved_queries);
                                }
                            }
                            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                                // Save current query
                                if !input.trim().is_empty() {
                                    input_mode = InputMode::SaveName(input.clone());
                                    input.clear();
                                    output = vec!["Enter a name for this query and press Enter:".into()];
                                }
                            }
                            (KeyCode::Char(c), _) => input.push(c),
                            (KeyCode::Backspace, _) => { input.pop(); }
                            (KeyCode::Tab, _) => {
                                // Autocomplete with first suggestion
                                if !suggestions.is_empty() {
                                    let first_suggestion = &suggestions[0];
                                    
                                    // IF it's a saved query, strip "Saved: " prefix
                                    let autofill = if first_suggestion.starts_with("Saved: ") {
                                        first_suggestion.trim_start_matches("Saved: ").to_string()
                                    } else {
                                        first_suggestion.clone()
                                    };

                                    input = autofill;
                                    suggestions.clear();
                                }
                            }
                            (KeyCode::Enter, _) => {
                                if !input.trim().is_empty() {
                                    // Prepare and run query
                                    let query_result = match conn.prepare(&input) {
                                        Ok(mut stmt) => {
                                            let col_count = stmt.column_count();
                                            match stmt.query_map([], move |row| {
                                                let mut vals = Vec::new();
                                                for i in 0..col_count {
                                                    let v = row.get_ref(i)?;
                                                    let s = match v {
                                                        ValueRef::Null => "NULL".to_string(),
                                                        ValueRef::Integer(i) => i.to_string(),
                                                        ValueRef::Real(r) => r.to_string(),
                                                        ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                                                        ValueRef::Blob(_) => "<BLOB>".to_string(),
                                                    };
                                                    vals.push(s);
                                                }

                                                Ok(vals.join(" | "))
                                            }) {
                                                Ok(rows_iter) => {
                                                    let rows: Vec<String> = rows_iter.filter_map(|r| r.ok()).collect();
                                                    if rows.is_empty() { vec!["Query returned 0 rows.".into()] } else { rows }
                                                }
                                                Err(e) => vec![format!("SQL error: {e}")]
                                            }
                                        }
                                        Err(e) => vec![format!("SQL error: {e}")]
                                    };

                                    output = query_result;
                                    input.clear();
                                }
                            }
                            _ => {}
                        }

                        // Update suggestions dynamically
                        if !input.is_empty() {
                            let input_upper = input.to_uppercase();
                            let mut matches: Vec<String> = SQL_KEYWORDS
                                .iter()
                                .filter(|kw| kw.starts_with(&input_upper))
                                .map(|s| s.to_string())
                                .collect();

                            let saved = load_saved_queries();
                            matches.extend(
                                saved.iter()
                                    .filter(|q| q.name.to_uppercase().starts_with(&input_upper))
                                    .map(|q| format!("Saved: {}", q.name))
                            );
                            suggestions = matches;
                        } else {
                            suggestions.clear();
                        }
                    }

                    InputMode::SaveName(query_text) => {
                        match code {
                            KeyCode::Char(c) => input.push(c),
                            KeyCode::Backspace => { input.pop(); }
                            KeyCode::Enter => {
                                if !input.trim().is_empty() {
                                    // Save with user provided name
                                    save_new_query(&input, query_text)?;
                                    output = vec![format!("Saved query as '{}'.", input)];
                                } else {
                                    // Save as "Unnamed Query"
                                    save_new_query("Unnamed Query", query_text)?;
                                    output = vec!["Saved query as 'Unnamed Query'.".into()];
                                }
                                input.clear();
                                input_mode = InputMode::SQL;
                            }
                            KeyCode::Esc => {
                                input.clear();
                                output = vec!["Save cancelled.".into()];
                                input_mode = InputMode::SQL;
                            }
                            _ => {}
                        }
                    }

                    InputMode::SelectSaved(_saved_list) => {
                        match code {
                            KeyCode::Char(c) if c.is_ascii_digit() => {
                                number_buffer.push(c);
                                output = vec![format!("Select query number: {}", number_buffer)];
                            }
                            KeyCode::Enter => {
                                if let Ok(index) = number_buffer.parse::<usize>() {
                                    let saved_queries = load_saved_queries();
                                    if index >= 1 && index <= saved_queries.len() {
                                        input = saved_queries[index - 1].sql.clone();
                                        output = vec![format!("Loaded query '{}'.", saved_queries[index - 1].name)];
                                    } else {
                                        output = vec!["Invalid selection.".into()];
                                    }
                                }
                                number_buffer.clear();
                                input_mode = InputMode::SQL;
                            }
                            KeyCode::Esc => {
                                number_buffer.clear();
                                input_mode = InputMode::SQL;
                                output = vec!["Cancelled loading saved query.".into()];
                            }
                            KeyCode::Backspace => {
                                number_buffer.pop();
                                output = vec![format!("Select query number: {}", number_buffer)];
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;

    Ok(())
}