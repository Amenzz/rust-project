use std::io;

fn get_input() -> u64 {
    let mut input = String::new();
    println!("Enter an integer to find it`s factorial");
    io::stdin().read_line(&mut input).expect("Error input");
    input.trim().parse::<u64>().expect("Enter a valid number")
}

fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }
    n * factorial(n - 1)
}

fn main() {
    let results = get_input();
    let get_factorial = factorial(results);
    if get_factorial == 0 {
        println!("{} error number", results);
    } else {
        println!("the factorial is {} ", get_factorial);
    }
}
