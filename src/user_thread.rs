use colored::Colorize;
use serde::{Deserialize, Serialize};
use termimad::crossterm::style::Color::*;

use crate::send_message;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn new(role: String, content: String) -> Self {
        Self { role, content }
    }
}

pub async fn main(model: &str, api_key: &str) {
    let line = "##############################################################################################################################";
    let mut skin = termimad::MadSkin::default();
    skin.paragraph.set_fg(Blue);

    println!(
        "{}\n\n\n{}\n\n\n{}\n\n",
        line.yellow(),
        "Hello! How can I assist you today?".blue(),
        line.yellow()
    );
    let mut commands: Vec<Message> = Vec::new();

    loop {
        let mut command = String::new();

        std::io::stdin().read_line(&mut command).expect("Error reading input");

        let command = command.trim();

        if command == "q" {
            break;
        }
        commands.push(Message::new("user".to_string(), command.to_string()));

        let response = send_message::main(&mut commands, model, api_key).await;

        println!(
            "\n\n{}\n\n\n{}\n\n{}\n\n",
            line.yellow(),
            skin.term_text(&response),
            line.yellow()
        );
    }
}
