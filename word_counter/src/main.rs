fn main() {
    println!("Write something. . . ");
    let input = read_string();
    let name_length = input.trim().chars().count(); // Count characters after trimming whitespace
    println!("Your word's length is: {name_length}");
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
