use std::env; //Standard library. To get arguments.
use std::process;
use rand::Rng; // Import the Rng trait

const version:&str="0.1.1";

fn main() {
    let mut rng = rand::thread_rng(); // Get a thread-local random number generator

    let args: Vec<String> = env::args().collect();
    let num_args = args.len();

    if num_args < 2 {
        println!("\nERROR! Insufficient arguments!\n");
        help();
    }

    let mut howmanyrolls: Vec<i8>=Vec::new();
    let mut howmanysides: Vec<i32>=Vec::new();
    let mut commandline:String;

    //create two lists: how many rolls, and how many sides associated with the roll.
    for item in (1..num_args) {
        let commandline=&args[item].to_lowercase();
        if commandline=="--help" {  //if you ask for help, break out of it.
            help();
        } else {
            let mut rolls:Vec<&str>=commandline.split('d').collect(); //split the command line
            howmanyrolls.push(rolls[0].parse().unwrap()); //number of rolls
            howmanysides.push(rolls[1].parse().unwrap()); //number of sides
        }

    }

    println!("Confirming this worked");
    for group in (1..num_args) {
        let mut rolltotal=0;
        println!("Rolling {}",&args[group]);      
        println!("\tRoll Count: {}",&howmanyrolls[group-1]);
        for roll in (0..(howmanyrolls[group-1])) {
            print!("\tRoll #{}: ",(roll+1));
            let result=rng.gen_range(1..(howmanysides[group-1]+1));
            printroll(result);
            rolltotal=rolltotal+result;
        }
        println!("Total for {}: {rolltotal}\n",&args[group]);
    }
}


fn printroll(result:i32) {
    println!("{result}");
}

//This function will be the help text. I'm not going to write anything until I get done.
fn help() {
    println!("diceroller v{version}");
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
}