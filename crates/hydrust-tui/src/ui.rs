use ratatui::{
    buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::Style, widgets::{Block, Borders, List, ListDirection, ListState, Paragraph, Widget}
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 1. Vertical Split
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        // 2. Horizontal Split of the top chunk
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(chunks[0]);

        // 3. Define the Widgets
        let left_panel = Block::default().title("Incoming events").borders(Borders::ALL);
        let main_panel = Block::default().title("Active tasks").borders(Borders::ALL);
        let bottom_panel = Block::default()
            .title("Command bar")
            .borders(Borders::ALL);

        let mut state = ListState::default(); 
        let items = ["Item 1", "Item 2", "Item 3"];
        let list = List::new(items)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::BottomToTop);

        let info_text = Paragraph::new(format!("Counter: {}\nPress 'q' to quit", self.counter));

        // 4. Render them to their SPECIFIC chunks
        // Instead of 'area', use the chunks we calculated
        left_panel.render(top_chunks[0], buf);
        main_panel.render(top_chunks[1], buf);

        // You can also wrap a widget (like Paragraph) inside a block in a specific chunk
        list.block(bottom_panel).render(chunks[1], buf);
    }
}
