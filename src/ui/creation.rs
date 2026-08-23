// Point-buy screen: two pools, one cursor

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::creation::{
    ATTRIBUTE_MAX, ATTRIBUTE_POINTS, PointBuy, Row, SKILL_MAX, SKILL_POINTS,
};

pub(super) fn render(frame: &mut Frame, point_buy: &PointBuy) {
    let area = frame.area().inner(Margin {
        vertical: 1,
        horizontal: 1,
    });

    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(3),
        Constraint::Length(1),
    ])
    .split(area);

    render_pools(frame, point_buy, chunks[0]);
    render_rows(frame, point_buy, chunks[1]);
    render_hint(frame, point_buy, chunks[2]);
}

fn render_pools(frame: &mut Frame, point_buy: &PointBuy, area: Rect) {
    let attributes = point_buy.attribute_points_left();
    let skills = point_buy.skill_points_left();

    let spent = |left: u16| {
        if left == 0 {
            Style::new().fg(Color::Green)
        } else {
            Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        }
    };

    let line = Line::from(vec![
        Span::styled("attribute points  ", Style::new().fg(Color::Gray)),
        Span::styled(
            format!("{attributes}/{ATTRIBUTE_POINTS}"),
            spent(attributes),
        ),
        Span::styled("      skill points  ", Style::new().fg(Color::Gray)),
        Span::styled(format!("{skills}/{SKILL_POINTS}"), spent(skills)),
    ]);

    let paragraph = Paragraph::new(line).block(
        Block::new()
            .borders(Borders::ALL)
            .title(" Cadet ")
            .style(Style::new().fg(Color::White)),
    );

    frame.render_widget(paragraph, area);
}

fn render_rows(frame: &mut Frame, point_buy: &PointBuy, area: Rect) {
    let height = area.height.saturating_sub(2) as usize;
    let offset = point_buy.cursor.saturating_sub(height.saturating_sub(1));

    let lines: Vec<Line> = point_buy
        .rows()
        .enumerate()
        .skip(offset)
        .take(height)
        .map(|(index, row)| line_for(index, row, point_buy.cursor))
        .collect();

    let paragraph = Paragraph::new(lines).block(
        Block::new()
            .borders(Borders::ALL)
            .style(Style::new().fg(Color::DarkGray)),
    );

    frame.render_widget(paragraph, area);
}

fn line_for(index: usize, row: Row, cursor: usize) -> Line<'static> {
    let selected = index == cursor;
    let marker = if selected { "> " } else { "  " };

    let (label, value, max) = match row {
        Row::Attribute(attribute, value) => (attribute.label(), value, ATTRIBUTE_MAX),
        Row::Skill(skill, value) => (skill.label(), value, SKILL_MAX),
    };

    let style = if selected {
        Style::new()
            .fg(Color::Black)
            .bg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else if value > 0 {
        Style::new().fg(Color::White)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    Line::from(Span::styled(
        format!("{marker}{label:<18} {value:>3} / {max}"),
        style,
    ))
}

fn render_hint(frame: &mut Frame, point_buy: &PointBuy, area: Rect) {
    let (hint, style) = if point_buy.is_complete() {
        (
            "enter begin   h/l -1 +1   H/L -5 +5   q quit",
            Style::new().fg(Color::Green),
        )
    } else {
        (
            "spend every point to begin   h/l -1 +1   H/L -5 +5   q quit",
            Style::new().fg(Color::DarkGray),
        )
    };

    frame.render_widget(Paragraph::new(Line::from(Span::styled(hint, style))), area);
}
