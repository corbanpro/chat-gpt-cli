use colored::Colorize;

use crate::send_message;

pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn new(role: String, content: String) -> Self {
        Self { role, content }
    }
}

pub async fn main(version: &str, api_key: &str) {
    println!("Enter message or type \"q\" to exit:\n");
    println!("{}", "Hello! How can I assist you today?\n".blue());
    let mut commands: Vec<Message> = Vec::new();

    loop {
        let mut command = String::new();
        match std::io::stdin().read_line(&mut command) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                continue;
            }
        }
        let command = command.trim().to_string();

        if command == "q" {
            break;
        }
        commands.push(Message::new("user".to_string(), command.clone()));

        let response = send_message::main(&mut commands, &version, &api_key)
            .await
            .unwrap_or_else(|err| {
                eprintln!("Error sending message: {}", err);
                "Error: No response from OpenAI".to_string()
            });
        println!("\n{}\n", response.blue())
    }
}
