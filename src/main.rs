use std::{error::Error, io};

use crossterm::{
    event::{EnableMouseCapture, self, Event, KeyCode, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::{CrosstermBackend, Backend}, Terminal};

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

    let _res = run_app(&mut terminal,&mut app);
    disable_raw_mode()?;
    execute!(terminal.backend_mut()
        ,LeaveAlternateScreen
        ,DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<bool>{
    loop{

        terminal.draw(|f| ui::ui(f,app))?;

        if let Event::Key(key) = event::read()?{
            if key.kind == event::KeyEventKind::Release{
                continue;
            }

            match key.code{
                KeyCode::Char('q') => {
                    return Ok(true);
                }
                KeyCode::Char('e') => {
                    app.messages.push(app::TwitchMessage { user: "Bingus".to_string(), message: "this is my test message!".to_string()})
                }
                _ => ()
            }
        }
    }
}
