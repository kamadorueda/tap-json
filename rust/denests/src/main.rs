extern crate clap;

// Standard libraries
use std::env;
use std::fs;
use std::path;

// Third party libraries
use clap::{App, Arg};
use uuid::Uuid;

struct Config {
    timestamps_detection: bool,
    records_dir: path::PathBuf,
    schemas_dir: path::PathBuf,
}

fn generate_uuid() -> String {
    return Uuid::new_v4().hyphenated().to_string();
}

fn create_temp_dir() -> path::PathBuf {
    let mut dir = env::temp_dir();
    dir.push(generate_uuid());

    return dir;
}

fn parse_arguments() -> Config {
    let matches = App::new("Denests")
        .about("Receive a json stream on stdin, outputs a json stream on stdout")
        .arg(
            Arg::with_name("timestamps_detection")
                .long("--timestamps-detection")
                .help("Enable UNIX timestamps detection"),
        )
        .get_matches();

    return Config {
        timestamps_detection: matches.occurrences_of("timestamps_detection") > 0,
        records_dir: create_temp_dir(),
        schemas_dir: create_temp_dir(),
    };
}

fn main() {
    let config = parse_arguments();

    eprintln!("timestamps_detection: {}", config.timestamps_detection);
    eprintln!("records_dir: {}", config.records_dir.display());
    eprintln!("schemas_dir: {}", config.schemas_dir.display());
}
