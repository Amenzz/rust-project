use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Rock, Paper, Scissors!");
    println!("Type 'rock', 'paper', or 'scissors' to play. Type 'quit' to exit.");

    loop {
        // Get user input
        let mut user_input = String::new();
        println!("Enter your choice:");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input = user_input.trim().to_lowercase();

        // Check if the user wants to quit
        if user_input == "quit" {
            println!("Thanks for playing! Goodbye.");
            break;
        }

        // Validate user input
        if !["rock", "paper", "scissors"].contains(&user_input.as_str()) {
            println!("Invalid choice! Please type 'rock', 'paper', or 'scissors'.");
            continue;
        }

        // Generate computer's choice
        let computer_choice = get_computer_choice();
        println!("Computer chose: {}", computer_choice);

        // Determine the winner
        match determine_winner(&user_input, &computer_choice) {
            Some("win") => println!("You win! 🎉"),
            Some("lose") => println!("You lose! 😢"),
            Some("draw") => println!("It's a draw! 🤝"),
            _ => println!("Something went wrong!"),
        }
    }
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let random_index = rand::thread_rng().gen_range(0..choices.len());
    choices[random_index].to_string()
}

fn determine_winner(user: &str, computer: &str) -> Option<&'static str> {
    match (user, computer) {
        ("rock", "scissors") | ("scissors", "paper") | ("paper", "rock") => Some("win"),
        ("rock", "paper") | ("scissors", "rock") | ("paper", "scissors") => Some("lose"),
        _ => Some("draw"),
    }
}
