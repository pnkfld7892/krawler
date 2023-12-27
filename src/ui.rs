use crossterm::style::style;
use ratatui::{
    layout::*,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem,Paragraph,Wrap},
    Frame
};


use crate::app::{self, App, AppMode}; 

pub fn ui(f: &mut Frame, app: &App){
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3)
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title = Paragraph::new(Text::styled("Krawler Twitch Chat", Style::default()
            .fg(Color::LightYellow)))
        .block(title_block)
        .alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();

    for msg in &app.messages{
       list_items.push(
           ListItem::new(
               Line::from(Span::styled(format!("{}: {}",msg.user.clone(),msg.message.clone()),
                       Style::default().fg(Color::White))
               ))
           ); 
    }

    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let display_list = List::new(list_items).block(list_block);

    f.render_widget(display_list, chunks[1]);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let message_input = Paragraph::new(app.chat_input.clone()).block(input_block);
    f.render_widget(message_input, chunks[2]);

}

