use std::io;

fn get_input() -> u64 {
   loop {
    let mut input = String::new();
    println!("Enter a number");
    io::stdin()
        .read_line(&mut input)
        .expect("Error please enter a valid number");
    match input.trim().parse::<u64>() {
        Ok(n) => return n,  
        Err(_) => {
            println!("Invalid input. Please enter a valid positive number."); 
        }
    }
   }
}

fn get_fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 2..=n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

fn main() {
    let n = get_input(); // Correctly capture user input
    let final_result = get_fibonacci(n); // Use the user input to calculate Fibonacci
    println!("Fibonacci number: {}", final_result); // Print the result
}
