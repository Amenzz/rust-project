fn input_handler() -> String {
    let mut input = String::new();
    println!("Enter a number to check whether it's even or odd:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error handling input");

    match input.trim().parse::<i64>() {
        Ok(n) if n % 2 == 0 => "The number is even".to_string(),
        Ok(n) if n % 2 != 0 => "The number is odd".to_string(),
        Ok(_) => "Unhandled case".to_string(),
        Err(_) => "Invalid input".to_string(),
    }
}

fn main() {
    let result = input_handler();
    println!("{}", result);
}
