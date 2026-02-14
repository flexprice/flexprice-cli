use std::io;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph, Wrap, Sparkline},
    Frame, Terminal,
};

use crate::api::client::ApiClient;
use crate::config::Credentials;
use super::theme::Theme;

const TABS: &[&str] = &[
    "Customers",
    "Plans",
    "Subscriptions",
    "Invoices",
    "Meters",
    "Wallets",
    "Features",
];

const TAB_ENDPOINTS: &[&str] = &[
    "/v1/customers",
    "/v1/plans",
    "/v1/subscriptions",
    "/v1/invoices",
    "/v1/meters",
    "/v1/wallets",
    "/v1/features",
];

pub struct App {
    client: ApiClient,
    creds: Credentials,
    active_tab: usize,
    list_state: ListState,
    data_items: Vec<String>,
    detail_text: String,
    loading: bool,
    error: Option<String>,
    should_quit: bool,
    sparkline_data: Vec<u64>,
}

impl App {
    pub fn new(creds: Credentials) -> Result<Self> {
        let client = ApiClient::new(creds.clone())?;
        let mut s = Self {
            client,
            creds,
            active_tab: 0,
            list_state: ListState::default(),
            data_items: vec![],
            detail_text: String::new(),
            loading: false,
            error: None,
            should_quit: false,
            sparkline_data: vec![3, 7, 2, 9, 5, 12, 8, 4, 11, 6, 14, 3, 8, 10, 5],
        };
        s.list_state.select(Some(0));
        Ok(s)
    }

    fn next_tab(&mut self) {
        self.active_tab = (self.active_tab + 1) % TABS.len();
        self.data_items.clear();
        self.detail_text.clear();
        self.error = None;
        self.list_state.select(Some(0));
    }

    fn prev_tab(&mut self) {
        self.active_tab = if self.active_tab == 0 { TABS.len() - 1 } else { self.active_tab - 1 };
        self.data_items.clear();
        self.detail_text.clear();
        self.error = None;
        self.list_state.select(Some(0));
    }

    fn next_item(&mut self) {
        if self.data_items.is_empty() { return; }
        let i = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some((i + 1) % self.data_items.len()));
    }

    fn prev_item(&mut self) {
        if self.data_items.is_empty() { return; }
        let i = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some(if i == 0 { self.data_items.len() - 1 } else { i - 1 }));
    }
}

pub async fn run(creds: Credentials) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(creds)?;

    // Initial data load
    load_data(&mut app).await;

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Tab | KeyCode::Char('l') => {
                        app.next_tab();
                        load_data(&mut app).await;
                    }
                    KeyCode::BackTab | KeyCode::Char('h') => {
                        app.prev_tab();
                        load_data(&mut app).await;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.next_item();
                        update_detail(&mut app);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.prev_item();
                        update_detail(&mut app);
                    }
                    KeyCode::Char('r') => {
                        load_data(&mut app).await;
                    }
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

async fn load_data(app: &mut App) {
    app.loading = true;
    app.error = None;

    let endpoint = TAB_ENDPOINTS[app.active_tab];
    match app.client.get_text(endpoint).await {
        Ok(body) => {
            // Parse as JSON, extract items
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                if let Some(items) = json.get("items").and_then(|v| v.as_array()) {
                    app.data_items = items.iter().map(|item| {
                        let id = item.get("id").and_then(|v| v.as_str()).unwrap_or("?");
                        let name = item.get("name")
                            .or_else(|| item.get("email"))
                            .or_else(|| item.get("event_name"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("-");
                        let status = item.get("status")
                            .or_else(|| item.get("subscription_status"))
                            .or_else(|| item.get("invoice_status"))
                            .or_else(|| item.get("wallet_status"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        if status.is_empty() {
                            format!("{}  {}", id, name)
                        } else {
                            format!("{}  {}  [{}]", id, name, status)
                        }
                    }).collect();
                    app.detail_text = serde_json::to_string_pretty(&json).unwrap_or_default();
                } else {
                    app.data_items = vec!["(no items)".to_string()];
                    app.detail_text = serde_json::to_string_pretty(&json).unwrap_or(body);
                }
            } else {
                app.data_items = vec!["(raw response)".to_string()];
                app.detail_text = body;
            }
        }
        Err(e) => {
            app.error = Some(format!("{}", e));
            app.data_items.clear();
            app.detail_text.clear();
        }
    }
    app.loading = false;
    app.list_state.select(Some(0));
    update_detail(app);
}

fn update_detail(app: &mut App) {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&app.detail_text) {
        if let Some(items) = json.get("items").and_then(|v| v.as_array()) {
            let idx = app.list_state.selected().unwrap_or(0);
            if let Some(item) = items.get(idx) {
                app.detail_text = serde_json::to_string_pretty(item).unwrap_or_default();
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Background
    f.render_widget(Block::default().style(Style::default().bg(Theme::BG)), size);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Header
            Constraint::Min(10),   // Body
            Constraint::Length(3), // Footer
        ])
        .split(size);

    render_header(f, main_layout[0], app);
    render_body(f, main_layout[1], app);
    render_footer(f, main_layout[2], app);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(30), Constraint::Length(40)])
        .split(area);

    // Logo
    let logo_text = vec![
        Line::from(vec![
            Span::styled("  ⚡ ", Style::default().fg(Theme::WARNING)),
            Span::styled("FlexPrice", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
            Span::styled(" Dashboard", Style::default().fg(Theme::TEXT_DIM)),
        ]),
        Line::from(vec![
            Span::styled("     Usage-based billing, visualized.", Style::default().fg(Theme::TEXT_MUTED)),
        ]),
    ];
    let logo = Paragraph::new(logo_text)
        .block(Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Theme::BORDER))
            .padding(Padding::new(0, 0, 1, 0))
        );
    f.render_widget(logo, header_layout[0]);

    // Connection info
    let info_lines = vec![
        Line::from(vec![
            Span::styled("  API: ", Style::default().fg(Theme::TEXT_DIM)),
            Span::styled(&app.creds.api_url, Style::default().fg(Theme::ACCENT)),
        ]),
        Line::from(vec![
            Span::styled("  Auth: ", Style::default().fg(Theme::TEXT_DIM)),
            Span::styled(
                if app.creds.api_key.is_some() { "API Key" } else { "JWT" },
                Style::default().fg(Theme::INFO)
            ),
        ]),
    ];
    let info = Paragraph::new(info_lines)
        .block(Block::default()
            .borders(Borders::BOTTOM | Borders::LEFT)
            .border_style(Style::default().fg(Theme::BORDER))
            .padding(Padding::new(0, 0, 1, 0))
        );
    f.render_widget(info, header_layout[1]);
}

fn render_body(f: &mut Frame, area: Rect, app: &mut App) {
    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(22), // Sidebar
            Constraint::Min(30),   // Resource list
            Constraint::Min(40),   // Detail panel
        ])
        .split(area);

    // Sidebar — tabs
    let tab_items: Vec<ListItem> = TABS.iter().enumerate().map(|(i, name)| {
        let style = if i == app.active_tab {
            Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Theme::TEXT_DIM)
        };
        let prefix = if i == app.active_tab { " ▸ " } else { "   " };
        ListItem::new(Line::from(vec![
            Span::styled(prefix, Style::default().fg(Theme::PRIMARY)),
            Span::styled(*name, style),
        ]))
    }).collect();

    let sidebar = List::new(tab_items)
        .block(Block::default()
            .title(Span::styled(" Navigate ", Style::default().fg(Theme::TEXT_DIM)))
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(Theme::BORDER))
            .padding(Padding::new(1, 1, 1, 0))
        );
    f.render_widget(sidebar, body_layout[0]);

    // Resource list
    if app.loading {
        let loading = Paragraph::new("  ⏳ Loading...")
            .style(Style::default().fg(Theme::WARNING))
            .block(Block::default()
                .title(Span::styled(format!(" {} ", TABS[app.active_tab]), Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)))
                .borders(Borders::RIGHT)
                .border_style(Style::default().fg(Theme::BORDER))
                .padding(Padding::new(1, 1, 1, 0))
            );
        f.render_widget(loading, body_layout[1]);
    } else if let Some(ref err) = app.error {
        let error_text = Paragraph::new(format!("  ✗ {}", err))
            .style(Style::default().fg(Theme::ERROR))
            .wrap(Wrap { trim: true })
            .block(Block::default()
                .title(Span::styled(format!(" {} ", TABS[app.active_tab]), Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)))
                .borders(Borders::RIGHT)
                .border_style(Style::default().fg(Theme::BORDER))
                .padding(Padding::new(1, 1, 1, 0))
            );
        f.render_widget(error_text, body_layout[1]);
    } else {
        let items: Vec<ListItem> = app.data_items.iter().map(|item| {
            ListItem::new(Line::from(Span::styled(format!(" {}", item), Style::default().fg(Theme::TEXT))))
        }).collect();

        let list = List::new(items)
            .highlight_style(Style::default().fg(Theme::PRIMARY).bg(Theme::SURFACE_HOVER).add_modifier(Modifier::BOLD))
            .highlight_symbol("▸ ")
            .block(Block::default()
                .title(Span::styled(format!(" {} ({}) ", TABS[app.active_tab], app.data_items.len()), Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)))
                .borders(Borders::RIGHT)
                .border_style(Style::default().fg(Theme::BORDER))
                .padding(Padding::new(0, 0, 0, 0))
            );
        f.render_stateful_widget(list, body_layout[1], &mut app.list_state);
    }

    // Detail panel
    let detail_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(5)])
        .split(body_layout[2]);

    let detail = Paragraph::new(Text::from(app.detail_text.clone()))
        .style(Style::default().fg(Theme::TEXT_DIM))
        .wrap(Wrap { trim: false })
        .block(Block::default()
            .title(Span::styled(" Detail ", Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Theme::BORDER))
            .padding(Padding::new(1, 1, 0, 0))
        );
    f.render_widget(detail, detail_layout[0]);

    // Mini sparkline
    let sparkline = Sparkline::default()
        .block(Block::default()
            .title(Span::styled(" Activity ", Style::default().fg(Theme::INFO)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Theme::BORDER))
        )
        .data(&app.sparkline_data)
        .style(Style::default().fg(Theme::ACCENT));
    f.render_widget(sparkline, detail_layout[1]);
}

fn render_footer(f: &mut Frame, area: Rect, _app: &App) {
    let shortcuts = vec![
        Span::styled("  ←/→ Tab", Style::default().fg(Theme::PRIMARY)),
        Span::styled("  │  ", Style::default().fg(Theme::BORDER)),
        Span::styled("↑/↓ Navigate", Style::default().fg(Theme::TEXT_DIM)),
        Span::styled("  │  ", Style::default().fg(Theme::BORDER)),
        Span::styled("r Refresh", Style::default().fg(Theme::ACCENT)),
        Span::styled("  │  ", Style::default().fg(Theme::BORDER)),
        Span::styled("q Quit", Style::default().fg(Theme::ERROR)),
    ];

    let footer = Paragraph::new(Line::from(shortcuts))
        .block(Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Theme::BORDER))
            .padding(Padding::new(0, 0, 0, 0))
        );
    f.render_widget(footer, area);
}
