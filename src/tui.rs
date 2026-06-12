use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Terminal,
};
use std::io;
use std::process::Command;
use std::time::{Duration, Instant};

use crate::ports::{self, PortEntry};

pub fn run() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut state = AppState::new();
    state.refresh();

    loop {
        terminal.draw(|f| ui(f, &mut state))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match handle_key(key, &mut state) {
                    Action::Quit => return Ok(()),
                    Action::Continue => {}
                }
            }
        }

        // Auto-refresh every 30s
        if state.last_refresh.elapsed() > Duration::from_secs(30) {
            state.refresh();
        }
    }
}

struct AppState {
    entries: Vec<PortEntry>,
    table_state: TableState,
    filter: String,
    filtering: bool,
    show_all: bool,
    confirm_kill: Option<PortEntry>,
    status_message: Option<(String, Instant)>,
    last_refresh: Instant,
}

impl AppState {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            table_state: TableState::default(),
            filter: String::new(),
            filtering: false,
            show_all: false,
            confirm_kill: None,
            status_message: None,
            last_refresh: Instant::now(),
        }
    }

    fn refresh(&mut self) {
        self.entries = ports::all();
        self.last_refresh = Instant::now();
        // Keep selection in bounds
        let filtered_len = self.filtered_entries().len();
        if let Some(selected) = self.table_state.selected() {
            if selected >= filtered_len && filtered_len > 0 {
                self.table_state.select(Some(filtered_len - 1));
            }
        }
    }

    fn filtered_entries(&self) -> Vec<&PortEntry> {
        let home = self.detect_home();
        self.entries
            .iter()
            .filter(|e| {
                if !self.show_all {
                    if let Some(ref h) = home {
                        match &e.cwd {
                            Some(cwd) if cwd.starts_with(h) => {}
                            _ => return false,
                        }
                    }
                }
                if self.filter.is_empty() {
                    return true;
                }
                let query = self.filter.to_lowercase();
                let haystack = format!(
                    "{} {} {} {}",
                    e.cwd.as_deref().unwrap_or(""),
                    e.name,
                    e.args.as_deref().unwrap_or(""),
                    e.listen.join(" ")
                )
                .to_lowercase();
                haystack.contains(&query)
            })
            .collect()
    }

    fn detect_home(&self) -> Option<String> {
        let homes: Vec<String> = self
            .entries
            .iter()
            .filter_map(|e| e.cwd.as_ref())
            .filter_map(|cwd| {
                let re = regex::Regex::new(r"^(/(?:Users|home)/[^/]+)").ok()?;
                re.captures(cwd).map(|c| c[1].to_string())
            })
            .collect();
        if homes.is_empty() {
            return None;
        }
        let mut counts = std::collections::HashMap::new();
        for h in &homes {
            *counts.entry(h.clone()).or_insert(0) += 1;
        }
        counts.into_iter().max_by_key(|(_k, v)| *v).map(|(k, _)| k)
    }

    fn selected_entry(&self) -> Option<&PortEntry> {
        let filtered = self.filtered_entries();
        self.table_state.selected().and_then(|i| filtered.get(i).copied())
    }

    fn set_status(&mut self, msg: String) {
        self.status_message = Some((msg, Instant::now()));
    }
}

enum Action {
    Quit,
    Continue,
}

fn handle_key(key: KeyEvent, state: &mut AppState) -> Action {
    // In confirm kill mode
    if let Some(ref entry) = state.confirm_kill.clone() {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                let pid = entry.pid;
                let result = Command::new("kill").arg(pid.to_string()).status();
                match result {
                    Ok(s) if s.success() => {
                        state.set_status(format!("Killed PID {}", pid));
                        state.refresh();
                    }
                    _ => {
                        state.set_status(format!("Failed to kill PID {}", pid));
                    }
                }
                state.confirm_kill = None;
            }
            _ => {
                state.confirm_kill = None;
                state.set_status("Kill cancelled".to_string());
            }
        }
        return Action::Continue;
    }

    // In filter mode, handle text input
    if state.filtering {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => {
                state.filtering = false;
            }
            KeyCode::Backspace => {
                state.filter.pop();
            }
            KeyCode::Char(c) => {
                state.filter.push(c);
            }
            _ => {}
        }
        return Action::Continue;
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Char('j') | KeyCode::Down => {
            let len = state.filtered_entries().len();
            if len > 0 {
                let i = state.table_state.selected().map_or(0, |i| (i + 1).min(len - 1));
                state.table_state.select(Some(i));
            }
            Action::Continue
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let i = state.table_state.selected().map_or(0, |i| i.saturating_sub(1));
            state.table_state.select(Some(i));
            Action::Continue
        }
        KeyCode::Char('r') => {
            state.refresh();
            state.set_status("Refreshed".to_string());
            Action::Continue
        }
        KeyCode::Char('/') => {
            state.filtering = true;
            Action::Continue
        }
        KeyCode::Char('a') => {
            state.show_all = !state.show_all;
            state.set_status(if state.show_all {
                "Showing all processes".to_string()
            } else {
                "Showing user processes only".to_string()
            });
            Action::Continue
        }
        KeyCode::Char('o') | KeyCode::Enter => {
            if let Some(entry) = state.selected_entry().cloned() {
                if let Some(addr) = entry.listen.first() {
                    let url = format!(
                        "http://{}",
                        addr.replace('*', "localhost")
                    );
                    if open::that(&url).is_ok() {
                        state.set_status(format!("Opened {}", url));
                    } else {
                        state.set_status("Failed to open browser".to_string());
                    }
                }
            }
            Action::Continue
        }
        KeyCode::Char('x') => {
            if let Some(entry) = state.selected_entry().cloned() {
                state.set_status(format!("Kill {} (PID {})? y/N", entry.name, entry.pid));
                state.confirm_kill = Some(entry);
            }
            Action::Continue
        }
        _ => Action::Continue,
    }
}

fn ui(f: &mut ratatui::Frame, state: &mut AppState) {
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(5),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(f.area());

    // Header
    let header_text = if state.filtering {
        format!(" Filter: {}▌", state.filter)
    } else {
        " h41 — Listening Ports".to_string()
    };
    let header = Paragraph::new(Line::from(vec![
        Span::styled(header_text, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]));
    f.render_widget(header, chunks[0]);

    // Table
    let filtered: Vec<PortEntry> = state.filtered_entries().into_iter().cloned().collect();

    if filtered.is_empty() && state.table_state.selected().is_some() {
        state.table_state.select(None);
    } else if !filtered.is_empty() && state.table_state.selected().is_none() {
        state.table_state.select(Some(0));
    }

    let header_cells = ["PID", "Command", "Working Dir", "Addresses"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    let header_row = Row::new(header_cells).height(1);

    let rows: Vec<Row> = filtered
        .iter()
        .map(|entry| {
            let addrs = entry
                .listen
                .iter()
                .map(|a| a.replace('*', "localhost"))
                .collect::<Vec<_>>()
                .join(", ");
            let cmd = entry.name.clone();
            Row::new(vec![
                Cell::from(entry.pid.to_string()),
                Cell::from(cmd),
                Cell::from(entry.cwd.clone().unwrap_or_default()),
                Cell::from(addrs),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(8),
        Constraint::Percentage(25),
        Constraint::Percentage(35),
        Constraint::Percentage(30),
    ];

    let table = Table::new(rows, widths)
        .header(header_row)
        .block(Block::default().borders(Borders::ALL))
        .row_highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");

    f.render_stateful_widget(table, chunks[1], &mut state.table_state);

    // Status
    let status_text = if let Some((ref msg, instant)) = state.status_message {
        if instant.elapsed() < Duration::from_secs(3) {
            msg.clone()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    let status = Paragraph::new(Line::from(Span::styled(
        format!(" {}", status_text),
        Style::default().fg(Color::Green),
    )));
    f.render_widget(status, chunks[2]);

    // Help bar
    let show_all_label = if state.show_all { "user-only" } else { "show-all" };
    let help = Paragraph::new(Line::from(vec![
        Span::styled(" q", Style::default().fg(Color::Yellow)),
        Span::raw(" quit  "),
        Span::styled("r", Style::default().fg(Color::Yellow)),
        Span::raw(" refresh  "),
        Span::styled("/", Style::default().fg(Color::Yellow)),
        Span::raw(" filter  "),
        Span::styled("a", Style::default().fg(Color::Yellow)),
        Span::raw(format!(" {}  ", show_all_label)),
        Span::styled("o", Style::default().fg(Color::Yellow)),
        Span::raw(" open  "),
        Span::styled("x", Style::default().fg(Color::Yellow)),
        Span::raw(" kill"),
    ]));
    f.render_widget(help, chunks[3]);
}
