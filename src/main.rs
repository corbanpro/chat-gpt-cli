mod read_command;

fn main() {
    let args = read_command::main();

    println!("message: '{}'", args.message);
    println!("version: '{}'", args.version);
}
