use std::env; //Standard library. To get arguments.
use std::process;
use rand::Rng; // Import the Rng trait

const VERSION:&str="1.5.1";   //VERSION number.

fn main() {
    let mut rng = rand::thread_rng(); // Get a thread-local random number generator

    let mut args: Vec<String> = env::args().collect();  //Retrieve arguments
    let mut num_args = args.len();
    let mut mode=2; //default mode. Prints verbose.

    //Modes!
        //0     No headers, roll count, etc. Just results, broken by argument
        //2     DEFAULT. Shows what you're rolling, roll count, and a summary
        //3     CSV
        //4     Pretty

    if num_args < 2 {   //No arguments? Roll a verbose D20
        print!("No arguments! Accpeting a default. ");
        args.push("1d20".to_string());
        num_args=num_args+1; //Otherwise, the loops below would break. This is a hack I'll fix later.
    }

    //create two lists: how many rolls, and how many sides associated with the roll.
    let mut howmanyrolls: Vec<i8>=Vec::new();   //Vector to store how many times to roll a given die.
    let mut howmanysides: Vec<i32>=Vec::new();  //Vector to store how many sides are on a given die.

    let mut commandline:String;

    //Parse the command line
    for item in (1..num_args) {
        let commandline=&args[item].to_lowercase(); //Smash case
        if commandline=="--help" {  //if you ask for help, show the help text and break out.
            help();
        } else if commandline=="--version" { //show the version, and break out.
            showversion();
        } else if commandline=="--silent" {
            mode=0;
            num_args=num_args-1; //Otherwise, the loops below would break. This is a hack I'll fix later.
                                 //(Temporary and permanent are synonyms.)
        } else if commandline=="--csv" {
            mode=3;
            num_args=num_args-1; //Otherwise, the loops below would break. This is a hack I'll fix later.
                                 //(Temporary and permanent are synonyms.)       
        } else if commandline=="--pretty" {
            mode=4;
            num_args=num_args-1; //Otherwise, the loops below would break. This is a hack I'll fix later.
                                 //(Temporary and permanent are synonyms.)       
        } else {
            let mut rolls:Vec<&str>=commandline.split('d').collect(); //split the command line
            howmanyrolls.push(rolls[0].parse().unwrap()); //Store the number of rolls
            howmanysides.push(rolls[1].parse().unwrap()); //Store the number of sides
        }

    }

    //The easy error handling: if only a format was specified.
    if howmanyrolls.len()<1 {
        println!("No dice specified. Using default.");
        howmanyrolls.push(1);
        howmanysides.push(20);
        num_args=num_args+1; //that same damn hack. Sure I'll fix it later.
        args[1]="1d20".to_string(); //Said what the default is (for CSV mode)
    }

    //mode=4; //set mode for testing
    for group in (1..num_args) {    //go through each requested set of rolls
        let mut rolltotal=0;    //store the roll total
        rollheader(&args[group],mode);
        for roll in (0..(howmanyrolls[group-1])) {
            let result=rng.gen_range(1..(howmanysides[group-1]+1)); //Pick the random number
            printroll(result,((roll+1)).into(),mode);
            rolltotal=rolltotal+result; //add to the total
        }
        rollsummary(&args[group],rolltotal,mode);
    }
}

//Prints the results, based on mode.
fn printroll(result:i32,rollno:i32,mode:i8) { 
    if mode==0 {
        println!("{result}"); //Mode 0: Just prints the rolls, line break between arguments. Allows a 1dS to feed into other things.
    } else if mode==2 {
        println!("\tRoll #{rollno}: {result}");
    } else if mode==3 { //CSV mode
        print!(",{}",result);
    } else if mode==4 { //Pretty mode
        printdice(result);
    } else {
        println!("INVALID OUTPUT MODE!"); //Fails if a mode number not defined is used.
        process::exit(0);
    }
}

//Prints the results header. Mode sensitive.
fn rollheader(description:&str, mode:i8) { 
    if mode==2 {
        println!("Rolling {}",description);      
    } else if mode==3 {     //CSV mode. First column would be what you requested
        print!("{}",description);
    }
}

fn rollsummary(description:&str,total:i32,mode:i8) {
        if mode==2 {
            println!("Total for {description}: {total}\n");
        } else {            //Just a line break.
            println!("");
        }
}

//This function will print the help text. 
fn help() {
    println!("diceroller v{VERSION}");
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
    println!("\t--silent\tOnly prints results--no headers, roll count, etc.");
    println!("\t--csv\t\tCreates an output to redirect to a CSV file (Excel import)");
    println!("\t--pretty\tASCII art dice");

    println!("");
    process::exit(0);
}

//Print version number, and, because of my ego, who wrote it.
fn showversion() {
    println!("\ndiceroller v{VERSION}");
    println!("");
    println!("Created by Charles Barilleaux (charles@mrguilt.com), September 2025");
    process::exit(0);
}

//////////////////////////////////////
// Pretty Dice
// All of this is to generate "pretty" looking ASCII-art dice. It's broken into five functions.

//Utility function. Will print the same character so-many times without a line break.
fn printrepeat(what:char,count:i8) {
    for i in (0..count) {
        print!("{what}");
    }
}

//Each of the next four functions prints part of the dice. They are called in a sequence by
//printdice() to put it all together.
//  _____  
// /     \ caprow()
// |     | blankrow()
// |  x  | resultsrow()
// |     | blankrow()
// \_____/ bottomrow()

fn caprow(width:i8) {
    print!(" ");
    printrepeat('_',width);
    println!("");
    print!("/");
    printrepeat(' ',width);
    println!("\\");
}

fn bottomrow(width:i8) {
//    print!(" ");
//    printrepeat('_',width);
//    println!("");
    print!("\\");
    printrepeat('_',width);
    println!("/");
}

fn blankrow(width:i8) {
    print!("|");
    printrepeat(' ',width);
    println!("|");    
}

//This prints the row in the middle of the dice. A bit of math is required to (roughly) center it. 
fn resultrow(result:i32,width:i8) {
    let resultsize:i8=result.to_string().len().try_into().unwrap();
    let backpadding:i8=(width-resultsize)/2; //how much space to put behind the result
    let mut frontpadding=backpadding;
    if (resultsize%2) == 0 {
        frontpadding=frontpadding+1;
    } 
    print!("|");
    printrepeat(' ',frontpadding);
    print!("{result}");
    printrepeat(' ',backpadding);
    println!("|");
}

fn printdice(result:i32) {
    let mut width:i8=7; //Width and height of the dice. I was going to vary 
                        //the dice size by the size of the result. but I couldn't 
                        //make a 5x5 die look right. 
    caprow(width);
    for i in (0..(width/4)) {
        blankrow(width);
    }
    resultrow(result,width);
    for i in (0..(width/4)) {
        blankrow(width);
    }
    bottomrow(width);
}