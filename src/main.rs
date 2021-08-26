use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde_json::from_str;
use std::{
    error::Error,
    fs, io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use transmutation::{replace_clipboard, Recipe};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, BorderType, Borders},
    Terminal,
};

const CONFIG_PATH: &str = "./config.json";

enum Event<I> {
    Input(I),
    Tick,
}

fn read_config() -> Result<Vec<Recipe>, io::Error> {
    let config_content = fs::read_to_string(CONFIG_PATH)?;
    let parsed: Vec<Recipe> = from_str(&config_content)?;
    Ok(parsed)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read in the config
    let recipes = read_config()?;

    // Set up the terminal
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let mut should_quit = false;

    // Setup input handling
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(250);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            // Wrapping block for a group
            // Just draw the block and the group on the same area and build the group
            // with at least a margin of 1
            let size = f.size();

            // Surrounding block
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Transmutation")
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);
        })?;
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    should_quit = true;
                    break;
                }
                _ => {}
            },
            Event::Tick => {
                replace_clipboard(recipes.clone());
            }
        }
        if should_quit {
            break;
        }
    }
    Ok(())
}
