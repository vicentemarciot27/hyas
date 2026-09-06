// Map + Character Exploration UI + Character + NPCs + Items

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app::{App, Mode},
    story::model::{
        CoverType,
        Direction,
        MapEntry,
        Position,
    },
};

#[derive(Clone, Copy, Default)]
struct WallCell {
    cover: Option<CoverType>,
}

const X_STEP: usize = 4;
const Y_STEP: usize = 2;

const PLAYER_COLOR: Color = Color::Red;
const FLOOR_COLOR: Color = Color::DarkGray;
const BLOCKED_COLOR: Color = Color::White;

const FULL_COVER_COLOR: Color = Color::White;
const HALF_COVER_COLOR: Color = Color::Cyan;

pub(super) fn render(frame: &mut Frame, app: &App) {
    let area = frame.area().inner(Margin {
        vertical: 1,
        horizontal: 1,
    });

    let Mode::Exploration {
        map_id,
        position,
    } = &app.mode
    else {
        return;
    };

    let map = app.story.get_map(map_id);

    let chunks = Layout::vertical([
        Constraint::Min(5),
        Constraint::Length(1),
    ])
    .split(area);

    render_map(
        frame,
        map,
        *position,
        chunks[0],
    );

    render_hint(frame, chunks[1]);
}
fn render_map(
    frame: &mut Frame,
    map: &MapEntry,
    player_position: Position,
    area: Rect,
) {
    let width = map.map.width as usize;
    let height = map.map.height as usize;

    let display_width =
        (width - 1) * X_STEP + 1;

    let display_height =
        (height - 1) * Y_STEP + 1;

    /*
     * Semantic wall layer.
     *
     * We don't decide the glyph yet.
     * First we draw every cover, then we inspect
     * neighboring wall cells to determine whether
     * each point is:
     *
     * │ ─ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼
     */
    let mut walls = vec![
        vec![WallCell::default(); display_width];
        display_height
    ];

    for &(x, y, direction, cover_type) in &map.map.cover {
        draw_cover(
            &mut walls,
            x as usize,
            y as usize,
            direction,
            cover_type,
        );
    }

    let player_screen_x =
        player_position.x as usize * X_STEP;

    let player_screen_y =
        player_position.y as usize * Y_STEP;

    let mut lines = Vec::new();

    for screen_y in 0..display_height {
        let mut spans = Vec::new();

        for screen_x in 0..display_width {
            /*
             * Player has highest visual priority.
             */
            if screen_x == player_screen_x
                && screen_y == player_screen_y
            {
                spans.push(Span::styled(
                    "@",
                    Style::new()
                        .fg(PLAYER_COLOR)
                        .add_modifier(Modifier::BOLD),
                ));

                continue;
            }

            /*
             * Logical tile centers.
             */
            let on_tile =
                screen_x % X_STEP == 0
                    && screen_y % Y_STEP == 0;

            if on_tile {
                let logical_x =
                    screen_x / X_STEP;

                let logical_y =
                    screen_y / Y_STEP;

                if map
                    .map
                    .blocked
                    .contains(&(
                        logical_x as u16,
                        logical_y as u16,
                    ))
                {
                    spans.push(Span::styled(
                        "#",
                        Style::new()
                            .fg(BLOCKED_COLOR)
                            .add_modifier(
                                Modifier::BOLD
                            ),
                    ));
                } else {
                    spans.push(Span::styled(
                        ".",
                        Style::new()
                            .fg(FLOOR_COLOR),
                    ));
                }

                continue;
            }

            /*
             * Wall / cover.
             */
            if let Some(cover_type) =
                walls[screen_y][screen_x].cover
            {
                let glyph =
                    wall_glyph(
                        &walls,
                        screen_x,
                        screen_y,
                    );

                let style =
                    cover_style(cover_type);

                spans.push(Span::styled(
                    glyph.to_string(),
                    style,
                ));

                continue;
            }

            /*
             * Nothing occupies this visual cell.
             */
            spans.push(Span::raw(" "));
        }

        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Map"),
        );

    frame.render_widget(
        paragraph,
        area,
    );
}

fn mark_wall(
    cell: &mut WallCell,
    new_cover: CoverType,
) {
    cell.cover = match (
        cell.cover,
        new_cover,
    ) {
        /*
         * If either wall is Full,
         * the intersection is rendered as Full.
         */
        (
            Some(CoverType::Full),
            _
        )
        | (
            _,
            CoverType::Full
        ) => Some(CoverType::Full),

        _ => Some(CoverType::Half),
    };
}

fn draw_cover(
    walls: &mut [Vec<WallCell>],
    x: usize,
    y: usize,
    direction: Direction,
    cover_type: CoverType,
) {
    match direction {
        Direction::South => {
            draw_horizontal_cover(
                walls,
                x,
                y,
                cover_type,
            );
        }

        Direction::East => {
            draw_vertical_cover(
                walls,
                x,
                y,
                cover_type,
            );
        }
    }
}

fn draw_horizontal_cover(
    walls: &mut [Vec<WallCell>],
    x: usize,
    y: usize,
    cover_type: CoverType,
) {
    if walls.is_empty() {
        return;
    }

    let wall_y =
        y * Y_STEP + Y_STEP / 2;

    // South edge would be outside the rendered map.
    if wall_y >= walls.len() {
        return;
    }

    let center_x =
        x * X_STEP;

    let half = X_STEP / 2;

    let start =
        center_x.saturating_sub(half);

    let end =
        (center_x + half)
            .min(walls[wall_y].len() - 1);

    for screen_x in start..=end {
        mark_wall(
            &mut walls[wall_y][screen_x],
            cover_type,
        );
    }
}

fn draw_vertical_cover(
    walls: &mut [Vec<WallCell>],
    x: usize,
    y: usize,
    cover_type: CoverType,
) {
    if walls.is_empty() {
        return;
    }

    let wall_x =
        x * X_STEP + X_STEP / 2;

    // East edge would be outside the rendered map.
    if wall_x >= walls[0].len() {
        return;
    }

    let center_y =
        y * Y_STEP;

    let half = Y_STEP / 2;

    let start =
        center_y.saturating_sub(half);

    let end =
        (center_y + half)
            .min(walls.len() - 1);

    for screen_y in start..=end {
        mark_wall(
            &mut walls[screen_y][wall_x],
            cover_type,
        );
    }
}

fn wall_glyph(
    walls: &[Vec<WallCell>],
    x: usize,
    y: usize,
) -> char {
    let north =
        y > 0 && walls[y - 1][x].cover.is_some();

    let south =
        y + 1 < walls.len() && walls[y + 1][x].cover.is_some();

    let west =
        x > 0 && walls[y][x - 1].cover.is_some();

    let east =
        x + 1 < walls[y].len() && walls[y][x + 1].cover.is_some();

    match (north, east, south, west) {
        // Straight lines.
        (true, false, true, false) => '│',
        (false, true, false, true) => '─',

        // Corners.
        (false, true, true, false) => '┌',
        (false, false, true, true) => '┐',
        (true, true, false, false) => '└',
        (true, false, false, true) => '┘',

        // T junctions.
        (true, true, true, false) => '├',
        (true, false, true, true) => '┤',
        (false, true, true, true) => '┬',
        (true, true, false, true) => '┴',

        // Four-way intersection.
        (true, true, true, true) => '┼',

        // End caps.
        (true, false, false, false)
        | (false, false, true, false) => '│',

        (false, true, false, false)
        | (false, false, false, true) => '─',

        // Isolated wall point. Should be uncommon, but this
        // makes malformed/very small walls visible.
        (false, false, false, false) => '•',
    }
}

fn cover_style(cover_type: CoverType) -> Style {
    match cover_type {
        CoverType::Full => Style::new()
            .fg(FULL_COVER_COLOR)
            .add_modifier(Modifier::BOLD),

        CoverType::Half => Style::new().fg(HALF_COVER_COLOR),
    }
}

fn render_hint(frame: &mut Frame, area: Rect) {
    let hint = "Use arrow keys to move. Press 'q' to quit.";

    let paragraph = Paragraph::new(hint)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Hint"),
        );

    frame.render_widget(paragraph, area);
}