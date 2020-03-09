use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Modifier};
use crossterm::event::{Event, KeyCode, read};
use std::fs::File;

#[derive(PartialEq, Eq)]
enum VimMode {
    Command,
    Edit,
    FileNameInsert,
}

struct VimState {
    pub mode: VimMode,
    pub main_text: String,
    pub debug_text: String,
    pub file_name: String,
}

pub fn start() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = VimState {
        mode: VimMode::Command,
        main_text: "".to_string(),
        debug_text: "".to_string(),
        file_name: "".to_string(),
    };

    terminal.clear().unwrap();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(80),
                        Constraint::Percentage(20),
                    ].as_ref()
                )
                .split(f.size());

            let block = Block::default()
                .borders(Borders::ALL)
                .title_style(Style::default().modifier(Modifier::BOLD));

            Paragraph::new([Text::raw(state.main_text.clone()), ].iter())
                .block(block.clone().title("Editor"))
                .render(&mut f, chunks[0]);

            Paragraph::new([Text::raw(state.debug_text.clone()), ].iter())
                .block(block.clone().title("Debug"))
                .render(&mut f, chunks[1]);
        })?;

        match read().unwrap() {
            Event::Key(k) => {
                if let KeyCode::Char(c) = k.code {
                    if state.mode == VimMode::Edit {
                        if c == ':' {
                            state.debug_text.push_str("Command mode\n");
                            state.mode = VimMode::Command;
                        } else {
                            state.main_text.push(c);
                        }
                    } else {
                        if c == 'q' {
                            break;
                        } else if c == 'f' {
                            state.debug_text.push_str("Edit mode\n");
                            state.mode = VimMode::Edit;
                        } else if c == 'w' {
                            state.debug_text.push_str("Name file > \n");
                            let f = File::open("buffer_dump.txt");
                        }
                    }
                }
                else {
                    if k.code == KeyCode::Backspace && state.mode == VimMode::Edit {
                        state.main_text.pop();
                        let c = terminal.get_cursor().unwrap();

                    } else if k.code == KeyCode::Enter && state.mode == VimMode::Edit {
                        state.main_text.push('\n');
                    }
                }
            },
            _ => {}
        }
    }

    terminal.clear().unwrap();

    Ok(())
}