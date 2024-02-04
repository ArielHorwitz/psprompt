use crate::colors::{Color, StyledText};
use crate::{Components, Style, RESET};

pub fn apply_colors(text: &Components<String>, fg: &Components<Color>) -> Components<StyledText> {
    Components {
        user: StyledText::new(&text.user, fg.user),
        host: StyledText::new(&text.host, fg.host),
        loc: StyledText::new(&text.loc, fg.loc),
        prompt: StyledText::new(&text.prompt, fg.prompt),
        icon: StyledText::new(&text.icon, fg.icon),
        at: StyledText::new(&text.at, fg.at),
        sleft: StyledText::new(&text.sleft, fg.sleft),
        sright: StyledText::new(&text.sright, fg.sright),
    }
}

pub fn format(components: &Components<StyledText>, command_color: Color, style: Style) -> String {
    let command_color = format!(
        r"\[\e[38;2;{};{};{}m\]",
        command_color.0, command_color.1, command_color.2
    );
    let ps = match style {
        Style::Double => format!(
            r"{} {}{}{} {} {} {}\n{}",
            components.icon,
            components.user,
            components.at,
            components.host,
            components.sleft,
            components.loc,
            components.sright,
            components.prompt,
        ),
        Style::Extended => format!(
            "{} {}{}{}{}{} {}",
            components.icon,
            components.user,
            components.at,
            components.host,
            components.sright,
            components.loc,
            components.prompt,
        ),
        Style::Simple => format!(
            "{}{}{}{}{} {}",
            components.user,
            components.at,
            components.host,
            components.sright,
            components.loc,
            components.prompt,
        ),
        Style::Small => format!(
            "{}{}{} {}",
            components.user,
            components.sright,
            components.loc,
            components.prompt,
        ),
        Style::Micro => format!(
            "{} {}",
            components.icon,
            components.prompt,
        ),
        Style::Nano => format!(
            "{}",
            components.prompt,
        ),
    };
    format!("{ps}{RESET}{command_color}")
}
