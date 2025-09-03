use anyhow::Result;
use chrono::{DateTime, Utc};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use git2::{BranchType, Repository};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
};
use std::{io, process::Command};

#[derive(Clone, Debug)]
struct GitBranch {
    name: String,
    is_current: bool,
    last_commit_time: DateTime<Utc>,
}

struct App {
    branches: Vec<GitBranch>,
    filtered_branches: Vec<usize>,
    list_state: ListState,
    filter: String,
}

impl App {
    fn new() -> Result<App> {
        let mut app = App {
            branches: Vec::new(),
            filtered_branches: Vec::new(),
            list_state: ListState::default(),
            filter: String::new(),
        };
        app.fetch_branches()?;
        app.update_filter();
        Ok(app)
    }

    fn fetch_branches(&mut self) -> Result<()> {
        let repo = Repository::open(".")?;
        let mut branches = Vec::new();

        let branch_iter = repo.branches(Some(BranchType::Local))?;
        for branch_result in branch_iter {
            let (branch, _) = branch_result?;
            if let Some(name) = branch.name()? {
                let is_current = branch.is_head();

                let last_commit_time = {
                    let reference = branch.get();
                    if let Some(target) = reference.target() {
                        if let Ok(commit) = repo.find_commit(target) {
                            let timestamp = commit.time();
                            DateTime::from_timestamp(timestamp.seconds(), 0)
                                .unwrap_or_else(|| Utc::now())
                        } else {
                            Utc::now()
                        }
                    } else {
                        Utc::now()
                    }
                };

                branches.push(GitBranch {
                    name: name.to_string(),
                    is_current,
                    last_commit_time,
                });
            }
        }

        branches.sort_by(|a, b| b.last_commit_time.cmp(&a.last_commit_time));
        self.branches = branches.into_iter().take(10).collect();
        Ok(())
    }

    fn update_filter(&mut self) {
        if self.filter.is_empty() {
            self.filtered_branches = (0..self.branches.len()).collect();
        } else {
            self.filtered_branches = self
                .branches
                .iter()
                .enumerate()
                .filter(|(_, branch)| {
                    branch
                        .name
                        .to_lowercase()
                        .contains(&self.filter.to_lowercase())
                })
                .map(|(i, _)| i)
                .collect();
        }

        if !self.filtered_branches.is_empty() {
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(None);
        }
    }

    fn next(&mut self) {
        if self.filtered_branches.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.filtered_branches.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.filtered_branches.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_branches.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn checkout_selected(&self) -> Result<()> {
        if let Some(selected) = self.list_state.selected() {
            if let Some(&branch_idx) = self.filtered_branches.get(selected) {
                if let Some(branch) = self.branches.get(branch_idx) {
                    if !branch.is_current {
                        let output = Command::new("git")
                            .args(["checkout", &branch.name])
                            .output()?;

                        if !output.status.success() {
                            return Err(anyhow::anyhow!(
                                "Failed to checkout branch: {}",
                                String::from_utf8_lossy(&output.stderr)
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn add_char(&mut self, c: char) {
        self.filter.push(c);
        self.update_filter();
    }

    fn remove_char(&mut self) {
        self.filter.pop();
        self.update_filter();
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let area = f.area();

    // Only show filter if there's text
    let list_area = if app.filter.is_empty() {
        area
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
            .split(area);

        let filter_text = format!("Filter: {}", app.filter);
        let filter_paragraph = Paragraph::new(filter_text).style(Style::default().fg(Color::Cyan));
        f.render_widget(filter_paragraph, chunks[0]);

        chunks[1]
    };

    let items: Vec<ListItem> = app
        .filtered_branches
        .iter()
        .enumerate()
        .map(|(idx, &i)| {
            let branch = &app.branches[i];
            let is_selected = app.list_state.selected() == Some(idx);

            let mut spans = vec![];

            // Selection indicator (like gum)
            if is_selected {
                spans.push(Span::styled("❯ ", Style::default().fg(Color::Magenta)));
            } else {
                spans.push(Span::raw("  "));
            }

            // Current branch indicator
            if branch.is_current {
                spans.push(Span::styled("● ", Style::default().fg(Color::Green)));
            } else {
                spans.push(Span::raw("  "));
            }

            // Branch name
            let name_style = if is_selected {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else if branch.is_current {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Gray)
            };

            spans.push(Span::styled(branch.name.clone(), name_style));

            // Time ago (more subtle)
            let time_ago = {
                let now = Utc::now();
                let duration = now.signed_duration_since(branch.last_commit_time);

                if duration.num_days() > 0 {
                    format!(" ({}d)", duration.num_days())
                } else if duration.num_hours() > 0 {
                    format!(" ({}h)", duration.num_hours())
                } else {
                    format!(" ({}m)", duration.num_minutes().max(1))
                }
            };

            spans.push(Span::styled(time_ago, Style::default().fg(Color::DarkGray)));

            ListItem::new(Line::from(spans))
        })
        .collect();

    // Clean list without borders
    let list = List::new(items)
        .highlight_style(Style::default()) // No background highlight
        .highlight_symbol(""); // No symbol since we handle it manually

    f.render_stateful_widget(list, list_area, &mut app.list_state);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Enter => {
                        app.checkout_selected()?;
                        return Ok(());
                    }
                    KeyCode::Backspace => app.remove_char(),
                    KeyCode::Char(c) => app.add_char(c),
                    _ => {}
                }
            }
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new()?;
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
