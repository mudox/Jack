use crate::config::Config;
use termion::color::{Fg, Reset};

pub fn print(scene: &str, text: &str) {
    let config = Config::load();

    let symbol = config.symbol(scene);
    let color = config.color(scene);

    println!(
        "{color}{icon} {text}{reset}",
        color = Fg(color),
        icon = symbol,
        text = text,
        reset = Fg(Reset)
    );
}

// use `indicatif` for progress reporting
