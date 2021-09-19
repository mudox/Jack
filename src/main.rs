use clap::{app_from_crate, Arg};

fn main() {
    let args = Args::parse();
    if args.scene == "progress" {
        jack::progress(&args.text);
    } else {
        jack::print(&args.scene, &args.text, args.before, args.after);
    }
}

pub struct Args {
    pub scene: String,
    pub text: String,
    pub before: usize,
    pub after: usize,
}

impl Args {
    pub fn parse() -> Args {
        let app = app_from_crate!()
            .arg(
                Arg::new("SCENE")
                    .about("The logging text")
                    .required(true)
                    .takes_value(true)
                    .possible_values(&[
                        "error", "warn", "info", "debug", "verbose", "success", "progress",
                    ]),
            )
            .arg(
                Arg::new("TEXT")
                    .about("The logging scene")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::new("before")
                    .short('b')
                    .long("before")
                    .about("Number of new lines before")
                    .takes_value(true)
                    .value_name("B")
                    .validator(is_count),
            )
            .arg(
                Arg::new("after")
                    .short('a')
                    .long("after")
                    .about("Number of new lines after")
                    .takes_value(true)
                    .value_name("A")
                    .validator(is_count),
            );

        let matches = app.get_matches();

        Args {
            scene: matches.value_of("SCENE").unwrap().to_string(),
            text: matches.value_of("TEXT").unwrap().to_string(),
            before: matches.value_of_t("before").unwrap_or(0),
            after: matches.value_of_t("after").unwrap_or(0),
        }
    }
}

fn is_count(v: &str) -> Result<(), String> {
    match v.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Invalid number string")),
    }
}
