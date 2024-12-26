fn unit_converter() -> f64 {
    let mut input = String::new();
    println!(
        "Enter which unit you want to convert to: 
        \n a. meter to kilometer 
        \n b. kilometer to meter 
        \n c. gram to kilogram  
        \n d. kilogram to gram"
    );
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    let get_input = input.trim().chars().next(); // Get the first character
    let mut calculate_unit = String::new();
    println!("Enter a value to convert to the unit you chose above:");
    std::io::stdin()
        .read_line(&mut calculate_unit)
        .expect("Error reading input");
    let get_unit: f64 = match calculate_unit.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid number entered!");
            return 0.0;
        }
    };

    match get_input {
        Some('a') => get_unit / 1000.0, // Meter to kilometer
        Some('b') => get_unit * 1000.0, // Kilometer to meter
        Some('c') => get_unit / 1000.0, // Gram to kilogram
        Some('d') => get_unit * 1000.0, // Kilogram to gram
        _ => {
            println!("Invalid choice");
            0.0
        }
    }
}

fn main() {
    let converter = unit_converter();
    println!("unit converter: {}", converter);
}
