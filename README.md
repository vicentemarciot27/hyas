# hyas

A terminal text adventure.

Built with [ratatui](https://ratatui.rs).

## Design

You are free to try what you want. The game is deterministic and choices are written by hand.
The aim is for it to feel like an LLM is reacting to your choices, but instead it's just the result
of countless hours of the developer thinking each and every narrative path, with minimal mandatory convergence.

Your skills, attributes and inclinations don't dictate the actions you can take, but the natural capacity
of your character to execute them

You control Cadet, which is defined by:

- Attributes are innate or very hard to acquire: Perception, Memory, Reasoning, Will,
  Physique, Agility.
- Skills are things you can learn: social, technical, knowledge and combat
  skills such as Empathy, Medicine or Stealth.
- Virtues and vices are your moral inclinations. Each virtue sits between two
  vices, for example Cowardice, Fortitude, Recklessness.

  Will is a special attribute, in a sense that it allows you to overcome your moral inclinations
  with fewer debuffs.

For example, if a guard's hand is shaking. 
- Perception allows for the Cadet to notice it
- Medicine tells cadet it is withdrawal.
- A compassionate Cadet is inclined to offer help, and a cruel one to abuse the a weakness.
    - Everyone is capable of good and evil, but some people are more inclined for evildoing, going against one's nature requires will

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

Act 1 only. No save/load implemented yet. Attributes and skills are set by
point-buy at character creation and gate passages; virtues and vices are not
in the game yet. Combat, items
(`data/items.ron`), enemies (`data/enemies.ron`) and dialogues
(`data/dialogues.ron`) are written but not wired into the game yet.
`validate_passages` only checks that the file is non-empty; the link and
condition checks named in its comments are not implemented.
