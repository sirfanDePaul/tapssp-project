use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    Terminal, 
    backend::{CrosstermBackend}, 
    layout::{Constraint, Direction, Layout}, 
    style::{Color, Style}, 
    widgets::{Block, Borders, Paragraph},
    text::{Text, Span, Line},
    prelude,
};
use rusqlite::{
    Connection,
    types::ValueRef,
};
use std::io;
use crossterm::cursor;

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

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1)
                ])
                .split(f.size());

            let input_block = Paragraph::new(input.as_str())
                    .block(Block::default().title("SQL Input").borders(Borders::ALL));

            let output_text = Text::from(
                output.iter().map(|line| Line::from(line.as_str())).collect::<Vec<Line>>()
            );

            let output_block = Paragraph::new(output_text)
                    .block(Block::default().title("Query Output").borders(Borders::ALL));

            f.render_widget(input_block, chunks[0]);
            f.render_widget(output_block, chunks[1]);
        })?;

        // Input handling
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => { input.pop(); }
                    KeyCode::Enter => {
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
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;

    Ok(())
}