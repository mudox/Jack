use crate::config::Config;
use termion::{
    clear,
    color::{Fg, Reset},
};

pub fn print(scene: &str, text: &str, before: usize, after: usize) {
    let config = Config::load();

    let symbol = config.symbol(scene);
    let color = config.color(scene);

    let before = format!("{0:\n<1$}", "", before);
    let after = format!("{0:\n<1$}", "", after);

    reset();
    println!(
        "{before}{color}{icon} {text}{reset}{after}",
        before = before,
        color = Fg(color),
        icon = symbol,
        text = text,
        reset = Fg(Reset),
        after = after,
    );
}

fn reset() {
    print!("\r{}", clear::CurrentLine);
}

pub fn progress(text: &str) {
    reset();
    print!("{}", text);
}
