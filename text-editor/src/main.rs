use crate::editor::Editor;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::backend::CrosstermBackend;
use ratatui::{widgets::Paragraph, Terminal};
use std::io::{self, Result, Stdout};
use std::time::Duration;

mod editor;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut editor = Editor::new();
    loop {
        terminal.draw(|frame| {
            let paragraph = Paragraph::new(editor.examine_string());
            frame.render_widget(paragraph, frame.size());
        })?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => editor.move_left_one(),
                    KeyCode::Right => editor.move_right_one(),
                    KeyCode::Char(c) => editor.insert_character(c),
                    KeyCode::Backspace => {
                        editor.backspace();
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
