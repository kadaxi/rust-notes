use rand::Rng;
use std::cmp::Ordering;
use std::io; // input/output library from std library (standard). // import rng method for random numbers. // enum type with variants (less, greater, equal). Cmp compare anything.

fn main() {
    println!("[!] Guessing game\n");

    // gen_range takes a range expression like "start..=end".
    let secret_number = rand::thread_rng().gen_range(1..=100); // random number from 1 to 100 using rand library.
    println!(">> Secret number: {secret_number}");

    loop {
        println!(">> Input your guess: ");

        // let tries to be an non typed variable (but can be typed).
        // ::new it's an associated function, creates a new empty instance of string.
        let mut guess = String::new(); // with mut, let could be mutable (change value).

        // io allows handle user input.
        io::stdin()
            // saying store input in guess variable (string). with & we define an reference value.
            .read_line(&mut guess) // returns result, enum type (for error handling like OK/ERR).
            .expect("Fail reading lines."); // if error exists (instance of result ERR).

        // trim eliminate whitespaces at beg and end (remove autonewline \n).
        // parse converts string to another type.
        // u:32 it's a reference for unsigned 32-bit integer, can store 2^32 values.
        // if value can't be parse the Result return Err (for that we use expect).
        let guess: u32 = guess.trim().parse().expect("Please type a number."); // redeclaring with shadowing.

        println!(">> You guessed: {guess}"); // print with variable.
                                             // comparing guess variable with secret number passing as reference.
                                             // match check for each ordering and compare the value until arm and value match, so program ends.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("** Too small."),
            Ordering::Greater => println!("** Too big."),
            Ordering::Equal => {
                println!("[!] You win.");
                break; // breaking loop, exiting main program.
            },
        }
    }
}
