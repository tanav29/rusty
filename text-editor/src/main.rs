use crate::editor::*;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::backend::CrosstermBackend;
use ratatui::{Frame, Terminal};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::io::{self, Result, Stdout};

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
        terminal.draw(|f| ui(f, &editor))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Esc => editor.mode = Mode::Normal,
                KeyCode::Char(c) => {
                    if editor.mode == Mode::Insert {
                        editor.insert_character(c);
                    } else {
                        match c {
                            'j' => editor.move_down(1),
                            'k' => editor.move_up(1),
                            'l' => editor.move_right(1),
                            'h' => editor.move_left(1),
                            'i' => editor.mode = Mode::Insert,
                            _ => {}
                        }
                    }
                }
                KeyCode::Enter => {
                    if editor.mode == Mode::Insert {
                        editor.insert_character('\n');
                    }
                }
                KeyCode::Delete => {
                    editor.delete();
                }
                KeyCode::Backspace => {
                    if editor.mode == Mode::Insert {
                        editor.backspace();
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, editor: &Editor) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.size());
    let paragraph =
        Paragraph::new(editor.examine_string()).block(Block::default().borders(Borders::NONE));
    frame.render_widget(paragraph, chunks[0]);

    let status = format!(
        " {:?} | {}:{} ",
        editor.mode,
        editor.posy + 1,
        editor.posx + 1
    );

    let status_bar = Paragraph::new(status);

    frame.render_widget(status_bar, chunks[1]);
}
