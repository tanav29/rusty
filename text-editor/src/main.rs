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
                KeyCode::Esc => {
                    editor.mode = Mode::Normal;
                    editor.command = String::from("");
                }
                KeyCode::Char(c) => {
                    if editor.mode == Mode::Insert {
                        editor.insert_character(c);
                    } else if editor.mode == Mode::Normal {
                        match c {
                            'j' => editor.move_down(),
                            'k' => editor.move_up(),
                            'l' => editor.move_x(1),
                            'h' => editor.move_x(-1),
                            'i' => editor.mode = Mode::Insert,
                            ':' => editor.mode = Mode::Command,
                            '/' => editor.mode = Mode::Find,
                            _ => {}
                        }
                    } else if editor.mode == Mode::Command || editor.mode == Mode::Find {
                        editor.command.push(c);
                    }
                }
                KeyCode::Enter => {
                    if editor.mode == Mode::Insert {
                        editor.insert_character('\n');
                        editor.posy += 1;
                        editor.posx = 0;
                    }
                }
                KeyCode::Delete => {
                    if editor.mode != Mode::Command {
                        editor.delete();
                    }
                }
                KeyCode::Backspace => {
                    if editor.mode == Mode::Insert {
                        editor.backspace();
                    } else if editor.mode == Mode::Command || editor.mode == Mode::Find {
                        editor.command.pop();
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

    let cmd = match editor.mode {
        Mode::Normal => "Normal".to_string(),
        Mode::Insert => "Insert".to_string(),
        Mode::Command => format!(":{}", editor.command),
        Mode::Find => format!("/{}", editor.command),
    };

    let status = format!(" {} | {}:{} ", cmd, editor.posy + 1, editor.posx + 1);

    let status_bar = Paragraph::new(status);

    frame.render_widget(status_bar, chunks[1]);
}
