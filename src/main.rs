use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "The unix `echo` command rewritten in Rust")]
struct Echo {
    #[arg()]
    args: Vec<String>,
}

fn main() {
    // Parse the command line arguments
    let echo = Echo::parse();

    // Combine the arguments into a single string
    let combined_args = echo.args.join(" ");

    // Replace the escaped newline character with an actual newline
    let processed_args = combined_args.replace("\\n", "\n");

    // Print the processed arguments
    println!("{}", processed_args);
}
