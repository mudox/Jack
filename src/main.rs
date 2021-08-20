use clap::{app_from_crate, Arg};

fn main() {
    let args = Args::parse();
    jack::print(&args.scene, &args.text);
}

pub struct Args {
    pub scene: String,
    pub text: String,
}

impl Args {
    pub fn parse() -> Args {
        let app = app_from_crate!()
            .arg(
                Arg::new("SCENE")
                    .about("The logging text")
                    .required(true)
                    .takes_value(true)
                    .possible_values(&["error", "warn", "info", "debug", "verbose", "success"]),
            )
            .arg(
                Arg::new("TEXT")
                    .about("The logging scene")
                    .required(true)
                    .takes_value(true),
            );

        let matches = app.get_matches();

        Args {
            scene: matches.value_of("SCENE").unwrap().to_string(),
            text: matches.value_of("TEXT").unwrap().to_string(),
        }
    }
}
