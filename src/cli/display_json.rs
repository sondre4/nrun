use crate::cli::json_representation::WorkspaceRepresentation;

pub fn display_json(representation: &WorkspaceRepresentation) -> () {
    let serializing_result = serde_json::to_writer_pretty(std::io::stdout(), representation);

    match serializing_result {
        Ok(_result) => {}
        Err(error) => {
            println!("Error whilst serializing to JSON: {error}");
            return;
        }
    }

    // print to stdout
    println!();
}
