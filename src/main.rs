use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

mod app;
mod system_info;
mod ui;

use app::{App, AppResult};

#[derive(Parser)]
#[command(name = "system_monitor")]
#[command(about = "Interactive Terminal System Monitor")]
struct Cli {
    /// Update interval in milliseconds
    #[arg(short, long, default_value_t = 1000)]
    interval: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let tick_rate = Duration::from_millis(cli.interval);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate).await;

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

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> AppResult<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('h') | KeyCode::Left => app.previous_tab(),
                        KeyCode::Char('l') | KeyCode::Right => app.next_tab(),
                        KeyCode::Char('j') | KeyCode::Down => app.next_process(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous_process(),
                        KeyCode::Char('r') => app.refresh(),
                        KeyCode::Char('s') => app.toggle_sort(),
                        KeyCode::Delete => app.kill_selected_process(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update().await;
            last_tick = Instant::now();
        }
    }
}