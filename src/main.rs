use std::io;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let game_options = ["rock", "paper", "scissors"];

    let mut options_dictionary = HashMap::new();
    options_dictionary.insert("rock", "rock");
    options_dictionary.insert("r", "rock");
    options_dictionary.insert("paper", "paper");
    options_dictionary.insert("p", "paper");
    options_dictionary.insert("scissors", "scissors");
    options_dictionary.insert("s", "scissors");

    println!("Welcome to rock paper scissors game!");
    println!("To play the game type what shape you choose");

    loop {
        // Handle user input
        let mut user_input = String::new();
        println!("Type next choice:");
        io::stdin()
            .read_line(& mut user_input)
            .expect("Failed to read user input!");
        user_input.make_ascii_lowercase();
        let mut user_choice = user_input.trim();
        if options_dictionary.contains_key(user_choice)  {
            if !game_options.contains(&user_choice) {
                user_choice = options_dictionary[user_choice];
            }
            // How does he know if there's continue (if we remove cont, it gives error at line 43)
            // However, if we put smth like 'if true { continue; }' then it gives an error as well
            else {
                continue;
            }
        }
        else {
            println!("Please, write 'rock', 'paper', or 'scissors' (or 'r', 'p', 's') (not case sensitive)");
            continue;
        }
        
        user_input.clear();
        println!("{}", user_choice);
        // Generate another 
        // Compare
        // print score & offer to play more
    }
}
