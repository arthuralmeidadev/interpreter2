use colored::Colorize;

pub fn throw_undefined_class_error<'a>(
    identifier: &str,
    span_str: &str,
    position: usize,
    line: usize,
    suggestion: &str,
) -> String {
    let mut hightlight = String::new();
    let at_line = format!("At: {} | ", line);

    while hightlight.len() < position {
        hightlight += " ";
    }

    at_line.chars().for_each(|c| {
        hightlight += " ";
    });

    hightlight += " ";

    identifier.chars().for_each(|c| {
        hightlight += "^";
    });

    format!(
        "{}: Cannot access member of undefined: {}
        {at_line}{}
        {}
        {}",
        "Error".red().bold(),
        format!("`{identifier}`").bold(),
        format!("`{span_str}`").yellow().bold(),
        hightlight.red(),
        suggestion.cyan().bold()
    )
}

pub fn throw_undefined(
    identifier: &str,
    span_str: &str,
    position: usize,
    line: usize,
) -> String {
    let mut hightlight = String::new();
    let at_line = format!("At: {} | ", line);

    while hightlight.len() < position {
        hightlight += " ";
    }

    at_line.chars().for_each(|c| {
        hightlight += " ";
    });

    hightlight += " ";

    identifier.chars().for_each(|c| {
        hightlight += "^";
    });

    format!(
        "{}: {} is undefined
        {at_line}{}
        {}",
        "Error".red().bold(),
        format!("`{identifier}`").bold(),
        format!("`{span_str}`").yellow().bold(),
        hightlight.red(),
    )
}