mod action;
mod app;
mod creation;
mod event;
mod state;
mod story;
mod ui;

use std::io;

use app::App;
use story::{Story, loader::embedded_passages, loader::embedded_maps};

fn main() -> io::Result<()> {
    let passages = embedded_passages();
    let maps = embedded_maps();

    let mut app = App::new(Story::new(passages, maps));

    ratatui::run(|terminal| app.run(terminal))
}
