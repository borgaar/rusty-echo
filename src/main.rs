use std::process;

fn main() {
    // Collect args
    let mut args: Vec<String> = std::env::args().collect();

    // Remove executable path from args
    args.remove(0);

    // Iterate and check whether any of the args is the `--help` flag
    if args.iter().any(|arg| arg == "--help") {
        print_help();
        process::exit(0);
    }

    // Join the args together into one string
    let args_combined = args.join(" ");

    // Echo the combined string back to the user
    println!("{}", args_combined);
}

fn print_help() {
    println!(
        r#"
Hi, and welcome to rusty-echo!

This is the unix `echo` command implemented in Rust for practice.
As of now there is not much to do as there is no flags other
than the `--help` flag.
        "#
    );
}
