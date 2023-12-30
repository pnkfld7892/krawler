use crossterm::style::style;
use ratatui::{
    layout::*,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::*,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{self, App, AppMode};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title = Paragraph::new(Text::styled(
        "Krawler Twitch Chat",
        Style::default().fg(Color::LightYellow),
    ))
    .block(title_block)
    .alignment(Alignment::Center);

    let mut list_items = Vec::<ListItem>::new();

    for msg in &app.messages {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}: {}", msg.user.clone(), msg.message.clone()),
            Style::default().fg(Color::White),
        ))));
    }

    let mut list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mut input_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    match app.app_mode {
        AppMode::ChatEnter => {
           input_block = input_block.border_style(Style::new().light_green());
        },
        AppMode::Normal => {
            list_block = list_block.border_style(Style::new().light_green());
        },

    }

    let display_list = List::new(list_items)
        .direction(ratatui::widgets::ListDirection::BottomToTop)
        .block(list_block.clone());

    let message_input = Paragraph::new(app.chat_input.clone()).block(input_block.clone());

    f.render_widget(title, chunks[0]);
    f.render_widget(display_list, chunks[1]);
    f.render_widget(message_input, chunks[2]);
}
