use std::{error::Error, io};

use app::{AppMode, TwitchMessage};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::app::App;
mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let mut app = App::new();

    enable_raw_mode()?;
    let mut stderr = io::stderr();

    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let _res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.app_mode {
                AppMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Char('e') | KeyCode::Tab => {
                        app.app_mode = AppMode::ChatEnter;
                    }

                    _ => (),
                },
                AppMode::ChatEnter if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        //TODO: Implement sending message
                        app.messages.insert(0,TwitchMessage {
                            user: String::from("test"),
                            message: app.chat_input.clone(),
                        });
                        app.chat_input = String::new();
                    }
                    KeyCode::Backspace => {
                        app.chat_input.pop();
                    }
                    KeyCode::Esc | KeyCode::Tab => {
                        app.app_mode = AppMode::Normal;
                    }
                    KeyCode::Char(value) => {
                        app.chat_input.push(value);
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
