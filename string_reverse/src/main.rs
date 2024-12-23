fn get_input() -> String {
    let mut input = String::new();
    println!("Type a string");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    input.to_string()
}

fn reverse_string(reversed: String) -> String {
    let string_reversed: String = reversed.chars().rev().collect();
    string_reversed
}
fn main() {
    let hallowa = get_input();
    let reversed = reverse_string(hallowa);
    println!("the string you put when reversed is: {}", reversed);
}
