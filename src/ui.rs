mod passage;

use ratatui::{
    Frame, style::Color,
    layout::{Margin},
    widgets::{
        Paragraph,Block,Borders
    },
    prelude::Stylize

};

// frame
// └── render app

fn render(frame: &mut Frame, content: &str) {
    let area = frame.area().inner(Margin {
        vertical: 1,
        horizontal: 1
    });

    frame.render_widget(
        Paragraph::new(content)
            .block(
                Block::new()
                    .bold()
                    .fg(Color::Blue)
                    .borders(Borders::ALL)
            ),
            area,
    )
}
