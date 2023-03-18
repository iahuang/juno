use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::runtime::errors::{FatalErrorType, RuntimeError};

pub struct Console<'a> {
    lines: Vec<Spans<'a>>,
}

impl<'a> Console<'a> {
    pub fn new() -> Console<'a> {
        Console {
            // Add an empty line to start with, just for visual purposes.
            lines: vec![Spans::from(vec![Span::raw("")])], 
        }
    }

    pub fn runtime_error(&mut self, err: &RuntimeError) {
        self.add_line(Spans::from(vec![
            Span::styled(
                "[runtime error] ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                match err.err_type {
                    FatalErrorType::IllegalMemoryAccess => "ILLEGAL_MEMORY_ACCESS",
                    FatalErrorType::IllegalInstruction => "ILLEGAL_INSTRUCTION",
                    FatalErrorType::IllegalRegisterAccess => "ILLEGAL_REGISTER",
                },
                Style::default().fg(Color::Red),
            ),
            Span::raw(format!(": {}", err.message)),
        ]));
    }

    pub fn trap_error(&mut self, message: &str) {
        self.add_line(Spans::from(vec![
            Span::styled(
                "[trap] ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(message.to_owned()),
        ]));
    }

    pub fn execution_finished(&mut self, message: &str) {
        self.add_line(Spans::from(vec![
            Span::styled(
                "[execution finished] ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(message.to_owned()),
        ]));
    }

    fn add_line(&mut self, line: Spans<'a>) {
        self.lines.push(line);
    }

    pub fn reset(&mut self) {
        self.lines.clear();
    }

    pub fn as_paragraph(&self) -> Paragraph<'a> {
        Paragraph::new(
            self.lines
                .iter()
                .map(|line| line.clone())
                .collect::<Vec<Spans>>(),
        )
    }
}
