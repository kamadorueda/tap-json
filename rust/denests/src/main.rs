extern crate clap;

// Standard libraries
use std::env;
use std::io;
use std::io::prelude::*;
use std::path;

// Third party libraries
use clap::{App, Arg};
use serde_json;
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
    let cli = App::new("Denests")
        .about("Receive a json stream on stdin, outputs Singer to stdout")
        .arg(
            Arg::with_name("timestamps_detection")
                .long("--timestamps-detection")
                .help("Enable UNIX timestamps detection"),
        )
        .get_matches();

    return Config {
        timestamps_detection: cli.occurrences_of("timestamps_detection") > 0,
        records_dir: create_temp_dir(),
        schemas_dir: create_temp_dir(),
    };
}

fn load_json(data: &str) -> Result<serde_json::Value, String> {
    let _: serde_json::Value = match serde_json::from_str(&data) {
        Ok(json) => return Ok(json),
        Err(error) => return Err(format!(
            "While loading JSON, data: {}, error: {}", data, error,
        )),
    };
}

fn process() -> Result<(), String> {
    for result in io::stdin().lock().lines() {
        match result {
            Ok(line) => {
                let json = load_json(&line)?;
                eprintln!("{}", json)
            },
            Err(error) => return Err(error.to_string()),
        }
    }

    return Ok(());
}

fn main() {
    let config = parse_arguments();

    eprintln!("timestamps_detection: {}", config.timestamps_detection);
    eprintln!("records_dir: {}", config.records_dir.display());
    eprintln!("schemas_dir: {}", config.schemas_dir.display());

    match process() {
        Ok(_) => eprintln!("Successfully processed stream!"),
        Err(e) => eprintln!("An error ocurred: {}", e),
    };
}
