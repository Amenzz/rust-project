use std::io;
fn main() {
    let mut input = String::new(); // Read input as a String
    println!("Please enter a number in celcius:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert the input string to an f64
    let input: f64 = input.trim().parse().expect("Please enter a valid number");

    let fahranite = from_celcius_to_fahranite(input);
    let kelvin: f64 = from_celcius_to_kelvin(input);

    println!("{}°C is equal to {}°F", input, fahranite);
    println!("{}°C is equal to {}K", input, kelvin);

}


fn from_celcius_to_kelvin(input: f64) -> f64 {
    input + 273.15
}

fn from_celcius_to_fahranite(input: f64) -> f64 {
    input * 9.0 / 5.0 + 32.0
}
