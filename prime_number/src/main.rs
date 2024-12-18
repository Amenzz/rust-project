use std::io;

fn get_prime_number() -> u32 {
    let mut input = String::new();
    println!("Enter a number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    input
        .trim()
        .parse::<u32>()
        .expect("Please enter a valid number")
}

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u32) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let test_prime = get_prime_number();
    if is_prime(test_prime) {
        println!("{}, is a prime number!", test_prime);
    } else {
        println!("{}, is not a prime number!", test_prime);
    }
}
