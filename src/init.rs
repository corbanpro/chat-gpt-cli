use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;

pub fn get_args() -> (String, String) {
    let mut version = String::from("gpt-3.5-turbo");
    let mut secrets_path = String::from("C:/Users/corba/secrets.json");

    let mut args = env::args().skip(1);

    while let Some(word) = args.next() {
        if word == "-v" {
            version = args.next().expect("Please input a version");
            let valid_versions = vec!["gpt-3.5-turbo", "gpt-4"];
            if !valid_versions.contains(&&version[..]) {
                panic!(
                    "Please input a valid version. Versions:\n{:?}",
                    valid_versions
                )
            }
            continue;
        } else if word == "-f" {
            secrets_path = args.next().expect("Please input a file path");
            continue;
        } else {
            panic!("Invalid argument: {}", word);
        }
    }

    let args: (String, String) = (version, secrets_path);
    args
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Secrets {
    openai_api_key: String,
}

pub fn get_api_key(secrets_path: &str) -> Result<serde_json::Result<String>, io::Error> {
    let contents = fs::read_to_string(secrets_path);
    match contents {
        Ok(contents) => {
            let v: Secrets = serde_json::from_str(&contents)?;
            Ok(Ok(v.openai_api_key))
        }
        Err(err) => Err(err),
    }
}
