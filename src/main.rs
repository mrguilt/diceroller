use std::env; //Standard library. To get arguments.
use std::process;
use rand::Rng; // Import the Rng trait

fn main() {
    let mut rng = rand::thread_rng(); // Get a thread-local random number generator

    let args: Vec<String> = env::args().collect();
    let num_args = args.len();

    if num_args < 2 {
        println!("\nERROR! Insufficient arguments!\n");
        help();
    }

 // This will iterate among the arguments. It will either add the roll to an array (lowering case), or
 // print the help or version message.   
 //   for arg in &args {
 //       print!("{arg}, ");
 //   }

    let commandline=&args[1].to_lowercase();
    if commandline == "--help" {
        help();
    }
    let mut rolls: Vec<&str>=commandline.split('d').collect();
    let mut rollcount:i32=rolls[0].parse().unwrap();
    let mut sides:i32=rolls[1].parse().unwrap();
    println!("Rolling a {sides}-sided die {rollcount} times:");
    let mut rolltotal=0;

    for i in (1..(rollcount+1)) {
        print!("\tRoll #{i}: ");
        let result=rng.gen_range(1..(sides+1));
        printroll(result);
        rolltotal=rolltotal+result;
    }
    println!("Total for {commandline}: {rolltotal}");
}

fn printroll(result:i32) {
    println!("{result}");
}

//This function will be the help text. I'm not going to write anything until I get done.
fn help() {
    println!("diceroller v0.1");
    println!("");
    println!("This program is a command line dice roller. To execute, type:");
    println!("\tdiceroller NdS");
    println!("\nWhere:\n");
    println!("\tN is the number of times to roll the die");
    println!("\tS is the number of sides on the die");
    println!("");
    println!("For instance:");
    println!("");
    println!("\tdicecroller 2d20");
    println!("");
    println!("Will roll a 20-sided die 2 times.");
    println!("");
    println!("Created by Charles Barilleaux (charles@mrguilt.com), September 2025");
    process::exit(0);   
}