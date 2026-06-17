use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Terminal,
};
use std::io;
use std::thread::{self, JoinHandle};
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
    state.start_refresh(false);

    loop {
        state.finish_refresh();
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
        if !state.is_refreshing() && state.last_refresh.elapsed() > Duration::from_secs(30) {
            state.start_refresh(false);
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
    refresh_started: Option<Instant>,
    refresh_handle: Option<JoinHandle<Vec<PortEntry>>>,
    show_refreshed_status: bool,
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
            refresh_started: None,
            refresh_handle: None,
            show_refreshed_status: false,
        }
    }

    fn start_refresh(&mut self, show_refreshed_status: bool) {
        if self.refresh_handle.is_some() {
            return;
        }

        self.refresh_started = Some(Instant::now());
        self.refresh_handle = Some(thread::spawn(ports::all));
        self.show_refreshed_status = show_refreshed_status;
    }

    fn finish_refresh(&mut self) {
        if !self
            .refresh_handle
            .as_ref()
            .is_some_and(|handle| handle.is_finished())
        {
            return;
        }

        if let Some(handle) = self.refresh_handle.take() {
            if let Ok(entries) = handle.join() {
                self.entries = entries;
            } else {
                self.set_status("Refresh failed".to_string());
            }
        }

        let show_refreshed_status = self.show_refreshed_status;
        self.refresh_started = None;
        self.show_refreshed_status = false;
        self.last_refresh = Instant::now();
        if show_refreshed_status {
            self.set_status("Refreshed".to_string());
        }

        // Keep selection in bounds
        let filtered_len = self.filtered_entries().len();
        if let Some(selected) = self.table_state.selected() {
            if selected >= filtered_len && filtered_len > 0 {
                self.table_state.select(Some(filtered_len - 1));
            }
        }
    }

    fn is_refreshing(&self) -> bool {
        self.refresh_handle.is_some()
    }

    fn spinner_frame(&self) -> &'static str {
        const FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let elapsed = self
            .refresh_started
            .map(|started| started.elapsed())
            .unwrap_or_default();
        let index = (elapsed.as_millis() / 100) as usize % FRAMES.len();
        FRAMES[index]
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
        let cwds = self.entries.iter().filter_map(|e| e.cwd.clone());
        ports::detect_home_prefix(cwds)
    }

    fn selected_entry(&self) -> Option<&PortEntry> {
        let filtered = self.filtered_entries();
        self.table_state
            .selected()
            .and_then(|i| filtered.get(i).copied())
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
                if ports::kill_pid(pid) {
                    state.set_status(format!("Killed PID {}", pid));
                    state.start_refresh(false);
                } else {
                    state.set_status(format!("Failed to kill PID {}", pid));
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
                let i = state
                    .table_state
                    .selected()
                    .map_or(0, |i| (i + 1).min(len - 1));
                state.table_state.select(Some(i));
            }
            Action::Continue
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let i = state
                .table_state
                .selected()
                .map_or(0, |i| i.saturating_sub(1));
            state.table_state.select(Some(i));
            Action::Continue
        }
        KeyCode::Char('r') => {
            state.start_refresh(true);
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
                    let url = format!("http://{}", addr.replace('*', "localhost"));
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
    let header = Paragraph::new(Line::from(vec![Span::styled(
        header_text,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]));
    let header_chunks =
        Layout::horizontal([Constraint::Min(0), Constraint::Length(14)]).split(chunks[0]);
    f.render_widget(header, header_chunks[0]);
    if state.is_refreshing() && !state.entries.is_empty() {
        let refreshing = Paragraph::new(Line::from(vec![
            Span::styled(state.spinner_frame(), Style::default().fg(Color::Cyan)),
            Span::raw(" refreshing"),
        ]))
        .alignment(Alignment::Right);
        f.render_widget(refreshing, header_chunks[1]);
    }

    // Table
    let filtered: Vec<PortEntry> = state.filtered_entries().into_iter().cloned().collect();

    if filtered.is_empty() && state.table_state.selected().is_some() {
        state.table_state.select(None);
    } else if !filtered.is_empty() && state.table_state.selected().is_none() {
        state.table_state.select(Some(0));
    }

    if state.entries.is_empty() && state.is_refreshing() {
        let block = Block::default().borders(Borders::ALL);
        let loader_area = block.inner(chunks[1]);
        f.render_widget(block, chunks[1]);

        let centered = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(loader_area)[1];
        let loader = Paragraph::new(Line::from(vec![
            Span::styled(state.spinner_frame(), Style::default().fg(Color::Cyan)),
            Span::raw(" scanning ports"),
        ]))
        .alignment(Alignment::Center);
        f.render_widget(loader, centered);
    } else {
        let header_cells = ["PID", "Command", "Working Dir", "Addresses"]
            .iter()
            .map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            });
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
            .row_highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        f.render_stateful_widget(table, chunks[1], &mut state.table_state);
    }

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
    let show_all_label = if state.show_all {
        "user-only"
    } else {
        "show-all"
    };
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
