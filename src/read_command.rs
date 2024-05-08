use std::env;
#[derive(Clone)]
pub struct Args {
    pub message: String,
    pub version: String,
}

impl Args {
    fn new() -> Args {
        Args {
            message: String::new(),
            version: String::new(),
        }
    }
    fn add_message(&mut self, message: &str) -> Args {
        self.message = String::from(message);
        self.clone()
    }
    fn add_version(&mut self, version: &str) -> Args {
        self.version = String::from(version);
        self.clone()
    }
}

pub fn main() -> Args {
    let mut message = String::new();
    let mut version = String::from("gpt-3.5-turbo");

    let mut args = env::args().skip(1);

    while let Some(word) = args.next() {
        if word == "-v" {
            version = args.next().unwrap();
            let valid_versions = vec!["gpt-3.5-turbo", "gpt-4"];
            if !valid_versions.contains(&&version[..]) {
                panic!(
                    "Please input a valid version. Versions:\n{:?}",
                    valid_versions
                )
            }
            continue;
        }
        message.push_str(&word);
        message.push_str(" ");
    }

    if let Some(last_char) = message.chars().last() {
        if last_char == ' ' {
            message = message[..message.len() - 1].to_string()
        }
    }

    Args::new().add_message(&message).add_version(&version)
}
