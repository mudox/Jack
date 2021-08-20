use std::collections::HashMap;
use std::path::PathBuf;

use clap::crate_name;
use termion::color::{self, Color};
// use serde::Deserialize;

/// Return config dir path.
pub fn dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".config");
    path.push(crate_name!());
    path
}

pub struct Config {
    pub default_color: &'static dyn Color,
    pub colors: HashMap<String, &'static dyn Color>,

    pub default_symbol: String,
    pub symbols: HashMap<String, String>,
}

/// Program configuration.
impl Config {
    pub fn load() -> Config {
        let mut colors: HashMap<String, &'static dyn Color> = HashMap::new();
        colors.insert("error".to_string(), &color::Red);
        colors.insert("warn".to_string(), &color::Magenta);
        colors.insert("info".to_string(), &color::Yellow);
        colors.insert("debug".to_string(), &color::Green);
        colors.insert("verbose".to_string(), &color::White);
        colors.insert("success".to_string(), &color::Blue);

        let mut symbols = HashMap::new();
        symbols.insert("error".to_string(), "".to_string());
        symbols.insert("warn".to_string(), " ".to_string());
        symbols.insert("info".to_string(), " ".to_string());
        symbols.insert("debug".to_string(), "  ".to_string());
        symbols.insert("verbose".to_string(), "  ".to_string());
        symbols.insert("success".to_string(), " ".to_string());

        Config {
            default_color: &color::White,
            colors,

            default_symbol: "  ".to_string(),
            symbols,
        }
    }
}

impl Config {
    pub fn color(&self, scene: &str) -> &dyn Color {
        self.colors.get(scene).unwrap_or(&self.default_color)
    }

    pub fn symbol(&self, scene: &str) -> &str {
        self.symbols.get(scene).unwrap_or(&self.default_symbol)
    }
}
