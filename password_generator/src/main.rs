use bcrypt::{hash, verify};

fn password_input() -> String {
    let mut input = String::new();
    println!("Enter your password:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    // Remove the trailing newline character added by read_line
    let input = input.trim();

    // Hash the password using bcrypt with a cost of 4
    let hashed_password = hash(input, 4).expect("Failed to hash password");

    hashed_password
}

fn main() {
    let hashed_password = password_input();
    println!("Generated hashed password: {}", hashed_password);

    // Optional: Verify the password
    let mut input = String::new();
    println!("Re-enter your password to verify:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    let input = input.trim();
    let is_valid = verify(input, &hashed_password).expect("Failed to verify password");

    if is_valid {
        println!("Password verified successfully!");
    } else {
        println!("Password verification failed!");
    }
}
