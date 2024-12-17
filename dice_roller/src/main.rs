use rand::Rng;
use std::io::{stdin, stdout, Write};

fn handle_input() -> u32 {
    let mut s = String::new();
    print!("Enter the number of sides on the die: ");
    stdout().flush().unwrap();

    stdin().read_line(&mut s).unwrap();

    match s.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Defaulting to 0.");
            0
        }
    }
}

fn roll_dice(user_input: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=user_input)
}

fn main() {
    let user_input = handle_input();
    println!("You entered: {}", user_input);

    let dice_roll = roll_dice(user_input);
    println!(
        "You rolled a {}-sided die and got: {}",
        user_input, dice_roll
    );
}
