use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use futures::executor::block_on;
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame,
    layout::{Constraint, Direction, Layout},
    Terminal, widgets::{Block, Borders},
};
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{List, ListItem, Paragraph, Wrap};

use app_state::UIState;
use crate::openlibrary;

mod app_state;
pub mod stateful_list;

pub fn open_ui() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let app = UIState::default();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: UIState) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    match app.search_results.state.selected() {
                        None => search_books(&mut app),
                        Some(selected) => return app.search_results.items[selected].crete_consolidated_files()
                    }
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

fn search_books(app: &mut UIState) {
    match block_on(openlibrary::search_books(&app.input)) {
        Ok(result) => app.search_results = result,
        Err(error) => panic!("Unexpected response from book search: {}", error)
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut UIState) {
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

fn render_search_box<B: Backend>(f: &mut Frame<B>, app: &UIState, vertical_layout: &[Rect]) {
    let search = Paragraph::new(app.input.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(search, vertical_layout[0]);
}

fn render_book_list<B: Backend>(f: &mut Frame<B>, app: &mut UIState, horizontal_layout: &[Rect]) {
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

fn render_synopsis<B: Backend>(f: &mut Frame<B>, app: &UIState, horizontal_layout: &[Rect]) {
    let block = Block::default().title("Description").borders(Borders::ALL);
    let description_text = match app.search_results.state.selected() {
        Some(index) => {
            let selected_book = &app.search_results.items[index];
            format!("Key: {} \nAuthors: {:?}", selected_book.openlibrary_id, selected_book.authors)
        },
        _ => "".to_string()
    };
    let description = Paragraph::new(description_text)
        .style(Style::default())
        .wrap(Wrap { trim: false })
        .block(block);
    f.render_widget(description, horizontal_layout[1]);
}
