# hyas

A terminal text adventure.

Built with [ratatui](https://ratatui.rs). Story lives in `data/passages.ron`
and is compiled into the binary.

## Play

Grab `hyas.zip` and run `hyas.exe` (Windows) or `./hyas-linux` (Linux x86_64).
Nothing to install. Needs an 80x24 terminal. `Ctrl+C` quits.

The Windows build is unsigned, so SmartScreen warns on first run:
More info > Run anyway.

## Build

    cargo run --release
    cargo test

Cross-compile for Windows from Linux (needs `cargo install cargo-xwin`):

    cargo xwin build --release --target x86_64-pc-windows-msvc

Act 1 only. No save/load implemented yet. Combat, items
(`data/items.ron`), enemies (`data/enemies.ron`) and dialogues
(`data/dialogues.ron`) are written but not wired into the game yet.
`validate_passages` only checks that the file is non-empty; the link and
condition checks named in its comments are not implemented.
