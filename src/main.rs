use std::env; //Standard library. To get arguments.
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_args = args.len();

    if num_args < 2 {
        println!("\nERROR! Insufficient arguments!\n");
        help();
    }

    let commandline=&args[1].to_lowercase();
    if commandline == "--help" {
        help();
    }
    println!("First Argument: {commandline}");
}

fn help() {
    println!("This is a placeholder for the help text.");
    process::exit(0);
}