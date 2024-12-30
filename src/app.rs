use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode};
use crossterm::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};
use ratatui::DefaultTerminal;
use ratatui::layout::{Constraint, Layout};

use crate::event::AppEvent;

pub struct App {
    log: String,
    scroll_state: u16,
    quit: bool,
}

impl App {
    pub fn new(log: &str) -> Self {
        Self {
            log: log.to_string(),
            scroll_state: 0,
            quit: false,
        }
    }

    pub fn handle_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Quit => self.quit = true,
            _ => {}
        }
    }

    pub fn capture_event(&self) -> Result<AppEvent> {
        match event::read()? {
            Event::Key(e) => match e.code {
                KeyCode::Char('q') => Ok(AppEvent::Quit),
                _ => Ok(AppEvent::Nothing),
            },
            _ => Ok(AppEvent::Nothing)
        }
    }

    pub fn draw(&self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.draw(|f| {
            let area = f.area();
            let layout = Layout::vertical([
                Constraint::Length(1),
                Constraint::Fill(0),
                Constraint::Length(1),
            ]).split(area);

            let header = Block::default()
                .title("idyllic")
                .bg(Color::DarkGrey)
                .fg(Color::Black);
            let footer = Block::default()
                .title("footer")
                .bg(Color::DarkGrey)
                .fg(Color::Black);
            let logs = Paragraph::new(self.log.clone())
                .scroll((self.scroll_state, 0));

            f.render_widget(header, layout[0]);
            f.render_widget(logs, layout[1]);
            f.render_widget(footer, layout[2]);
        }).context("failed to draw frame")?;

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.handle_event(self.capture_event()?);

        Ok(())
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.quit {
            self.draw(terminal)?;
            self.update()?;
        }

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
