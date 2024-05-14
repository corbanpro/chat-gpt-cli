pub fn main(command: &str, version: &str, api_key: &str) -> String {
    let response = String::from(format!(
        "You entered: '{}'\nVersion: '{}'\nAPI key: '{}'",
        command, version, api_key
    ));
    response
}
