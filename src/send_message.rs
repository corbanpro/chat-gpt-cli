use crate::user_thread::Message;
use serde_json::json;

pub async fn main(commands: &mut Vec<Message>, model: &str, api_key: &str) -> String {
    let request_url = "https://api.openai.com/v1/chat/completions";

    let body = json!({"model": model, "messages": commands}).to_string();

    let client = reqwest::Client::new();
    let res = client
        .post(request_url)
        .header("Content-Type", "application/json")
        .bearer_auth(api_key)
        .body(body)
        .send()
        .await
        .expect("Error sending request");

    let json = res.text().await.expect("Error reading response body");

    let json: serde_json::Value = match serde_json::from_str(&json) {
        Ok(json) => json,
        Err(err) => {
            return format!("Error parsing response json: {}", err).to_string();
        }
    };

    let response_message = json["choices"][0]["message"]["content"].as_str();
    let response_message = match response_message {
        Some(response_message) => response_message,
        None => {
            return format!("Error: No response from OpenAI\n\n{}", json).to_string();
        }
    };

    commands.push(Message::new("assistant".to_string(), response_message.to_string()));

    response_message.to_string()
}
