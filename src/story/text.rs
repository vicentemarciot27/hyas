// Passages are written in two shapes and must render the same:
//
//   text: "First line sits next to the quote
//          continuation lines carry the file's indentation"
//
//   text: r#"
//          every line carries the indentation
//   "#
//
// The raw form also brings a blank first line and a whitespace-only last
// line

fn indent_of(line: &str) -> usize {
    line.bytes().take_while(|b| *b == b' ' || *b == b'\t').count()
}

fn is_blank(line: &str) -> bool {
    line.trim().is_empty()
}

pub fn dedent(text: &mut String) {
    let indent = text
        .lines()
        .skip(1)
        .filter(|line| !is_blank(line))
        .map(indent_of)
        .min()
        .unwrap_or(0);

    let lines: Vec<&str> = text.lines().collect();

    let first = lines.iter().position(|line| !is_blank(line));
    let last = lines.iter().rposition(|line| !is_blank(line));

    let (Some(first), Some(last)) = (first, last) else {
        text.clear();
        return;
    };

    let mut out = String::with_capacity(text.len());

    for (index, line) in lines[first..=last].iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }

        // Never cut into line 0 of the original text; it starts at column 0.
        let strip = if first + index == 0 {
            0
        } else {
            indent.min(indent_of(line))
        };

        out.push_str(&line[strip..]);
    }

    *text = out;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dedented(text: &str) -> String {
        let mut text = text.to_owned();
        dedent(&mut text);
        text
    }

    #[test]
    fn both_passage_shapes_land_flush_left() {
        let raw = dedented("\n                He tightens his grip on the folder.\n            ");
        let plain = dedented("Goiânia, July 6th 2053\n            It's winter.\n");

        assert_eq!(raw, "He tightens his grip on the folder.");
        assert_eq!(plain, "Goiânia, July 6th 2053\nIt's winter.");
    }

    #[test]
    fn interior_blank_lines_and_relative_indent_survive() {
        let text = dedented(
            "\n                One.\n\n                    Indented under one.\n\n                Two.\n            ",
        );

        assert_eq!(text, "One.\n\n    Indented under one.\n\nTwo.");
    }

    /// player_death pads a line with trailing spaces on purpose.
    #[test]
    fn trailing_spaces_inside_a_line_are_kept() {
        assert_eq!(dedented("It... hurts...   ..  .   "), "It... hurts...   ..  .   ");
    }
}
