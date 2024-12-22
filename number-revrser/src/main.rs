use std::io;

fn get_number() -> i32 {
    let mut input = String::new();
    println!("Enter a  number more than one digit");
    io::stdin().read_line(&mut input).expect("Error reading");
    input
        .trim()
        .parse::<i32>()
        .expect("Please enter a valid number")
}

fn number_reverse(num: i32) -> i32 {
    let reversed_str: String = num
        .abs() // Handle negative numbers
        .to_string()
        .chars()
        .rev()
        .collect();
    let reversed_num: i32 = reversed_str.parse().unwrap_or(0);
    if num < 0 {
        -reversed_num
    } else {
        reversed_num
    }
}

fn main() {
    let number = get_number();
    let reversed = number_reverse(number);
    println!("You have entered: {}", number);
    println!("Reversed number: {}", reversed);
}
