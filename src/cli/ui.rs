use chrono::{FixedOffset, Local, NaiveTime, TimeZone, Timelike, Utc};
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
    widgets::{
        Block, Borders, Chart, GraphType, HighlightSpacing, List, ListItem, Paragraph,
        canvas::{Canvas, Circle, Line as CanvasLine, Points},
    },
};

use crate::api::*;
use std::io;

pub struct App {
    prayer_times: ParsedPrayerTimesResponse,
    should_quit: bool,
}

impl App {
    fn get_timezone_now(&self) -> NaiveTime {
        let mut timezone = self.prayer_times.location.timezone as i32;

        if self.prayer_times.location.daylight {
            timezone = timezone + 1;
        }

        let hour_secs: i32 = 3600;
        let offset = if timezone >= 0 {
            FixedOffset::east_opt(hour_secs * timezone)
        } else {
            FixedOffset::west_opt(hour_secs * timezone.abs())
        };

        let utc_now = Utc::now().naive_utc();

        let timezone_now = offset.unwrap().from_utc_datetime(&utc_now).time();

        timezone_now
    }

    fn get_next_prayer(&self) -> Option<(Prayer, NaiveTime)> {
        let today = &self.prayer_times.items.get(0);

        if today.is_none() {
            return None;
        }

        let timezone_now = self.get_timezone_now();
        let today = today.unwrap();

        let prayer_map = today.to_hash_map();

        // Check each prayer in order
        for prayer in Prayer::all_prayers() {
            let prayer_time = prayer_map.get(&prayer).unwrap();

            if timezone_now < *prayer_time {
                return Some((prayer, *prayer_time));
            }
        }

        // If no prayer today, return tomorrow's Fajr (or first prayer of next day)
        Some((Prayer::Fajr, today.fajr))
    }

    fn get_countdown_to_next_prayer(&self) -> Option<String> {
        let next_prayer = self.get_next_prayer();
        if next_prayer.is_none() {
            return None;
        }

        let (_, next_prayer_time) = next_prayer.unwrap();

        let timezone_now = self.get_timezone_now();
        // Calculate the difference in seconds
        let now_seconds = timezone_now.num_seconds_from_midnight() as i64;
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

    let qibla_rect = menus_layout[0];

    draw_qibla(frame, &qibla_rect, app);

    let prayers_list_rect = menus_layout[1];

    draw_prayers_list(frame, &prayers_list_rect, app);
}

pub fn draw_header(frame: &mut Frame, rect: &Rect, app: &mut App) {
    let block = Block::default()
        .title("Current Date")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::new().cyan()); // Changed from blue to cyan

    let full_date = app.prayer_times.items[0].date.format("%D").to_string();
    let hour = app.get_timezone_now().format("%-I:%M %p").to_string();
    let location = format!(
        "{} {}",
        app.prayer_times.location.country, app.prayer_times.location.city
    );
    let text = format!("{location} {full_date} {hour}");

    let widget = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(block)
        .style(Style::new().yellow()); // Changed from white to yellow for better visibility

    frame.render_widget(widget, *rect);
}

pub fn draw_prayers_list(frame: &mut Frame, rect: &Rect, app: &mut App) {
    let prayer_list_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(*rect);

    let prayer_items: Vec<ListItem> = Prayer::all_prayers()
        .iter()
        .flat_map(|prayer| {
            let mut style = Style::new().fg(Color::LightCyan).italic();

            let (next_prayer, _) = app.get_next_prayer().unwrap();

            if next_prayer == *prayer {
                style = style
                    .bg(Color::Magenta)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD);
            }

            let prayer_time = app.prayer_times.items[0].get_prayer(prayer);
            let text = Span::default()
                .content(format!(
                    "{} {}",
                    prayer.name(),
                    prayer_time.format("%-I:%M %p").to_string()
                ))
                .style(Style::new().underlined())
                .into_centered_line();

            [
                ListItem::new(""),
                ListItem::new(text).style(style),
                ListItem::new(""),
            ]
        })
        .collect();

    let block = Block::default()
        .title("Prayers Time")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::new().cyan());
    let list: List = List::default()
        .items(prayer_items)
        .block(block)
        .style(Style::new())
        .highlight_spacing(HighlightSpacing::Always);

    let next_prayer_block = Block::default()
        .title("Next Prayer")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::new().cyan());

    let (next_prayer, next_prayer_time) = app.get_next_prayer().unwrap();
    let prayer_count_down = app.get_countdown_to_next_prayer().unwrap();

    let next_prayer_text = format!(
        "{} {} ({})",
        next_prayer.name(),
        next_prayer_time.format("%-I:%M %p").to_string(),
        prayer_count_down
    );

    let next_prayer_span = Paragraph::new(next_prayer_text)
        .alignment(Alignment::Center)
        .block(next_prayer_block)
        .style(Style::new().yellow().add_modifier(Modifier::BOLD));

    frame.render_widget(list, prayer_list_layout[0]);
    frame.render_widget(next_prayer_span, prayer_list_layout[1]);
}

fn draw_qibla(frame: &mut Frame, rect: &Rect, app: &mut App) {
    let c_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ])
        .split(*rect);

    let d_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(c_layout[1]);

    let block = Block::new()
        .title("Qibla")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::new().cyan())
        .style(Style::new());

    let qibla_angle = 90.0f64.to_radians() - app.prayer_times.location.qibla_direction.to_radians();

    let x_bounds = [0.0, 100.0];
    let y_bounds = [0.0, 100.0];

    let canvas = Canvas::default()
        .marker(ratatui::symbols::Marker::HalfBlock)
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .paint(|ctx| {
            let circle = Circle {
                color: Color::Magenta,
                x: 50.0,
                y: 50.0,
                radius: 30.0,
            };

            let line = CanvasLine {
                color: Color::Yellow,
                x1: circle.x,
                y1: circle.y,
                x2: circle.x + 3.0 * circle.radius / 4.0 * qibla_angle.cos(),
                y2: circle.y + 3.0 * circle.radius / 4.0 * qibla_angle.sin(),
            };

            ctx.draw(&circle);
            ctx.layer();
            ctx.draw(&line);
        });

    frame.render_widget(block, *rect);
    frame.render_widget(canvas, d_layout[1]);
}
