use std::collections::HashSet;
use std::io;

fn main() {
    println!("Welcome to Hangman!");

    // Word list for the game
    let words = ["rust", "hangman", "programming", "developer", "language"];
    let secret_word = words[rand::random::<usize>() % words.len()];
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    
    let mut guessed_letters = HashSet::new();
    let mut remaining_attempts = 6;

    loop {
        // Display the word with guessed letters revealed
        let mut display_word = String::new();
        let mut all_guessed = true;
        for &c in &secret_word_chars {
            if guessed_letters.contains(&c) {
                display_word.push(c);
            } else {
                display_word.push('_');
                all_guessed = false;
            }
            display_word.push(' ');
        }

        println!("\nWord: {}", display_word.trim());
        println!("Remaining attempts: {}", remaining_attempts);

        if all_guessed {
            println!("Congratulations! You guessed the word: {}", secret_word);
            break;
        }

        if remaining_attempts == 0 {
            println!("Game over! The word was: {}", secret_word);
            break;
        }

        // Get user input
        println!("Enter a letter to guess:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input_char = match input.trim().chars().next() {
            Some(c) => c.to_lowercase().next().unwrap(),
            None => {
                println!("Invalid input. Please enter a single letter.");
                continue;
            }
        };

        // Check if the input was already guessed
        if guessed_letters.contains(&input_char) {
            println!("You already guessed '{}'. Try a different letter.", input_char);
            continue;
        }

        // Check if the input is in the word
        guessed_letters.insert(input_char);
        if secret_word_chars.contains(&input_char) {
            println!("Good guess!");
        } else {
            println!("Wrong guess!");
            remaining_attempts -= 1;
        }
    }
}
