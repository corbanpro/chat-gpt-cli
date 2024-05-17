mod init;
mod send_message;
mod user_thread;

#[tokio::main]
async fn main() {
    let (version, secrets_path) = init::get_args();
    let api_key = match init::get_api_key(&secrets_path) {
        Ok(api_key) => match api_key {
            Ok(api_key) => api_key,
            Err(err) => panic!("Error parsing secrets file json: {}", err),
        },
        Err(err) => panic!("Error reading/opening secrets file: {}", err),
    };

    user_thread::main(&version, &api_key).await;
}
