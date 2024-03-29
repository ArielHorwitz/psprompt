use clap::Parser;
use psprompt::Style;
use std::path::PathBuf;

pub const ABOUT: &str = "Generate colorful prompts for the terminal.";
pub const LONG_ABOUT: &str = "Generate colorful prompts for the terminal.

## Generate the config:
psprompt --generate > ~/.config/psprompt.toml

## Apply to the current session only:
eval $(psprompt)

## Apply to new sessions:
echo 'eval $(psprompt)' >> ~/.bashrc";

#[derive(Debug, Parser)]
#[clap(name = "psprompt")]
#[clap(about = ABOUT, long_about = LONG_ABOUT)]
#[clap(version, author = "https://ariel.ninja")]
pub struct Cli {
    /// Config file (default: ~/.config/psprompt.toml)
    #[arg(long)]
    config: Option<PathBuf>,
    /// Print template config file
    #[arg(long)]
    generate: bool,
    /// Style
    #[arg(short, long)]
    style: Option<Style>,
    /// User name
    #[arg(short = 'U', long)]
    user: Option<String>,
    /// Host name
    #[arg(short = 'H', long)]
    host: Option<String>,
    /// Location
    #[arg(short = 'L', long)]
    location: Option<String>,
}

fn main() {
    let args = Cli::parse();
    if args.generate {
        println!("{}", psprompt::CONFIG_TEMPLATE);
        return;
    }
    let config_file = args.config.unwrap_or_else(|| {
        let home = std::env::var("HOME").expect("missing HOME environment variable");
        let path = PathBuf::from(&home);
        path.join(".config/psprompt.toml")
    });
    let mut config = psprompt::read_config(&config_file);
    if let Some(style) = args.style {
        config.style = style;
    };
    if let Some(user) = args.user {
        config.text.user = user;
    };
    if let Some(host) = args.host {
        config.text.host = host;
    };
    if let Some(location) = args.location {
        config.text.loc = location;
    };
    psprompt::write(&config);
}
