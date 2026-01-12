# Overview

Rust is becoming a relevant programming language especially nowadays since there is a demand for performance and security. 
Learning this programming language and mastering its complexities is an essential skill so we can become relevant in the current job market. 

This program is a game written in Rust that works through the command line.
The game allows the user to choose two game modalities. 
1. the user guesses the number generated randomly by the computer. 
2. The machine has to guess the user's number.

The following code stores the user input in the game_mode variable. The io::stdin handles the user input. The read_line command will read the input.
The expect command will trigger an error message if the reading input fails.
```
let mut game_mode: String = String::new();
io::stdin()
   .read_line(&mut game_mode)
   .expect("Unexpected Error! Please restart program!");
```
The variable attempts is mutable therefore it will change according to the number of guesses:
```
let mut attempts = 0;
```
External Crates
```
use rand::Rng;
```
We use the utility 'Range' to generate a number between 1 and the max user value
```
let secret_number = rand::thread_rng().gen_range(1..=max_uservalue);
```   
Pattern Matching: This code will trim the white space before or after the user input. It will then convert the user input from string into an unsigned integer.
If the conversion succeeds, it stores the number otherwise it will return an error.
```
let user_input: u32 = match user_input.trim().parse() {
    Ok(num) => num, // If parsing succeed, use the number
    Err(_) => {
        println!("Please enter a positive integer");
        continue; // Goes to the next loop iteration if the user enters an invalid number, the number of attempts is not affected.
    }
};
```

The purpose for writing this program is to internalize and put into practice Rust's specifics unique features such as 
Ownership Model and Borrow Checker, variables default immutability, and error handling.


[Software Demo Video](https://youtu.be/w9hEDvcZCgs)

# Development Environment

For this program, I have mainly used Rustrover by Jetbrains https://www.jetbrains.com/rust/
Rustrover is a great IDE for dealing with Rust programming features.

As a programming language I have used Rust. Rust is a system programming language like C and C++.
It uses a compiler which translates what we write into machine code. It provides the perfect balance in speed, security, portability, 
and concurrency; therefore, it is not surprising that 80% of programmers would use Rust again (stats provided by Stack Overflow). 
For this project, I have used the Cargo package manager.
Additionally, I have used libraries such as rand::Rng, std::collections::HashMap, and std::io.
The library (Utility) rand::Rng is implemented to generate random numbers.
The std::collections::HashMap works like dictionaries and maps; therefore, it is used to store keys and values. In this project
the keys are represented by level of difficulty (ex. E, M, D) and the values by the number of attempts (ex. 10, 5, 3).
The std::io module handles the Read and Write inputs from the keyboard.

 
# Useful Websites

- [Rand Documentation](https://docs.rs/rand/latest/rand/index.html)
- [RustDocumentation](https://rust-lang.org/)
- [YouTube: Rust Programming Full Course](https://www.youtube.com/watch?v=rQ_J9WH6CGk&t=4219s)

# Future Work

- Implement a user interface.
- Advanced game mechanics
- Score history 
- Player vs player