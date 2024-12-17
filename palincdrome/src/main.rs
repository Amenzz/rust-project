use std::io::stdin;

fn get_input() -> String {
    let mut s = String::new();
    println!("Enter a string:");
    // Read user input
    if let Err(_) = stdin().read_line(&mut s) {
        println!("Failed to read input. Returning an empty string.");
        return String::new();
    }

    // Trim whitespace and return the result
    s.trim().to_string()
}

fn get_palindrome() {
    let input = get_input();
    let reversed: String = input.chars().rev().collect();
    if input == reversed {
        println!("{}: is palindrome input, which is : {}", input, reversed);
    } else {
        println!(
            "{}: is not a palindrome input, which is : {}", input, reversed);
    }
}
fn main() {
    let _input = get_palindrome();
}
