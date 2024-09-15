use crate::colors::{Color, StyledText};
use crate::{Components, Config, RESET};
use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone, clap::ValueEnum)]
#[allow(non_camel_case_types)]
pub enum Style {
    double,
    extended,
    full,
    normal,
    micro,
    nano,
}

#[must_use]
pub fn format(config: &Config) -> String {
    let components = apply_colors(&config.text, &config.fg.clone().into());
    let command_color = Color::from_hex(&config.command.fg);
    let command_color = format!(
        r"\[\e[38;2;{};{};{}m\]",
        command_color.0, command_color.1, command_color.2
    );
    let icon = if config.show_error_icon {
        format!(
            "$([[ $? -eq 0 ]] && echo \"{}\" || echo \"{}\") ",
            components.icon_ok, components.icon_err
        )
    } else {
        String::new()
    };
    let ps = match config.style {
        Style::double => format!(
            r"{}{}{} {} {}\n{}",
            components.user,
            components.at,
            components.host,
            components.delim,
            components.loc,
            components.prompt,
        ),
        Style::extended => format!(
            "{}{}{} {} {} {}",
            components.user,
            components.at,
            components.host,
            components.delim,
            components.loc,
            components.prompt,
        ),
        Style::full => format!(
            "{}{}{}{}{} {}",
            components.user,
            components.at,
            components.host,
            components.delim,
            components.loc,
            components.prompt,
        ),
        Style::normal => format!(
            "{}{}{} {}",
            components.user, components.delim, components.loc, components.prompt,
        ),
        Style::micro => format!("{} {}", components.loc, components.prompt),
        Style::nano => format!("{}", components.prompt),
    };
    format!("{icon}{ps}{RESET}{command_color}")
}

fn apply_colors(text: &Components<String>, fg: &Components<Color>) -> Components<StyledText> {
    Components {
        user: StyledText::new(&text.user, fg.user),
        host: StyledText::new(&text.host, fg.host),
        loc: StyledText::new(&text.loc, fg.loc),
        prompt: StyledText::new(&text.prompt, fg.prompt),
        icon_ok: StyledText::new(&text.icon_ok, fg.icon_ok),
        icon_err: StyledText::new(&text.icon_err, fg.icon_err),
        at: StyledText::new(&text.at, fg.at),
        delim: StyledText::new(&text.delim, fg.delim),
    }
}
