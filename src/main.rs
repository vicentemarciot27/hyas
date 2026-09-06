mod action;
mod app;
mod creation;
mod event;
mod state;
mod story;
mod ui;

use std::{env, io};

use app::App;
use story::{
    Story,
    loader::{embedded_maps, embedded_passages},
};
use story::model::Position;

fn main() -> io::Result<()> {
    let passages = embedded_passages();
    let maps = embedded_maps();

    let mut app = App::new(
        Story::new(passages, maps)
    );

    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--map" => {
                let map_id = args
                    .next()
                    .expect("--map requires a map id");

                app.enter_map(
                    map_id,
                    Position {
                        x: 0,
                        y: 0,
                    },
                );
            }

            _ => {
                panic!("unknown argument: {arg}");
            }
        }
    }

    ratatui::run(|terminal| {
        app.run(terminal)
    })
}