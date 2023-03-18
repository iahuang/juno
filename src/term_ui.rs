pub mod console;

use crate::{
    mips::instruction::InstructionArgs,
    runtime::{register_aliases::*, vm::VM},
};
use crossterm::{event, execute, terminal};
use std::io::{self, Write};
use tui::{
    backend::{self, Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame, Terminal,
};

pub enum VMViewerEvent {
    None,
    Quit,
    TogglePause,
}

pub struct VMState<'a> {
    pub vm: &'a VM,
    pub paused: bool,
    pub halted: bool,
    pub console: &'a console::Console<'a>,
}

pub struct VMViewer<B: Backend> {
    terminal: Box<Terminal<B>>,
}

pub fn make_crossterm_viewer() -> VMViewer<CrosstermBackend<io::Stdout>> {
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend).unwrap();

    VMViewer {
        terminal: Box::new(terminal),
    }
}

fn ui_registers<B: Backend>(root: &mut Frame<B>, state: &VMState, rect: Rect) {
    let block = Block::default();

    root.render_widget(block, rect);

    let registers_block = Block::default()
        .borders(Borders::ALL)
        .title(" Registers ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    // Render registers

    let header = Row::new(vec!["#", "Name", "Value"])
        .style(Style::default().fg(Color::Yellow))
        .bottom_margin(1);
    let mut rows = vec![];

    fn make_row<'a>(reg: &'a str, name: &'a str, value: u32) -> Row<'a> {
        Row::new(vec![
            Cell::from(reg),
            Cell::from(name),
            Cell::from(format!("{}", value)),
        ])
    }

    fn make_row_hex<'a>(reg: &'a str, name: &'a str, value: u32) -> Row<'a> {
        Row::new(vec![
            Cell::from(reg),
            Cell::from(name),
            Cell::from(format!("{:#010x}", value)),
        ])
    }

    let vm = state.vm;

    rows.push(make_row("$0", "zero", vm.get_register(0).unwrap()));
    rows.push(make_row("$1", "at", vm.get_register(1).unwrap()));
    rows.push(make_row("$2", "v0", vm.get_register(2).unwrap()));
    rows.push(make_row("$3", "v1", vm.get_register(3).unwrap()));
    rows.push(make_row("$4", "a0", vm.get_register(4).unwrap()));
    rows.push(make_row("$5", "a1", vm.get_register(5).unwrap()));
    rows.push(make_row("$6", "a2", vm.get_register(6).unwrap()));
    rows.push(make_row("$7", "a3", vm.get_register(7).unwrap()));
    rows.push(make_row("$8", "t0", vm.get_register(8).unwrap()));
    rows.push(make_row("$9", "t1", vm.get_register(9).unwrap()));
    rows.push(make_row("$10", "t2", vm.get_register(10).unwrap()));
    rows.push(make_row("$11", "t3", vm.get_register(11).unwrap()));
    rows.push(make_row("$12", "t4", vm.get_register(12).unwrap()));
    rows.push(make_row("$13", "t5", vm.get_register(13).unwrap()));
    rows.push(make_row("$14", "t6", vm.get_register(14).unwrap()));
    rows.push(make_row("$15", "t7", vm.get_register(15).unwrap()));
    rows.push(make_row("$16", "s0", vm.get_register(16).unwrap()));
    rows.push(make_row("$17", "s1", vm.get_register(17).unwrap()));
    rows.push(make_row("$18", "s2", vm.get_register(18).unwrap()));
    rows.push(make_row("$19", "s3", vm.get_register(19).unwrap()));
    rows.push(make_row("$20", "s4", vm.get_register(20).unwrap()));
    rows.push(make_row("$21", "s5", vm.get_register(21).unwrap()));
    rows.push(make_row("$22", "s6", vm.get_register(22).unwrap()));
    rows.push(make_row("$23", "s7", vm.get_register(23).unwrap()));
    rows.push(make_row("$24", "t8", vm.get_register(24).unwrap()));
    rows.push(make_row("$25", "t9", vm.get_register(25).unwrap()));
    rows.push(make_row("$26", "k0", vm.get_register(26).unwrap()));
    rows.push(make_row("$27", "k1", vm.get_register(27).unwrap()));
    rows.push(make_row_hex("$28", "gp", vm.get_register(28).unwrap()));
    rows.push(make_row_hex("$29", "sp", vm.get_register(29).unwrap()));
    rows.push(make_row_hex("$30", "fp", vm.get_register(30).unwrap()));
    rows.push(make_row_hex("$31", "ra", vm.get_register(31).unwrap()));

    rows.push(make_row_hex("", "pc", vm.get_pc() as u32));

    rows.push(make_row("", "hi", vm.get_hi()));
    rows.push(make_row("", "lo", vm.get_lo()));

    let table = Table::new(rows)
        .header(header)
        .block(registers_block)
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(30),
            Constraint::Percentage(60),
        ]);

    root.render_widget(table, rect);
}

fn ui_console<B: Backend>(root: &mut Frame<B>, state: &VMState, rect: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Percentage(90)].as_ref())
        .split(rect);

    let mut color = if state.paused { Color::Yellow } else { Color::White };

    if state.halted {
        color = Color::Red;
    }

    let mut title = " Running ";

    if state.paused {
        title = " Paused ";
    }
    if state.halted {
        title = " Halted ";
    }

    let control_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let control_text = Paragraph::new(if state.paused {
        "[P] Resume\n[R] Reset\n[Q] Quit"
    } else {
        if state.halted {
            "[R] Reset\n[Q] Quit"
        } else {
            "[P] Pause\n[R] Reset\n[Q] Quit"
        }
    })
    .style(Style::default().fg(color))
    .block(control_block)
    .alignment(Alignment::Left);

    let console_block = Block::default()
        .borders(Borders::ALL)
        .title(" Console ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    root.render_widget(state.console.as_paragraph().block(console_block), layout[1]);

    root.render_widget(control_text, layout[0]);
}

fn ui_next_instructions<B: Backend>(root: &mut Frame<B>, state: &VMState, rect: Rect) {
    let block = Block::default();

    root.render_widget(block, rect);

    let memory_block = Block::default()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let header = Row::new(vec!["Address", "Code", "Decoded Instruction"])
        .style(Style::default().fg(Color::Yellow))
        .bottom_margin(1);

    let mut rows = vec![];

    /// Make a row for the memory table.
    fn make_row<'a>(address: u32, code: u32, instruction: Spans<'a>) -> Row<'a> {
        Row::new(vec![
            Cell::from(format!("{:#010x}", address)),
            Cell::from(format!("{:08x}", code)),
            Cell::from(instruction),
        ])
    }

    let mut decoded_instructions: Vec<Spans> = vec![];
    let vm = state.vm;
    
    for i in vm.get_pc() / 4..vm.get_pc() / 4 + 10 {
        let address = i * 4;
        let code = vm.memory.get_word(address).unwrap();
        let instruction = vm.decode_instruction(code);

        let mut spans = vec![];
        if let Ok(instruction) = instruction {
            if !instruction.is_null() {
                spans.push(Span::styled(
                    format!("{}", instruction.base.name),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ));

                match instruction.args {
                    InstructionArgs::RFormat(args) => {
                        spans.push(Span::styled(
                            format!(" ${}, ${}, ${}", args.rd, args.rs, args.rt),
                            Style::default().fg(Color::Green),
                        ));
                    }
                    InstructionArgs::IFormat(args) => {
                        spans.push(Span::styled(
                            format!(" ${}, ${}, {:#06x}", args.rt, args.rs, args.imm),
                            Style::default().fg(Color::Green),
                        ));
                    }
                    InstructionArgs::JFormat(args) => {
                        spans.push(Span::styled(
                            format!(" {:#010x}", args.address << 2),
                            Style::default().fg(Color::Green),
                        ));
                    }
                }
            }
        } else {
            spans.push(Span::styled("???", Style::default().fg(Color::Red)));
        }

        decoded_instructions.push(Spans::from(spans));

        rows.push(make_row(
            address as u32,
            code,
            decoded_instructions.last().unwrap().to_owned(),
        ));
    }

    let table = Table::new(rows)
        .header(header)
        .block(memory_block)
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(60),
        ]);

    root.render_widget(table, rect);
}

fn ui_state<B: Backend>(root: &mut Frame<B>, state: &VMState, rect: Rect) {
    let block = Block::default();

    root.render_widget(block, rect);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(rect);

    ui_registers(root, state, chunks[0]);

    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[1]);

    ui_next_instructions(root, state, v_chunks[0]);
    ui_console(root, state, v_chunks[1]);
}

fn ui<B: Backend>(root: &mut Frame<B>, vm: &VMState) {
    let size = root.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Juno ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    root.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(size);

    // render state

    ui_state(root, vm, chunks[0]);

    // render controls

    let controls_block = Block::default()
        .borders(Borders::ALL)
        .title(" Controls ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
}

impl<'a, B: Backend> VMViewer<B> {
    pub fn init(&mut self) -> Result<(), io::Error> {
        terminal::enable_raw_mode()?;

        execute!(io::stdout(), terminal::EnterAlternateScreen)?;

        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), io::Error> {
        execute!(io::stdout(), terminal::LeaveAlternateScreen)?;

        terminal::disable_raw_mode()?;

        Ok(())
    }

    /// Update the UI with the current state of the VM.
    pub fn update(
        &mut self,
        state: &VMState,
    ) -> Result<VMViewerEvent, io::Error> {
        self.terminal.draw(|f| {
            ui(f, state);
        })?;

        // handle input
        let poll = event::poll(std::time::Duration::from_millis(100))?;

        if !poll {
            return Ok(VMViewerEvent::None);
        }

        let event = event::read();

        match event {
            Ok(event::Event::Key(key)) => match key.code {
                // check for "q" or "ctrl+c"
                event::KeyCode::Char('q') => return Ok(VMViewerEvent::Quit),
                event::KeyCode::Char('c') => {
                    if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                        return Ok(VMViewerEvent::Quit);
                    }
                }
                event::KeyCode::Char('p') => return Ok(VMViewerEvent::TogglePause),
                _ => {}
            },
            _ => {}
        }

        Ok(VMViewerEvent::None)
    }
}
