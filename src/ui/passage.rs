// Speaker + message + available choices

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{app::App, story::model::Condition, ui};

const ATTRIBUTE_COLOR: Color = Color::Cyan;
const SKILL_COLOR: Color = Color::Magenta;

pub(super) fn render(frame: &mut Frame, app: &App) {
    let area = frame.area().inner(Margin {
        vertical: 1,
        horizontal: 1,
    });

    let choices = app.available_choices();

    let choices_height = if choices.is_empty() {
        1
    } else {
        (choices.len() as u16 * 2) + 2
    };

    let chunks =
        Layout::vertical([Constraint::Min(5), Constraint::Length(choices_height)]).split(area);

    render_message(frame, app, chunks[0]);

    if choices.is_empty() {
        render_hint(frame, app, chunks[1]);
    } else {
        render_choices(frame, app, &choices, chunks[1]);
    }
}

// Check tags
// Only top-level positive checks are shown

fn check_tags(when: &[Condition]) -> Vec<Span<'static>> {
    when.iter()
        .filter_map(|condition| match condition {
            Condition::AttributeAtLeast { attribute, .. } => Some(Span::styled(
                format!("[{} CHECK PASSED]", attribute.label().to_uppercase()),
                Style::new()
                    .fg(ATTRIBUTE_COLOR)
                    .add_modifier(Modifier::BOLD),
            )),

            Condition::SkillAtLeast { skill, .. } => Some(Span::styled(
                format!("[{} CHECK PASSED]", skill.label().to_uppercase()),
                Style::new().fg(SKILL_COLOR).add_modifier(Modifier::BOLD),
            )),

            _ => None,
        })
        .collect()
}

fn tag_line(tags: Vec<Span<'static>>) -> Line<'static> {
    let mut spans = Vec::with_capacity(tags.len() * 2);

    for (index, tag) in tags.into_iter().enumerate() {
        if index > 0 {
            spans.push(Span::raw(" "));
        }

        spans.push(tag);
    }

    Line::from(spans)
}

// Message

fn render_message(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();

    if let Some(message) = app.message() {
        let tags = check_tags(&message.when);

        // A narrated observation the player only got because of a stat gets printed
        // A spoken line does not
        if message.speaker.is_none() && !tags.is_empty() {
            lines.push(tag_line(tags));
            lines.push(Line::from(""));
        }

        if let Some(speaker) = &message.speaker {
            lines.push(Line::from(Span::styled(
                ui::display_name(speaker, &app.state.story).to_uppercase(),
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
        }

        for line in ui::expand(&message.text, &app.state.player.name).lines() {
            lines.push(Line::from(line.to_owned()));
        }
    }

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .style(Style::new().fg(Color::White)),
        );

    frame.render_widget(paragraph, area);
}

// Choices

fn choice_tags(when: &[Condition]) -> Vec<Span<'static>> {
    when.iter()
        .filter_map(|condition| match condition {
            Condition::AttributeAtLeast { attribute, .. } => Some(Span::styled(
                format!("[{}] ", attribute.label().to_uppercase()),
                Style::new()
                    .fg(ATTRIBUTE_COLOR)
                    .add_modifier(Modifier::BOLD),
            )),

            Condition::SkillAtLeast { skill, .. } => Some(Span::styled(
                format!("[{}] ", skill.label().to_uppercase()),
                Style::new().fg(SKILL_COLOR).add_modifier(Modifier::BOLD),
            )),

            _ => None,
        })
        .collect()
}

fn render_choices(frame: &mut Frame, app: &App, choices: &[usize], area: Rect) {
    let lines: Vec<Line> = choices
        .iter()
        .enumerate()
        .map(|(position, &index)| {
            let selected = position == app.selected_choice;
            let marker = if selected { "> " } else { "  " };

            let style = if selected {
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::new().fg(Color::White)
            };

            let choice = app.choice(index);

            let mut spans = vec![Span::styled(marker.to_owned(), style)];
            spans.extend(choice_tags(&choice.when));
            spans.push(Span::styled(choice.text.clone(), style));

            Line::from(spans)
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(
            Block::new()
                .borders(Borders::ALL)
                .style(Style::new().fg(Color::Gray)),
        );

    frame.render_widget(paragraph, area);
}

fn render_hint(frame: &mut Frame, app: &App, area: Rect) {
    let hint = if app.passage().next.is_some() {
        "enter continue   pgup/pgdn scroll   q quit"
    } else {
        "enter end   q quit"
    };

    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            hint,
            Style::new().fg(Color::DarkGray),
        ))),
        area,
    );
}