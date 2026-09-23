mod command;
mod paths;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    command::handle_command(arguments);

    // println!("adacacadaArguments: {:?}", arguments);
}
