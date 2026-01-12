// A simple guess number game built in Rust
// The program randomly generate a number between 1 and 10
// The user has a number of attempts to guess the number
// After each guess the app tells the user if the number it too high or too low, or correct.

use rand::Rng; // Import the Random number generator from the rand crate // Ref. https://docs.rs/rand/latest/rand/index.html
use std::collections::HashMap; // data structure similar to dictionaries or maps // Ref: https://doc.rust-lang.org/rust-by-example/std/hash.html
use std::io; // Import the standard I/O module // Ref. https://doc.rust-lang.org/std/io/index.html


fn main() {
    loop {
        run_game();
        println!("Would you like to play again (yes/no)?");
        println!("Press Y to play again or press any other key to quit!");
        let mut playagain = String::new();
        io::stdin()
            .read_line(&mut playagain)
            .expect("Unexpected Error! Please restart program!"); // If reading input succeeds → nothing happens, program continues

        if playagain.trim().to_uppercase() != "Y" {
            break;
        }
    }
}

fn run_game() {
    let mut max_uservalue: u32 = 0;
    println!("Welcome to the Guess the Number Game!");
    println!(
        "Enter 1 if you want to be the one who guesses the number, or enter 2 if you want the machine to guess your number."
    );

    // The loop keeps working until we tell it to stop
    let game_mode: u32 = loop {
        // The user chooses option 1 or option 2 game mode
        let mut game_mode: String = String::new();

        // Read the game mode from the standard input (keyboard entry)
        // .expect() will crash the program if the reading fails
        io::stdin()
            .read_line(&mut game_mode)
            .expect("Unexpected Error! Please restart program!"); // If reading input succeeds → nothing happens, program continues

        match game_mode.trim().parse::<u32>() {
            Ok(num) if num == 1 || num == 2 => break num,
            Ok(_) => {
                println!("Please enter a valid number (1 or 2).")
            }
            Err(_) => {
                println!("Only 1 and 2 are valid.")
            }
        }
    };

    println!("Please choose a level of difficulty. \nE = Easy, M = Medium, and D = Difficult");

    let game_difficulty: String = loop {
        // The user chooses the level of difficulty "E", "M", and "D"
        // E = Easy, M = Medium, and D = Difficult
        let mut game_difficulty: String = String::new();

        // Read the level of difficulty from the standard input (keyboard entry)
        // ,expect() will crash the program if the reading fails (ex. input stream fails)
        io::stdin()
            .read_line(&mut game_difficulty)
            .expect("Unexpected Error! Please restart program!"); // If reading input succeeds → nothing happens, program continues

        // Change any user input to uppercase
        match game_difficulty.trim().to_uppercase() {
            letter => {
                if letter == "E" || letter == "M" || letter == "D" {
                    break letter;
                } else {
                    println!("Please enter a valid letter (E or M or D).")
                }
            }
            _ => {
                println!("Please enter a valid letter (E or M or D).")
            }
        }
    };

    println!("Game Difficulty {}", game_difficulty);

    println!("Enter a max value number that you want to guess - it must be greater than 10");

    // The loop keeps working until we tell it to stop
    max_uservalue = loop {
        // The user chooses the max value
        let mut max_value: String = String::new();

        // Read the max value from the standard input (keyboard entry)
        // .expect() will crash the program if the reading fails
        io::stdin()
            .read_line(&mut max_value)
            .expect("Unexpected Error! Please restart program!");

        match max_value.trim().parse::<u32>() {
            Ok(num) if num > 10 => break num,
            Ok(_) => {
                println!("Please enter a valid number greater than 10.")
            }
            Err(_) => {
                println!("Only numbers above 10 are valid")
            }
        }
    };

    let mut difficulty = HashMap::new();

    difficulty.insert(String::from("E"), 10);
    difficulty.insert(String::from("M"), 5);
    difficulty.insert(String::from("D"), 3);
    let get_difficulty = difficulty.get(&game_difficulty).copied().unwrap_or(0);

    // Call the appropriate game function based on the selected game mode.
    if game_mode == 1 {
        mode1(get_difficulty, max_uservalue)
    } else if game_mode == 2 {
        mode2(get_difficulty, max_uservalue)
    }
}

fn mode1(game_difficulty: u32, max_uservalue: u32) {
    // Generate a random number between 1 and the user max value (inclusive)
    // thread_rng() provides a random number generator local to this thread
    // gen_range(1..=10) means numbers from 1 up to user max value and including the variable.
    let secret_number = rand::thread_rng().gen_range(1..=max_uservalue);

    // Track the number of attempts of the user
    let mut attempts = 0;

    // Explain the game to the user
    println!(
        "Guess a number from 1 to the user max value! You have {} chances to guess it. Good Luck!",
        game_difficulty
    );

    // Start a loop that runs until the user makes 5 guesses.
    while attempts < game_difficulty {
        println!("Please enter your guess: ");

        // Create a new mutable String to store the input
        let mut user_input = String::new();

        // Read the user input from the standard input (keyboard entry)
        // .expect() will crash the program if the reading fails
        io::stdin()
            .read_line(&mut user_input)
            .expect("Unexpected Error! Please restart program!");

        // Convert (Parse) the string input into an integer (u32)
        // trim() the string to remove white spaces
        // 'match' handles cases where the user doesn't type a valid number
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num, // If parsing succeed, use the number
            Err(_) => {
                println!("Please enter a positive integer");
                continue; // Goes to the next loop iteration if the user enters an invalid number, the number of attempts is not affected.
            }
        };
        // Increase the number of attempts
        attempts += 1;

        // Compare the user guess with the secret number
        if user_input == secret_number {
            println!("You win! The number {} is correct", secret_number);
            return;
        } else if user_input > secret_number {
            println!("{} is too high! Try again", user_input);
        } else {
            println!("{} is too low! Try again", user_input);
        }
    }
    // If the user reaches the limit of attempts, the game ends with a failure message.
    println!(
        "Sorry! You lose. {} was the number! You have reached the limit of {} attempts",
        secret_number, attempts
    );
}

fn mode2(game_difficulty: u32, max_uservalue: u32) {
    // Track the number of attempts of the user
    let mut attempts = 0;

    // Generate an array of numbers from 1 to max_uservalue
    // Vec is a growable array
    // collect fills the array with numbers
    let mut possible_numbers: Vec<u32> = (1..=max_uservalue).collect();
    println!("Think a number from 1 to the user max value (inclusive)!");
    println!("Guess a number will now try to guess your number.");

    while attempts < game_difficulty {
        let half_array = possible_numbers.len() / 2;
        println!(
            "Is {} the number? Type Y for yes and N for no ",
            possible_numbers[half_array] // index
        );

        let user_approval: String = loop {
            // The user chooses the yes or no
            let mut user_approval: String = String::new();

            // Read if the user entered yes or no from the standard input (keyboard entry)
            // ,expect() will crash the program if the reading fails
            io::stdin()
                .read_line(&mut user_approval)
                .expect("Unexpected Error! Please restart program!");

            // Change any user input to lowercase
            match user_approval.trim().to_uppercase() {
                letter => {
                    if letter == "Y" || letter == "N" {
                        break letter;
                    } else {
                        println!("Please enter a valid letter (Y or N).")
                    }
                }
                _ => {
                    println!("Please enter a valid letter (Y or N).")
                }
            }
        };
        if user_approval == "Y" {
            println!("The machine won!");
            return;
        } else {
            attempts += 1;
            println!("Is the number you thought higher than {}? Type Y/N", possible_numbers[half_array]);

            let higher_lower: String = loop {
                // The user chooses the yes or no
                let mut higher_lower: String = String::new();

                // Read if the user entered yes or no from the standard input (keyboard entry)
                // ,expect() will crash the program if the reading fails
                io::stdin()
                    .read_line(&mut higher_lower)
                    .expect("Unexpected Error! Please restart program!");

                // Change any user input to lowercase
                match higher_lower.trim().to_uppercase() {
                    letter => {
                        if letter == "Y" || letter == "N" {
                            break letter;
                        } else {
                            println!("Please enter a valid letter (Y or N).")
                        }
                    }
                    _ => {
                        println!("Please enter a valid letter (Y or N).")
                    }
                }
            };

            // Slice vector to narrow down the possible values.
            possible_numbers = if higher_lower == "Y" {possible_numbers[half_array..].to_vec()} else {possible_numbers[..half_array].to_vec()};
        }
    }

    println!("Oops! The machine was not able to guess your number.");
}
