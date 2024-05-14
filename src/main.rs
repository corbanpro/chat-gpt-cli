mod init;
mod send_message;
mod user_thread;

#[tokio::main]
async fn main() {
    let (version, secrets_path) = init::get_args();
    let api_key = init::get_api_key(&secrets_path).unwrap().unwrap();
    user_thread::main(&version, &api_key).await;
}
