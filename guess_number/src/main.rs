use rand::Rng;
use std::io;
fn main() {
    let mut input = String::new(); // Read input as a String
    println!("Guess a number between 1 and 3");
    io::stdin()
    .read_line( &mut input)
    .expect("Error reading input");

    let input: i32 = input.trim().parse().expect("error parsing input");
    let secret_number = get_random_number();
    if input == secret_number{
        println!("correct guess, the number is: {}" , secret_number )
    }
    println!("The secret number is: {}", secret_number);
}

fn get_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..5) 
}
