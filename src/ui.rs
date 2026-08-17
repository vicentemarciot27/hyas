// frame
// └── render app

mod passage;

use ratatui::Frame;

pub fn render(
    frame: &mut Frame,
    content: &str,
    scroll: u16,
) {
    passage::render(frame, content, scroll);
}