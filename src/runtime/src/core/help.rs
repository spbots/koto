use {crate::ValueMap, std::iter::Peekable};

pub fn add_help_from_markdown(module: &mut ValueMap, markdown: &str) {
    // Dirt cheap markdown parsing, splitting up the help markdown by section.

    let mut lines = markdown.lines().peekable();

    let mut help = String::with_capacity(1024);

    let module_name = {
        let first_line = lines.next().expect("Empty help source");
        lines.next(); // Skip over empty line

        help.push_str(first_line);
        help.push_str("\n\n");

        let module_name = first_line
            .strip_prefix("# ")
            .expect("Missing module name")
            .trim_matches('`');

        consume_help_section(&mut lines, &mut help, "# ");

        module.add_self_help(&help.trim_end());
        module_name
    };

    while let Some(line) = lines.peek() {
        if let Some(function_name) = line.strip_prefix("## ") {
            lines.next(); // Consume peeked line
            lines.next(); // Skip over empty line

            help.clear();
            help.push_str("# ");
            help.push_str(module_name);
            help.push_str(".");
            help.push_str(function_name);
            help.push_str("\n\n");

            consume_help_section(&mut lines, &mut help, "## ");

            module.add_help(function_name, help.trim_end());
        } else {
            lines.next();
        }
    }
}

fn consume_help_section<'a, T>(line_iter: &mut Peekable<T>, help: &mut String, end_marker: &str)
where
    T: Iterator<Item = &'a str>,
{
    while let Some(line) = line_iter.peek() {
        if line.starts_with(end_marker) {
            return;
        }

        // Strip out ``` code block markers
        if !line.starts_with("```") {
            // Reduce subsection markers by 1
            if line.starts_with("###") {
                help.push_str(&line[1..]);
            } else {
                help.push_str(line)
            }

            help.push_str("\n");
        }

        line_iter.next();
    }
}
