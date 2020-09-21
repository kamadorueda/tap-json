extern crate clap;

// Standard libraries
use std::env;
use std::io;
use std::io::prelude::*;
use std::path;

// Third party libraries
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct Input {
    stream: String,
    record: serde_json::Value,
}

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

fn json_copy(json: &serde_json::Value) -> serde_json::Value {
    let as_string = serde_json::to_string(json).unwrap();
    let as_json = serde_json::from_str(&as_string).unwrap();

    return as_json;
}

fn json_load(data: &str) -> Result<Input, String> {
    let _: Input = match serde_json::from_str(&data) {
        Ok(json) => return Ok(json),
        Err(error) => {
            return Err(format!(
                "While loading JSON, data: {}, error: {}",
                data, error,
            ))
        }
    };
}

fn is_base_type(json: &serde_json::Value) -> bool {
    json.is_boolean() || json.is_null() || json.is_number() || json.is_string()
}

fn json_simplify(json: &mut serde_json::Value) {
    if is_base_type(&json) {
        // Nothing to modify
    } else if json.is_array() {
        for elem in json.as_array_mut().unwrap().iter_mut() {
            json_simplify(elem);
        }
    } else if json.is_object() {
        let mut should_recurse = false;
        let mut object_to_remove: Vec<String> = vec!();
        let mut object_to_insert: Vec<(String, serde_json::Value)> = vec!();

        for (key, val) in json.as_object_mut().unwrap() {
            if val.is_object() {
                should_recurse = true;
                object_to_remove.push(key.clone());
                for (sub_key, sub_val) in val.as_object().unwrap() {
                    object_to_insert.push((
                        format!("{}__{}", key, sub_key),
                        json_copy(sub_val),
                    ));
                };
            }
        }

        let object = json.as_object_mut().unwrap();
        for to_remove in object_to_remove {
            object.remove(&to_remove);
        }
        for (key, value) in object_to_insert {
            object.insert(key, value);
        }

        if should_recurse {
            json_simplify(json);
        }
    } else {
        *json = serde_json::Value::Null
    }
}

fn process() -> Result<(), String> {
    for result in io::stdin().lock().lines() {
        match result {
            Ok(line) => {
                let mut json = json_load(&line)?;
                let stream = json.stream;
                let record = &mut json.record;
                json_simplify(record);
                eprintln!("stream: {}, record: {}", stream, record);
            }
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
