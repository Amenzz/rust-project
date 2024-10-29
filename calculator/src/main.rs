fn add(a: f64, b: f64) -> f64 {
    a + b
}
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Error: division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
fn main() {
    let num1: f64 = 5.7;
    let num2: f64 = 8.7;
    println!("Addition: {}", add(num1, num2));
    println!("subtraction: {}", subtract(num1, num2));
    println!("multiply: {}", multiply(num1, num2));
    match divide(num1, num2) {
        Ok(result) => println!("Division: {}", result),
        Err(e) => println!("{}", e),
    }
}
