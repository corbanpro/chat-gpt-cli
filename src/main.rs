mod init;
mod send_message;
use colored::Colorize;

fn main() {
    let (version, secrets_path) = init::get_args();
    let api_key = init::get_api_key(&secrets_path).unwrap().unwrap();

    println!("\nchat gpt version: '{}'", version);
    println!("Enter message or enter \"q\" to exit:");
    loop {
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        if command == "q" {
            break;
        }
        let response = send_message::main(command, &version, &api_key);
        println!("\n{}\n", response.blue())
    }
}
