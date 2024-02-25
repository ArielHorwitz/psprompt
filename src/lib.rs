pub mod colors;
pub mod styles;

use colors::{Color, RESET};
use serde::Deserialize;
pub use styles::Style;

pub const CONFIG_TEMPLATE: &str = include_str!("config_template.toml");

#[derive(Debug, Deserialize, Clone)]
pub struct Components<T>
where
    T: Clone,
{
    pub user: T,
    pub host: T,
    pub loc: T,
    pub prompt: T,
    pub icon_ok: T,
    pub icon_err: T,
    pub at: T,
    pub delim: T,
}

impl<T> From<Components<T>> for Components<Color>
where
    T: std::fmt::Display + Clone,
{
    fn from(value: Components<T>) -> Self {
        Components {
            user: Color::from_hex(&value.user.to_string()),
            host: Color::from_hex(&value.host.to_string()),
            loc: Color::from_hex(&value.loc.to_string()),
            prompt: Color::from_hex(&value.prompt.to_string()),
            icon_ok: Color::from_hex(&value.icon_ok.to_string()),
            icon_err: Color::from_hex(&value.icon_err.to_string()),
            at: Color::from_hex(&value.at.to_string()),
            delim: Color::from_hex(&value.delim.to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub style: Style,
    pub show_error_icon: bool,
    pub ps0: Ps,
    pub ps2: Ps,
    pub ps3: Ps,
    pub ps4: Ps,
    pub text: Components<String>,
    pub fg: Components<String>,
    pub command: Command,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ps {
    pub text: String,
    pub color: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Command {
    pub fg: String,
}

pub fn read_config(path: &std::path::Path) -> Config {
    let config_text = std::fs::read_to_string(path).expect("read config file");
    toml::from_str(&config_text).expect("parse config toml")
}

fn get_ps(ps: &Ps) -> String {
    colors::StyledText::new(&ps.text, Color::from_hex(&ps.color)).to_string()
}

pub fn write(config: &Config) {
    println!(r"PS0='{}'", get_ps(&config.ps0));
    println!(r"PS1='{}'", styles::format(config));
    println!(r"PS2='{}'", get_ps(&config.ps2));
    println!(r"PS3='{}'", get_ps(&config.ps3));
    println!(r"PS4='{}'", get_ps(&config.ps4));
}
