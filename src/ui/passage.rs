// App
//  ↓
// current Message
//  ↓
// Paragraph
//  ↓
// Frame

use ratatui::{
    Frame,
    layout::Margin,
    prelude::Stylize,
    style::Color,
    widgets::{
        Block,
        Borders,
        Paragraph,
        Wrap,
    },
};

pub(super) fn render(
    frame: &mut Frame,
    content: &str,
    scroll: u16,
) {
    let area = frame.area().inner(Margin {
        vertical: 1,
        horizontal: 1,
    });

    let passage = Paragraph::new(content)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0))
        .block(
            Block::new()
                .bold()
                .fg(Color::Blue)
                .borders(Borders::ALL),
        );

    frame.render_widget(passage, area);
}