fn _is_tab_or_space(c: char) -> bool {
    return c == '\t' || c == ' ';
}

pub fn dedent(text: &mut String) {
    // grabs the minimum size of a line that contains a tab to the left
    let min_tabs = text
        .lines()
        .map(|line| line.chars().take_while(|&c| _is_tab_or_space(c) ).count())
        .filter(|&count| count > 0)
        .min()
        .unwrap_or(0);

    if min_tabs == 0 {
        return;
    }

    // removes tabs from all lines that contain a tab to the left
    let mut reconstructed_string = String::with_capacity(text.len());

    for (i, line) in text.lines().enumerate() {
        if i > 0 {
            reconstructed_string.push('\n');
        }

        let line_tabs = line.chars().take_while(|&c| _is_tab_or_space(c)).count();
        // If the line doesn't have a tab (line_tabs = 0),
        // tabs_to_remove is 0
        let tabs_to_remove = line_tabs.min(min_tabs);

        reconstructed_string.push_str(&line[tabs_to_remove..]);
    }


    *text = reconstructed_string;
}
