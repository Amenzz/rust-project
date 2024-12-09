use std::io;
use std::collections::VecDeque;

fn main() {
    let mut to_do_list: VecDeque<String> = VecDeque::new();

    loop {
        println!("\nTo-Do List Application");
        println!("1. Add a task");
        println!("2. Remove a task");
        println!("3. View tasks");
        println!("4. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter the task to add:");
                let mut task = String::new();
                io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read input");
                let task = task.trim().to_string();

                if !task.is_empty() {
                    to_do_list.push_back(task);
                    println!("Task added successfully!");
                } else {
                    println!("Task cannot be empty.");
                }
            }
            "2" => {
                if to_do_list.is_empty() {
                    println!("No tasks to remove.");
                } else {
                    println!("Enter the task number to remove:");

                    for (index, task) in to_do_list.iter().enumerate() {
                        println!("{}. {}", index + 1, task);
                    }

                    let mut index_str = String::new();
                    io::stdin()
                        .read_line(&mut index_str)
                        .expect("Failed to read input");

                    if let Ok(index) = index_str.trim().parse::<usize>() {
                        if index > 0 && index <= to_do_list.len() {
                            let removed_task = to_do_list.remove(index - 1).unwrap();
                            println!("Removed task: {}", removed_task);
                        } else {
                            println!("Invalid task number.");
                        }
                    } else {
                        println!("Invalid input. Please enter a valid number.");
                    }
                }
            }
            "3" => {
                if to_do_list.is_empty() {
                    println!("No tasks in the list.");
                } else {
                    println!("\nYour tasks:");
                    for (index, task) in to_do_list.iter().enumerate() {
                        println!("{}. {}", index + 1, task);
                    }
                }
            }
            "4" => {
                println!("Exiting the application. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
