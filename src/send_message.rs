use crate::user_thread::Message;
use reqwest::Error;

pub async fn main(
    commands: &mut Vec<Message>,
    version: &str,
    api_key: &str,
) -> Result<String, Error> {
    let request_url = "https://api.openai.com/v1/chat/completions";
    let mut messages = String::new();

    for (index, command) in commands.iter().enumerate() {
        messages.push_str(&format!(
            r#"{{"role": "{}", "content": "{}"}}"#,
            command.role, command.content
        ));
        if index != commands.len() - 1 {
            messages.push_str(", ");
        }
    }

    let body = format!(
        r#"{{ "model": "{}", "messages": [ {} ] }}"#,
        version, messages
    );

    let client = reqwest::Client::new();
    let res = client
        .post(request_url)
        .header("Content-Type", "application/json")
        .bearer_auth(api_key)
        .body(body)
        .send()
        .await?;
    let json = res.text().await?;
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    let response_message = json["choices"][0]["message"]["content"].as_str().unwrap();

    commands.push(Message::new(
        "assistant".to_string(),
        response_message.to_string(),
    ));

    Ok(response_message.to_string())
}
