use std::io::{self, Write}; // For user input/output

fn main() {
    let mut shopping_list: Vec<String> = Vec::new(); // The shopping list is stored as a vector of strings.

    loop {
        // Display the menu options
        println!("\nShopping List Manager");
        println!("1. Add an item");
        println!("2. Remove an item");
        println!("3. View the list");
        println!("4. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); // Flush the output buffer to display the prompt immediately

        // Read the user's choice
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => add_item(&mut shopping_list),
            "2" => remove_item(&mut shopping_list),
            "3" => view_list(&shopping_list),
            "4" => {
                println!("Exiting the Shopping List Manager. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

// Function to add an item to the shopping list
fn add_item(shopping_list: &mut Vec<String>) {
    print!("Enter the item to add: ");
    io::stdout().flush().unwrap(); // Flush the output buffer to display the prompt immediately

    let mut item = String::new();
    io::stdin()
        .read_line(&mut item)
        .expect("Failed to read input");

    let item = item.trim().to_string(); // Remove trailing whitespace and convert to string
    if item.is_empty() {
        println!("Item cannot be empty!");
    } else {
        shopping_list.push(item);
        println!("Item added successfully.");
    }
}

// Function to remove an item from the shopping list
fn remove_item(shopping_list: &mut Vec<String>) {
    if shopping_list.is_empty() {
        println!("The shopping list is empty. Nothing to remove.");
        return;
    }

    view_list(shopping_list);
    print!("Enter the number of the item to remove: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<usize>() {
        Ok(index) if index > 0 && index <= shopping_list.len() => {
            let removed_item = shopping_list.remove(index - 1);
            println!("Removed: {}", removed_item);
        }
        _ => println!("Invalid index. Please try again."),
    }
}

// Function to view the current shopping list
fn view_list(shopping_list: &Vec<String>) {
    if shopping_list.is_empty() {
        println!("The shopping list is empty.");
    } else {
        println!("\nCurrent Shopping List:");
        for (i, item) in shopping_list.iter().enumerate() {
            println!("{}. {}", i + 1, item);
        }
    }
}
