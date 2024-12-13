use clap::Parser;
use colored::Colorize;
use home::home_dir;
use std::fs;

mod send_message;
mod user_thread;

#[derive(Parser, Debug)]
#[command(name = "cgpt")]
#[command(author = "Corban Procuniar <corbanpro@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Interface with ChatGpt with the command line", long_about = None)]
struct Args {
    #[arg(short, long, value_name = "MODEL", default_value = "gpt-4o-mini")]
    model: String,

    #[arg(short = 'p', long, value_name = "KEY", default_value = "~/.secrets/OPENAI_API_KEY")]
    api_key_path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let model = args.model;
    let home_directory = home_dir().unwrap().display().to_string();
    let api_key_path = args.api_key_path;
    let api_key_path = api_key_path.replace("~", &home_directory);

    let api_key_res = fs::read_to_string(&api_key_path);

    if let Err(err) = api_key_res {
        println!("\n{}\n", "Error opening API key file: ".red());
        println!("{}", err);
        println!("File attempted: {}", &api_key_path);
        println!("\nPut plaintext api key in ~/.secrets/OPENAI_API_KEY or specify file path\n");
        return;
    }

    let api_key = api_key_res.unwrap();
    let api_key = api_key.trim();

    user_thread::main(&model, api_key).await;
}
