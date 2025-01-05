use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Frame, text::Text};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser, Debug)]
struct Args {
    /// Path to directory
    #[arg(
        default_value = std::env::current_dir().expect("Can't get current directory path").into_os_string(),
        value_hint = clap::ValueHint::DirPath
    )]
    path: PathBuf,
}

struct App {
    path: PathBuf,
    running: bool,
}

impl App {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            running: true,
        }
    }

    fn on_key(&mut self, key: char) {
        match key {
            'q' | 'Q' => self.running = false,
            _ => {}
        }
    }
    fn on_down(&self) {}
    fn on_left(&self) {}
    fn on_right(&self) {}
    fn on_up(&self) {}
    fn shuld_close(&self) -> bool {
        !self.running
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut app = App::new(args.path);
    let mut terminal = ratatui::init();
    while !app.shuld_close() {
        terminal.draw(draw)?;
        if event::poll(Duration::from_secs(2))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => app.on_key(c),
                        KeyCode::Down | KeyCode::Char('j') => app.on_down(),
                        KeyCode::Left | KeyCode::Char('h') => app.on_left(),
                        KeyCode::Right | KeyCode::Char('l') => app.on_right(),
                        KeyCode::Up | KeyCode::Char('k') => app.on_up(),
                        _ => {}
                    }
                }
            }
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}
