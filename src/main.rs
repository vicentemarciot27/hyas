mod action;
mod app;
mod event;
mod story;
mod ui;

use std::io;

use app::App;
use story::loader::load_passages;

fn main() -> io::Result<()> {
    let passages =
        load_passages("data/passages.ron")
            .expect("failed to load passages");

    let mut app = App::new(passages);

    ratatui::run(|terminal| app.run(terminal))
}