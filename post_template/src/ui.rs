use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{List, ListItem, Paragraph, Wrap};

use crate::app_state::AppState;
use crate::openlibrary;

pub fn open_ui() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = AppState::default();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: AppState) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    app.search_results = openlibrary::search_books(&app.input);
                }
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Esc => {
                    return Ok(());
                }
                KeyCode::Left => {
                    app.search_results.unselect();
                }
                KeyCode::Down => {
                    app.search_results.next();
                }
                KeyCode::Up => {
                    app.search_results.previous();
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ]
                .as_ref(),
        )
        .split(f.size());

    render_search_box(f, app, &vertical_layout);

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ]
                .as_ref(),
        )
        .split(vertical_layout[1]);

    render_book_list(f, app, &horizontal_layout);
    render_synopsis(f, app, &horizontal_layout)
}

fn render_search_box<B: Backend>(f: &mut Frame<B>, app: &AppState, vertical_layout: &Vec<Rect>) {
    let search = Paragraph::new(app.input.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(search, vertical_layout[0]);
}

fn render_book_list<B: Backend>(f: &mut Frame<B>, app: &mut AppState, horizontal_layout: &Vec<Rect>) {
    let block = Block::default().title("Books").borders(Borders::ALL);
    let book_items: Vec<ListItem> = app
        .search_results
        .items
        .iter()
        .map(|book| {
            ListItem::new(book.title.clone()).style(Style::default())
        })
        .collect();
    let book_list = List::new(book_items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(book_list, horizontal_layout[0], &mut app.search_results.state);
}

fn render_synopsis<B: Backend>(f: &mut Frame<B>, app: &AppState, horizontal_layout: &Vec<Rect>) {
    let block = Block::default().title("Description").borders(Borders::ALL);
    let description_text = match app.search_results.state.selected() {
        Some(index) => app.search_results.items[index].synopsis.clone(),
        _ => "".to_string()
    };
    let description = Paragraph::new(description_text)
        .style(Style::default())
        .wrap(Wrap { trim: false })
        .block(block);
    f.render_widget(description, horizontal_layout[1]);
}
