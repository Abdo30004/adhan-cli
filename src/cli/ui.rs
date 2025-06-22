use chrono::{Local, NaiveTime, Timelike};
use ratatui::{
    Frame, Terminal,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style, Stylize},
    symbols::block,
    text::{Line, Span},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, Paragraph},
};

use crate::api::*;
use std::io;

pub struct App {
    prayer_times: ParsedPrayerTimesResponse,
    should_quit: bool,
}

impl App {
    fn get_next_prayer(&self) -> Option<(Prayer, NaiveTime)> {
        let now = Local::now().time();
        let today = &self.prayer_times.items[0];
        let prayer_map = today.to_hash_map();

        // Check each prayer in order
        for prayer in Prayer::all_prayers() {
            let prayer_time = prayer_map.get(&prayer).unwrap();

            if now < *prayer_time {
                return Some((prayer, *prayer_time));
            }
        }

        // If no prayer today, return tomorrow's Fajr (or first prayer of next day)
        Some((Prayer::Fajr, today.fajr))
    }

    fn get_countdown_to_next_prayer(&self) -> Option<String> {
        if let Some((_, next_prayer_time)) = self.get_next_prayer() {
            let now = Local::now().time();
            // Calculate the difference in seconds
            let now_seconds = now.num_seconds_from_midnight() as i64;
            let prayer_seconds = next_prayer_time.num_seconds_from_midnight() as i64;

            let diff_seconds = if prayer_seconds >= now_seconds {
                prayer_seconds - now_seconds
            } else {
                // Next day's prayer (add 24 hours)
                (24 * 60 * 60) - now_seconds + prayer_seconds
            };

            let hours = diff_seconds / 3600;
            let minutes = (diff_seconds % 3600) / 60;
            let seconds = diff_seconds % 60;

            if hours > 0 {
                Some(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
            } else {
                Some(format!("{:02}:{:02}", minutes, seconds))
            }
        } else {
            None
        }
    }
}

pub fn entry(data: ParsedPrayerTimesResponse) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App {
        prayer_times: data,
        should_quit: false,
    };

    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

pub fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        ui(terminal, &mut app)?;

        if event::poll(std::time::Duration::from_millis(1000))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

pub fn ui<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> std::io::Result<()> {
    terminal.draw(|frame| draw_main(frame, app))?;

    Ok(())
}

pub fn draw_main(frame: &mut Frame, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(frame.area());

    let header_rect = main_layout[0];

    draw_header(frame, &header_rect, app);

    let menus_rect = main_layout[1];

    let menus_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(menus_rect);

    let prayers_list_rect = menus_layout[1];

    draw_payers_list(frame, &prayers_list_rect, app);
}

pub fn draw_header(frame: &mut Frame, rect: &Rect, app: &mut App) {
    let block = Block::default()
        .title("Current Date")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::new().blue());

    let widget = Paragraph::new(app.prayer_times.items[0].date.format("%D").to_string())
        .alignment(Alignment::Center)
        .block(block)
        .style(Style::new().white());

    frame.render_widget(widget, *rect);
}

pub fn draw_payers_list(frame: &mut Frame, rect: &Rect, app: &mut App) {
    let prayer_items: Vec<ListItem> = Prayer::all_prayers()
        .iter()
        .flat_map(|prayer| {
            let mut style = Style::new().white().italic().fg(Color::White);

            if app.get_next_prayer().unwrap().0 == *prayer {
                style = style.bg(Color::LightMagenta);
            }

            let prayer_time = app.prayer_times.items[0].get_prayer(prayer);
            let text = Span::default()
                .content(format!(
                    "{} {}",
                    prayer.name(),
                    prayer_time.format("%H:%M").to_string()
                ))
                .into_centered_line();

            [ListItem::new(text).style(style), ListItem::new("")]
        })
        .collect();

    let block = Block::default()
        .title("Prayers Time")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::new().blue());

    let list: List = List::default()
        .items(prayer_items)
        .block(block)
        .style(Style::new())
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_widget(list, *rect);
}
