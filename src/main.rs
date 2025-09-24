use std::env; //Standard library. To get arguments.
use std::process;
use rand::Rng; // Import the Rng trait

const version:&str="1.0";   //Version number.

fn main() {
    let mut rng = rand::thread_rng(); // Get a thread-local random number generator

    let args: Vec<String> = env::args().collect();  //Retrieve arguments
    let num_args = args.len();

    if num_args < 2 {   //There has to be at least one argument. This stops the flow and prints the help info.
        println!("\nERROR! Insufficient arguments!\n");
        help();
    }

    let mut howmanyrolls: Vec<i8>=Vec::new();   //Vector to store how many times to roll a given die.
    let mut howmanysides: Vec<i32>=Vec::new();  //Vector to store how many sides are on a given die.
    let mut commandline:String;

    //create two lists: how many rolls, and how many sides associated with the roll.
    for item in (1..num_args) {
        let commandline=&args[item].to_lowercase(); //Smash case
        if commandline=="--help" {  //if you ask for help, show the help text and break out.
            help();
        } else if commandline=="--version" { //show the version, and break out.
            showversion();
        }else {
            let mut rolls:Vec<&str>=commandline.split('d').collect(); //split the command line
            howmanyrolls.push(rolls[0].parse().unwrap()); //Store the number of rolls
            howmanysides.push(rolls[1].parse().unwrap()); //Store the number of sides
        }

    }

    for group in (1..num_args) {    //go through each requested set of rolls
        let mut rolltotal=0;    //store the roll total
        println!("Rolling {}",&args[group]);      
        for roll in (0..(howmanyrolls[group-1])) {
            print!("\tRoll #{}: ",(roll+1));
            let result=rng.gen_range(1..(howmanysides[group-1]+1)); //Pick the random number
            printroll(result);
            rolltotal=rolltotal+result; //add to the total
        }
        println!("Total for {}: {rolltotal}\n",&args[group]); //Show the sum of all rolls.
    }
}

//This just prints the results. I may get fancy at a later date. 
fn printroll(result:i32) { 
    println!("{result}");
}

//This function will be the help text. I'm not going to write anything until I get done.
fn help() {
    println!("diceroller v{version}");
    println!("");
    println!("This program is a command line dice roller. To execute, type:");
    println!("\tdiceroller CdS [CdS...]");
    println!("\nWhere:\n");
    println!("\tC is the number of times to roll the die");
    println!("\tS is the number of sides on the die");
    println!("");
    println!("For instance:");
    println!("");
    println!("\tdicecroller 2d20");
    println!("");
    println!("will roll a 20-sided die 2 times.");
    println!("");
    println!("You can roll multiple dice at once. Typing");
    println!("");
    println!("\tdiceroller 2d20 1d6");
    println!("");
    println!("will roll a 20-sided die 2 times, and a 6-sided die once.");
    println!("");
    println!("Other options:");
    println!("\t--help\t\tThis help information");
    println!("\t--version\tVersion information");
    println!("");
    println!("Created by Charles Barilleaux (charles@mrguilt.com), September 2025");
    process::exit(0);
}

//Print version number, and, because of my ego, who wrote it.
fn showversion() {
    println!("\ndiceroller v{version}");
    println!("");
    println!("Created by Charles Barilleaux (charles@mrguilt.com), September 2025");
    process::exit(0);
}